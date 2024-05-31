from decimal import Decimal as dec
from decimal import localcontext
from fractions import Fraction as frac
import math
from typing import Self

from .config import quanfig
from .dimensions import Dimensions
from .exceptions import MismatchedUnitsError, NotDimensionlessError
from .format import group_digits
from .uncertainties import get_uncertainty

from . import units


class Quantity:
    """A class that represents physical quantities.

    The quantity is expressed as a numerical value `number` and a unit of measurement `unit`, with an
    optional associated `uncertainty`, which is also a numerical value.
    `number` is any type that can be converted to `Decimal`, including strings.
    `unit` is any `BaseUnit` or `DerivedUnit`, or any `CompoundUnit` formed by multiplication thereof,
    or any `UnitlessUnit` such as `quanstants.unit.unitless`.
    `uncertainty` is also any type that can be converted to `Decimal`, including strings, however if
    `uncertainty` is not specified or given as `0` or `None` (default) or anything else "falsy" then
    the quantity`s uncertainty is set to `0`.
    `uncertainty` may also be a `Quantity` itself, which is useful if it was calculated separately.

    Normally, `number` and `unit` are both required. To create a unitless quantity, the special unitless
    unit (found in the main units namespace i.e. under `quanstants.units.unitless`) should be provided.

    Alternatively, all three may be left as `None` and `value` maybe specified instead as either a
    string or another `Quantity`. Passing a string simply invokes `Quantity.parse(value)`.

    If only a single string is passed to `number`, and no unit is provided, the string will be parsed
    by `Quantity.parse()`, so if it contains both a number and a unit, an appropriate `Quantity` will
    be created.

    However, while both the above work, the preferred method for quantity creation from a single string
    is to call `Quantity.parse(string)`.
    """

    __slots__ = ("_number", "_unit", "_uncertainty", "_pending_cancel")

    # kwargs is for the things shown via comments that should be hidden from public API
    def __init__(
        self,
        number: str | int | float | dec | None = None,
        unit=None,
        uncertainty: str | int | float | dec | Self | None = None,
        value: str | Self | None = None,
        **kwargs,
    #   pending_cancel: bool,
    ):

        # Making a Quantity from a single string should be done with `Quantity.parse()`, but allow for the
        # fact that people might also just try to pass a string straight to `Quantity()`
        if (
            (isinstance(number, str))
            and (unit is None)
            and (uncertainty is None)
            and (value is None)
        ):
            parsed = self.parse(number)
            number, unit, uncertainty = parsed.number, parsed.unit, parsed.uncertainty

        # Accept a string or other Quantity
        if (number is None) and (unit is None) and (value is not None):
            if isinstance(value, str):
                parsed = self.parse(value)
                number, unit, uncertainty = (
                    parsed.number,
                    parsed.unit,
                    parsed.uncertainty,
                )
            else:
                number, unit, uncertainty = value.number, value.unit, value.uncertainty

        # Did extensive timings for this, shown in comments in ns in format:
        # dec; str, int, float => sum(str, int, float)
        # Prioritize fast dec as most Quantity creation is internal with a dec
        # Also choose to prioritize float over int/str as performance-sensitive things
        # are likely using floats
        # Considered using function but adds around 40 ns overhead
        # One-liner isn't faster than multi-line
        # Timings tested with dec("3.2"); "3.2", 3, 3.2
        # For reference, timings for conversion of each type directly and via str:
        # _number = dec(number)  # 86; 703, 116, 708 => 1527
        # _number = dec.from_float(number)  # N/A; N/A, 116, 719
        # _number = dec(str(number))  # 516; 331, 185, 328 => 844
        # 844 ns is presumably the lower limit for the total of the non-dec timings
        # I have absolutely no idea why the latter should be 2x faster for str and float
        # Naturally "3.2" vs 3 is unfair, but it is more relevant here
        # For reference, timings for dec(number) with 3 in each type:  # 86; 143, 116, 318 => 577
        # And for dec(str(number)):  # 217; 151, 191, 250 => 592

        # Current method: # 18; 405, 244, 420 => 1069
        if isinstance(number, dec):
            self._number = number    
        elif isinstance(number, float) and quanfig.CONVERT_FLOAT_AS_STR:
            self._number = dec(str(number))
        else:
            self._number = dec(number)

        # _number = number if isinstance(number, dec) else dec(str(number)) if (isinstance(number, float) and quanfig.CONVERT_FLOAT_AS_STR) else dec(number)  # 18; 430, 203, 422 => 1055
        # _number = number if isinstance(number, dec) else dec(number) if isinstance(number, (int, str)) else dec(str(number)) if quanfig.CONVERT_FLOAT_AS_STR else dec(str(number)) # 18; 518, 184, 498 => 1200
        # _number = number if isinstance(number, dec) else dec(number) if isinstance(number, (int, str)) or not quanfig.CONVERT_FLOAT_AS_STR else dec(str(number))  # 20; 483, 186, 493 => 1162
        # Old method, very slow for decimals:
        # _number = dec(str(number)) if quanfig.CONVERT_FLOAT_AS_STR else dec(number)  # 540; 368, 213, 367 => 948

        if isinstance(unit, str):
            self._unit = units.parse(unit)
        elif unit is None:
            self._unit = units.unitless
        else:
            self._unit = unit

        if uncertainty is None:
            self._uncertainty = dec("0")
        elif isinstance(uncertainty, Quantity):
            if uncertainty._unit != self._unit:
                self._uncertainty = uncertainty.to(self._unit).number
            else:
                self._uncertainty = uncertainty.number
        elif isinstance(uncertainty, dec):
            self._uncertainty = uncertainty    
        elif isinstance(uncertainty, float) and quanfig.CONVERT_FLOAT_AS_STR:
            self._uncertainty = dec(str(uncertainty))
        else:
            self._uncertainty = dec(uncertainty)
        
        # Variable that indicates unit needs cancelling but is initially uncancelled
        self._pending_cancel = kwargs.get("pending_cancel", False)


    # Generally accessing properties via self.x rather than self._x is safer
    # self._x is faster though, in my tests 9.7 ns vs 48.6 ns
    # However for most operations this is not a bottleneck
    @property
    def number(self):
        return self._number

    # Note: auto_cancel is done lazily after arithmetic so accessing _unit directly
    # should only be done with care as it may give an uncancelled unit
    # Doing that may be desirable for chained arithmetic operations so that the units
    # are only cancelled after all operations are done and the representation becomes
    # relevant
    # In most cases self.unit should be accessed so that cancellation can be done
    @property
    def unit(self):
        if self._pending_cancel is True:
            self._inplace_cancel()
            self._pending_cancel = False
        return self._unit

    # Note: the uncertainty is returned to the user as a Quantity, so internally
    # usually the decimal value should be accessed directly with _uncertainty
    @property
    def uncertainty(self):
        return Quantity(
            self._uncertainty,
            self._unit,
            pending_cancel=self._pending_cancel,
        )
    
    @property
    def value(self):
        """Return the value of the object as a `Quantity`.
        
        For a `Quantity`, just returns itself.
        Defined so that all quantities, units, constants etc. can be guaranteed to have a representation
        as a `Quantity` available.
        """
        return self

    def __repr__(self):
        if not self._uncertainty:
            return f"Quantity({group_digits(self.number)}, {self.unit})"
        else:
            return f"Quantity({group_digits(self.number)}, {self.unit}, uncertainty={group_digits(self._uncertainty)})"

    def __str__(self):
        if quanfig.ROUND_BEFORE_PRINT:
            self = self.round()
        if not self._uncertainty:
            return f"{group_digits(self.number)} {self.unit}"
        # Check that uncertainty is not more precise than the number via the exponent
        # More negative (smaller) exponent means more precise
        elif (
            self.number.as_tuple().exponent <= self._uncertainty.as_tuple().exponent
        ) and (quanfig.UNCERTAINTY_STYLE == "PARENTHESES"):
            number_string = group_digits(self.number)
            bracketed_uncertainty = (
                f"({''.join([str(n) for n in self._uncertainty.as_tuple().digits])})"
            )
            # Insert before exponential if present
            if any(x in number_string for x in ["E", "e"]):
                number_string = number_string.replace(
                    "E", f"{bracketed_uncertainty}E"
                ).replace("e", f"{bracketed_uncertainty}e")
            else:
                number_string = number_string + bracketed_uncertainty
            return f"{number_string} {self.unit}"
        else:
            return f"{group_digits(self.number)} ± {group_digits(self._uncertainty)} {self.unit}"

    def __int__(self):
        if not self.is_dimensionless():
            raise NotDimensionlessError(
                "Cannot cast a non-dimensionless quantity to an integer!"
            )
        else:
            dimensionless_quant = self.base()
            return int(dimensionless_quant.number)

    def __float__(self):
        if not self.is_dimensionless():
            raise NotDimensionlessError(
                "Cannot cast a non-dimensionless quantity to a float!"
            )
        else:
            dimensionless_quant = self.base()
            return float(dimensionless_quant.number)

    # *** Arithmetic functions, both dunder methods and not ***
    # In general, maintain laziness of auto cancel i.e. retain the uncancelled forms
    # but perpetuate the state of pending cancellation so that it is eventually assessed

    def __add__(self, other, correlation=0):
        if isinstance(other, Quantity):
            if self._unit == other._unit:
                new_number = self.number + other.number
                new_uncertainty = get_uncertainty(
                    new_number, "add", self, quantityB=other, correlation=correlation
                )
            # Allow mixed units with the same dimension
            elif self._unit.dimensions == other._unit.dimensions:
                converted = other.to(self._unit)
                new_number = self.number + converted.number
                new_uncertainty = get_uncertainty(
                    new_number,
                    "add",
                    self,
                    quantityB=converted,
                    correlation=correlation,
                )
            else:
                raise MismatchedUnitsError(
                    f"Can't add quantity in {other.unit} to quantity in {self.unit}."
                )
            return Quantity(
                new_number,
                self._unit,
                new_uncertainty,
                pending_cancel=self._pending_cancel,
            )
        else:
            return NotImplemented

    def __sub__(self, other, correlation=0):
        if isinstance(other, Quantity):
            if self._unit == other._unit:
                new_number = self.number - other.number
                new_uncertainty = get_uncertainty(
                    new_number, "sub", self, quantityB=other, correlation=correlation
                )
            # Allow mixed units with the same dimension
            elif self._unit.dimensions == other._unit.dimensions:
                converted = other.to(self._unit)
                new_number = self.number - converted.number
                new_uncertainty = get_uncertainty(
                    new_number,
                    "sub",
                    self,
                    quantityB=converted,
                    correlation=correlation,
                )
            else:
                raise MismatchedUnitsError(
                    f"Can't subtract quantity in {other.unit} from quantity in {self.unit}."
                )
            return Quantity(
                new_number,
                self._unit,
                new_uncertainty,
                pending_cancel=self._pending_cancel,
            )
        else:
            return NotImplemented

    def __mul__(self, other, correlation=0):
        if isinstance(other, (str, int, float, dec)):
            if quanfig.CONVERT_FLOAT_AS_STR:
                other = dec(str(other))
            else:
                other = dec(other)
            new_number = self.number * other
            new_uncertainty = get_uncertainty(
                new_number, "mul", self, numberx=other, correlation=correlation
            )
            return Quantity(
                new_number,
                self._unit,
                new_uncertainty,
                pending_cancel=self._pending_cancel,
            )
        elif isinstance(other, Quantity):
            new_number = self.number * other.number
            new_uncertainty = get_uncertainty(
                new_number, "mul", self, quantityB=other, correlation=correlation
            )
            # Use uncancelled form of units for lazy cancellation
            # Mark for later automatic cancellation if desired
            return Quantity(
                new_number,
                self._unit * other._unit,
                new_uncertainty,
                pending_cancel=quanfig.AUTO_CANCEL,
            )
        else:
            return NotImplemented

    def __rmul__(self, other, correlation=0):
        if isinstance(other, (str, int, float, dec)):
            if quanfig.CONVERT_FLOAT_AS_STR:
                other = dec(str(other))
            else:
                other = dec(other)
            new_number = other * self.number
            new_uncertainty = get_uncertainty(
                new_number, "mul", self, numberx=other, correlation=correlation
            )
            return Quantity(
                new_number,
                self._unit,
                new_uncertainty,
                pending_cancel=self._pending_cancel,
            )
        else:
            return NotImplemented

    def __truediv__(self, other, correlation=0):
        if isinstance(other, (str, int, float, dec)):
            if quanfig.CONVERT_FLOAT_AS_STR:
                other = dec(str(other))
            else:
                other = dec(other)
            new_number = self.number / other
            new_uncertainty = get_uncertainty(
                new_number, "truediv", self, numberx=other, correlation=correlation
            )
            return Quantity(
                new_number,
                self._unit,
                new_uncertainty,
                pending_cancel=self._pending_cancel,
            )
        elif isinstance(other, Quantity):
            new_number = self.number / other.number
            new_uncertainty = get_uncertainty(
                new_number, "truediv", self, quantityB=other, correlation=correlation
            )
            # Automatically cancel if desired
            return Quantity(
                new_number,
                self._unit / other._unit,
                new_uncertainty,
                pending_cancel=quanfig.AUTO_CANCEL,
            )
        else:
            return NotImplemented

    def __rtruediv__(self, other, correlation=0):
        if isinstance(other, (str, int, float, dec)):
            if quanfig.CONVERT_FLOAT_AS_STR:
                other = dec(str(other))
            else:
                other = dec(other)
            new_number = other / self.number
            new_uncertainty = get_uncertainty(
                new_number, "rtruediv", self, numberx=other, correlation=correlation
            )
            return Quantity(
                new_number,
                self._unit.inverse(),
                new_uncertainty,
                pending_cancel=self._pending_cancel,
            )
        else:
            return NotImplemented

    # For now Unit only supports integer or fractional exponents
    def __pow__(self, other):
        if isinstance(other, int):
            new_number = self.number**other
            new_uncertainty = get_uncertainty(new_number, "pow", self, numberx=other)
            return Quantity(
                new_number,
                self._unit**other,
                new_uncertainty,
                pending_cancel=self._pending_cancel,
            )
        elif isinstance(other, frac):
            frac_as_dec = dec(str(float(other)))
            new_number = self.number**frac_as_dec
            new_uncertainty = get_uncertainty(
                new_number, "pow", self, numberx=frac_as_dec
            )
            return Quantity(
                new_number,
                self._unit**other,
                new_uncertainty,
                pending_cancel=self._pending_cancel,
            )
        else:
            return NotImplemented

    # Can only use a Quantity as an exponent if it is dimensionless
    def __rpow__(self, other, correlation=0):
        if not self.is_dimensionless():
            raise NotDimensionlessError(
                "Cannot raise to the power of a non-dimensionless quantity!"
            )
        else:
            dimensionless_quant = self.base()
        if isinstance(other, (str, int, float, dec)):
            if quanfig.CONVERT_FLOAT_AS_STR:
                other = dec(str(other))
            else:
                other = dec(other)
            new_number = other ** dimensionless_quant.number
            new_uncertainty = get_uncertainty(
                new_number, "rpow", self, numberx=other, correlation=correlation
            )
            return Quantity(new_number, None, new_uncertainty)
        elif isinstance(other, Quantity):
            # Unit only supports integer or fractional exponents
            ratio = dimensionless_quant.number.as_integer_ratio()
            if ratio.denominator == 1:
                exponent = ratio.numerator
            else:
                exponent = frac(*ratio)
            new_number = other.number ** exponent
            new_unit = other._unit ** exponent
            new_uncertainty = get_uncertainty(
                new_number, "rpow", self, quantityB=other, correlation=correlation 
            )
        else:
            return NotImplemented

    def sqrt(self):
        """Return the square root of the quantity, equivalent to `Quantity**Fraction(1, 2)`."""
        return self ** frac(1, 2)

    def exp(self):
        """Return the value of e raised to the power of the quantity, for dimensionless quantities only."""
        if not self.is_dimensionless():
            raise NotDimensionlessError(
                "Cannot raise to the power of a non-dimensionless quantity!"
            )
        else:
            dimensionless_quant = self.base()
            new_number = dimensionless_quant.number.exp()
            new_uncertainty = get_uncertainty(new_number, "exp", self)
            return Quantity(new_number, None, new_uncertainty)

    def ln(self):
        """Return the natural logarithm of the quantity, for dimensionless quantities only."""
        if not self.is_dimensionless():
            raise NotDimensionlessError(
                "Cannot take the logarithm of a non-dimensionless quantity!"
            )
        else:
            dimensionless_quant = self.base()
            new_number = dimensionless_quant.number.ln()
            new_uncertainty = get_uncertainty(new_number, "ln", self)
            return Quantity(new_number, None, new_uncertainty)

    def log(self, base=None):
        if base is None:
            return self.ln()
        elif base == 10:
            return self.log10()
        elif not self.is_dimensionless():
            raise NotDimensionlessError(
                "Cannot take the logarithm of a non-dimensionless quantity!"
            )
        else:
            dimensionless_quant = self.base()
            new_number = dec(math.log(dimensionless_quant.number, base))
            new_uncertainty = get_uncertainty(new_number, "log", self, log_base=base)
            return Quantity(new_number, None, new_uncertainty)

    def log10(self):
        """Return the base-10 logarithm of the quantity, for dimensionless quantities only."""
        if not self.is_dimensionless():
            raise NotDimensionlessError(
                "Cannot take the logarithm of a non-dimensionless quantity!"
            )
        else:
            dimensionless_quant = self.base()
            new_number = dimensionless_quant.number.log10()
            new_uncertainty = get_uncertainty(new_number, "log10", self)
            return Quantity(new_number, None, new_uncertainty)

    def __hash__(self):
        base = self.base()
        if base.number == 0:
            return 0
        # Check if unitless
        elif base.unit == 1:
            return hash(base.number)
        else:
            base_ids = ((id(unit), exponent) for unit, exponent in base.unit.components)
            return hash(
                (base.number, *base_ids)
            )

    def __eq__(self, other):
        if self.number == 0:
            return 0 == other
        # Check if unitless
        elif self._unit == 1:
            return self.number == other
        elif isinstance(other, Quantity):
            # Convert both to canonical base unit representations
            a = self.base()
            b = other.base()
            if ((a.number == b.number) and (a.unit == b.unit)):
                return True
            else:
                return False
        else:
            return NotImplemented

    def __gt__(self, other):
        # TODO match hash and eq
        if isinstance(other, Quantity):
            # Convert both to canonical base unit representations
            a = self.base()
            b = other.base()
            if a.unit.dimensions != b.unit.dimensions:
                raise MismatchedUnitsError
            elif (
                (a.number > b.number)
                and (a.unit.symbol == b.unit.symbol)
                and (a.unit.dimensions == b.unit.dimensions)
            ):
                return True
            else:
                return False
        else:
            return NotImplemented

    def __ge__(self, other):
        # TODO match hash and eq
        if isinstance(other, Quantity):
            # Convert both to canonical base unit representations
            a = self.base()
            b = other.base()
            if a.unit.dimensions != b.unit.dimensions:
                raise MismatchedUnitsError
            elif (
                (a.number >= b.number)
                and (a.unit.symbol == b.unit.symbol)
                and (a.unit.dimensions == b.unit.dimensions)
            ):
                return True
            else:
                return False
        else:
            return NotImplemented

    def __neg__(self):
        return Quantity(
            -1 * self.number,
            self._unit,
            self._uncertainty,
            )

    def __pos__(self):
        return self

    def __round__(self, ndigits=None, mode=None, pad=quanfig.ROUND_PAD):
        """Alias for `Quantity.round()` to allow the use of the built-in `round()`."""
        return self.round(ndigits, mode, pad)

    def round(
        self,
        ndigits=None,
        mode=None,
        pad=quanfig.ROUND_PAD,
        mode_if_uncertainty=None,
        mode_if_exact=None,
    ):
        """Return the quantity with the numerical part rounded by the set method.

        Calls one of `Quantity`'s other rounding methods depending on the value of `mode`:
        `"PLACES"` ⇒ `Quantity.round_to_places()`
        `"FIGURES"` ⇒ `Quantity.round_to_figures()`
        `"UNCERTAINTY"` ⇒ `Quantity.round_to_uncertainty()`

        By specifying `mode_if_uncertainty` and `mode_if_exact` separately, quantities can be rounded
        differently depending on whether they have an uncertainty or not. Specifying these values overrides
        the value of `mode` for those quantities.

        If no rounding modes are provided explicitly, the default rounding modes are specified by
        `quanstants.quanfig.ROUND_TO_IF_UNCERTAINTY` and `quanstants.quanfig.ROUND_TO_IF_EXACT`.

        Similarly, if no value is provided for `ndigits`, each rounding mode will use the default value
        specified by `quanstants.quanfig.NDIGITS_<rounding mode>`.

        The current defaults are such that exact quantities are rounded to 3 s.f. and quantities with
        uncertainties have their uncertainty rounded to 1 s.f. and the quantity is then rounded to the same
        precision.
        """
        if self._uncertainty:
            selected_mode = (
                mode_if_uncertainty
                if mode_if_uncertainty
                else mode
                if mode
                else quanfig.ROUND_TO_IF_UNCERTAINTY
            )
        else:
            selected_mode = (
                mode_if_exact
                if mode_if_exact
                else mode
                if mode
                else quanfig.ROUND_TO_IF_EXACT
            )

        if selected_mode == "PLACES":
            return self.round_to_places(ndigits, pad)
        elif selected_mode == "FIGURES":
            return self.round_to_figures(ndigits, pad)
        elif selected_mode == "UNCERTAINTY":
            return self.round_to_uncertainty(ndigits, pad)

    def round_to_places(self, ndigits=None, pad=quanfig.ROUND_PAD):
        """Return the quantity with the numerical part rounded to the specified number of decimal places.

        If `ndigits` is not provided, the default set by `quanstants.quanfig.NDIGITS_PLACES` will be used.

        The method used for rounding is that specified by the `quanstants.quanfig.ROUNDING_MODE` variable,
        which takes any of the `decimal` module's rounding modes. The default is `"ROUND_HALF_UP"`, i.e.
        to nearest with ties going away from zero.

        Like `siunitx`, and like `Decimal`, by default extra zeroes will be added to a short number to reach
        the desired number of decimal places. This behaviour can be forced on or off by passing `pad=True`
        or `pad=False`, or set globally by changing `quanstants.quanfig.ROUND_PAD`.

        Note that the rounding is done within a `decimal.localcontext()`, which means that the mode
        specified by `quanstants.quanfig.ROUNDING_MODE` does not override the current `decimal.Context()`
        and other `Decimal` instances will continue to round based on `decimal.getcontext().rounding`,
        which by default uses `"ROUND_HALF_EVEN"`.
        """
        if ndigits is None:
            ndigits = quanfig.NDIGITS_PLACES
        current_places = self.number.as_tuple().exponent * -1
        # Don't round if padding is turned off and the number doesn't have enough places
        if (current_places < ndigits) and (not pad):
            return self
        # Set decimal rounding to the specified method, which by default is the traditionally
        # expected behaviour
        # Use in a local context so that user's context isn't overwritten
        with localcontext() as ctx:
            ctx.rounding = quanfig.ROUNDING_MODE
            rounded = type(self)(
                round(self.number, ndigits),
                self._unit,
                self._uncertainty,
                pending_cancel=self._pending_cancel,
            )
        return rounded

    def round_to_figures(self, ndigits=None, pad=quanfig.ROUND_PAD):
        """Return the quantity with the numerical part rounded to the specified number of significant figures.

        If `ndigits` is not provided, the default set by `quanstants.quanfig.NDIGITS_FIGURES` will be used.

        The method used for rounding is that specified by `quanstants.quanfig.ROUNDING_MODE`, which takes
        any of the `decimal` module's rounding modes. The default is `"ROUND_HALF_UP"`, i.e. to
        nearest with ties going away from zero.

        Like `siunitx`, and like `Decimal`, by default extra zeroes will be added to a short number to reach
        the desired number of significant figures. This behaviour can be forced on or off by passing
        `pad=True` or `pad=False`, or set globally by changing `quanstants.quanfig.ROUND_PAD`.

        Note that the rounding is done within a `decimal.localcontext()`, which means that the mode
        specified by `quanstants.quanfig.ROUNDING_MODE` does not override the current `decimal.Context()`
        and other `Decimal` instances will continue to round based on `decimal.getcontext().rounding`,
        which by default uses `"ROUND_HALF_EVEN"`.
        """
        if ndigits is None:
            ndigits = quanfig.NDIGITS_FIGURES
        # Sanity check for requested number of sigfigs
        if ndigits < 1:
            return self
        digits = self.number.as_tuple().digits
        current_sigfigs = len(digits)
        # First deal with request for fewer sigfigs than currently (usual case)
        if ndigits <= current_sigfigs:
            exponent = math.floor(self.number.log10())
            significand = self.number / dec(f"1E{exponent}")
            # Set decimal rounding to the specified method, which by default is the traditionally
            # expected behaviour
            # Use in a local context so that user's context isn't overwritten
            with localcontext() as ctx:
                ctx.rounding = quanfig.ROUNDING_MODE
                rounded_significand = round(significand, ndigits - 1)
            return type(self)(
                rounded_significand * dec(f"1E{exponent}"),
                self._unit,
                self._uncertainty,
                pending_cancel=self._pending_cancel,
            )
        # If request is for more sigfigs than currently, only pad if asked/permitted to do so
        elif (ndigits > current_sigfigs) and (not pad):
            return self
        elif (ndigits > current_sigfigs) and pad:
            # Add significant zeroes
            n_digits_to_add = ndigits - current_sigfigs
            new_digits = list(digits)
            for i in n_digits_to_add:
                new_digits.append(0)
            new_exponent = self.number.as_tuple().exponent - n_digits_to_add
            return type(self)(
                dec((self.number.as_tuple().sign, new_digits, new_exponent)),
                self._unit,
                self._uncertainty,
                pending_cancel=self._pending_cancel,
            )

    def round_to_sigfigs(self, ndigits=None, pad=quanfig.ROUND_PAD):
        """Alias for `round_to_figures()`."""
        return self.round_to_figures(ndigits, pad)

    def round_to_uncertainty(self, ndigits=None, pad=quanfig.ROUND_PAD):
        """Round the uncertainty to the specified number of significant figures, then return the quantity with the numerical part rounded to the same precision.

        If `ndigits` is not provided, the default set by `quanstants.quanfig.NDIGITS_UNCERTAINTY` will be
        used.

        The method used for rounding is that specified by `quanstants.quanfig.ROUNDING_MODE`, which takes
        any of the `decimal` module's rounding modes. The default is `"ROUND_HALF_UP"`, i.e. to
        nearest with ties going away from zero.

        Like `siunitx`, and like `Decimal`, by default extra zeroes will be added to a short number to reach
        the same precision as the uncertainty. This behaviour can be forced on or off by passing `pad=True`
        or `pad=False`, or set globally by changing `quanstants.quanfig.ROUND_PAD`.
        However, no padding is applied to the uncertainty itself, so the rounded quantity will never have a
        precision greater than the original uncertainty.

        Note that the rounding is done within a `decimal.localcontext()`, which means that the mode
        specified by `quanstants.quanfig.ROUNDING_MODE` does not override the current `decimal.Context()`
        and other `Decimal` instances will continue to round based on `decimal.getcontext().rounding`,
        which by default uses `"ROUND_HALF_EVEN"`.
        """
        if ndigits is None:
            ndigits = quanfig.NDIGITS_UNCERTAINTY
        # Check that the quantity even has an uncertainty
        if not self._uncertainty:
            return self
        # Sanity check for requested number of sigfigs
        if ndigits < 1:
            return self
        # First round the uncertainty
        # Get the uncertainty as a Quantity, because then we can just use its own rounding method
        # Never pad the uncertainty as that would be increasing its precision
        rounded_uncertainty = self.uncertainty.round_to_figures(ndigits, pad=False)
        # Now round the number to the same precision
        return self.round_to_resolution_of(rounded_uncertainty, pad=pad).with_uncertainty(
            rounded_uncertainty
        )

    def round_to_resolution_of(self, other, pad=quanfig.ROUND_PAD):
        if self._unit != other._unit:
            raise MismatchedUnitsError
        places = other.number.as_tuple().exponent * -1
        return self.round_to_places(places, pad=pad)

    def round_uncertainty(self, ndigits=None, mode=None):
        """Round the uncertainty without changing the number.

        Calls `.round()` on the uncertainty with the passed options, so will otherwise default to the
        behaviour of `round()` i.e. the rounding mode will be `quanstants.quanfig.ROUND_TO_IF_EXACT` and
        the number of digits will be `quanstants.quanfig.NDIGITS_<rounding mode>`.

        Unlike when rounding the number of a quantity, `quanstants.quanfig.ROUND_PAD` has no effect.
        The uncertainty is never padded when rounding as that would imply an increase in the precision of
        the uncertainty itself.
        """
        return self.with_uncertainty(self.uncertainty.round(ndigits, mode, pad=False))

    def resolution(self):
        """Return the resolution of the quantity, ignoring the uncertainty.
        
        The resolution is the size of a unit of the smallest significant figure.

        Termed "resolution" to avoid confusion between this and the computer science
        concept of "precision", which typically refers to the number of digits.
        """
        return Quantity(
            10 ** self.number.as_tuple().exponent,
            self._unit,
            pending_cancel=self._pending_cancel,
        )

    def is_dimensionless(self):
        """Check if unit is dimensionless."""
        return self._unit.is_dimensionless()

    def dimensions(self) -> Dimensions:
        """Return the dimensions of the unit."""
        return self._unit.dimension_string()

    def with_uncertainty(self, uncertainty):
        """Return a new quantity with the provided uncertainty."""
        return type(self)(
            self.number,
            self._unit,
            uncertainty,
            pending_cancel=self._pending_cancel,
        )

    def plus_minus(self, uncertainty):
        """Alias for `with_uncertainty()`."""
        return self.with_uncertainty(uncertainty)

    def cancel(self):
        """Combine any like terms in the unit."""
        return Quantity(
            self.number,
            self._unit._cancel_to_unit(),
            self._uncertainty,
        )
    
    def _inplace_cancel(self):
        """Cancel without returning new quantity.
        
        We can do cancellation in place as the unit will still have the same value and
        hash, so we aren't truly compromising our immutability.

        The principal idea is that a Quantity with an uncancelled unit and a
        `_pending_cancel = True` flag is identical to one with the same unit cancelled.

        Generally methods in the API return a new object, so keep normal cancel() the
        way it is and make this one private to avoid confusion.
        """
        self._unit = self._unit._cancel_to_unit()
        return None

    def fully_cancel(self):
        """Combine any terms of the same dimension in the unit."""
        # The unit may not have the same value afterwards
        # e.g. CompoundUnit(m km) cancels to Quantity(1000 m^2)
        # so we can't do this in place
        # We need to multiply both the number and uncertainty by the number that results
        # from the unit cancellation
        cancelled = self._unit.fully_cancel()
        return Quantity(
            self.number * cancelled.number,
            cancelled.unit,
            self._uncertainty * cancelled.number,
        )

    def _auto_cancel(self):
        """Apply automatic cancelling if specified by `quanstants.quanfig.AUTO_CANCEL`."""
        # DEPRECATED
        if quanfig.AUTO_CANCEL:
            return self.cancel()
        else:
            return self

    def canonical(self):
        """Express the quantity with its units in a canonical order."""
        return Quantity(
            self.number,
            self.unit.canonical().unit,
            self._uncertainty,
        )

    def base(self):
        """Return the quantity expressed in terms of base units.
        
        The unit is always returned in a fully cancelled, canonical form.
        """
        # Need to be careful here as unit.base() might return a Quantity with a number
        # other than 1
        base = self._unit.base()
        return Quantity(
            self.number * base.number,
            base.unit,
            self._uncertainty * base.number,
        )

    def to(self, other):
        """Express the quantity in terms of another unit."""
        if isinstance(other, str):
            # Allow parsing of unit string first
            other = units.parse(other)
        if self.number == 0:
            if self._uncertainty == 0:
                return Quantity(0, other)
            else:
                return Quantity(0, other, self.uncertainty.to(other))
        else:
            # Convert both args to quantities in base units, then divide, then cancel to
            # get ratio
            result = (self.base() / other.base()).cancel()
            # Before the implementation of lazy cancelling the above line took 96 µs in
            # one test on a slow PC, vs 186 for (self/other).base()
            # and 427 for (a/b).fully_cancel()
            if not self._uncertainty:
                return Quantity(result.number, result.unit.__mul__(other, concatenate_symbols=True))
            else:
                return Quantity(
                    result.number,
                    result.unit.__mul__(other, concatenate_symbols=True),
                    self.uncertainty.to(other),
                )

    def on_scale(self, other):
        """Convert an absolute quantity to a point on a relative scale.

        For example, express an absolute temperature in kelvin as a relative temperature on a scale with a
        different zero point.
        Defers to the argument's implementation of `.from_absolute()`.
        """
        return other.from_absolute(self)

    @classmethod
    def parse(cls, string: str):
        """Take a string of a number and unit, and optionally an uncertainty, and convert to an appropriate `Quantity` object.

        The string should take the form "<number> <unit>", with number and unit separated by whitespace.
        After separation, the number string is turned directly into a `Decimal`, so it can be any string that
        `Decimal()` accepts.
        The unit string is parsed by `quanstants.units.parse()`, so it must follow the same rules as for that.
        """
        try:
            # Look for uncertainty shown with plus_minus symbol
            if any(plus_minus in string for plus_minus in ["±", "+/-"]):
                split_by_plus_minus = string.replace("+/-", "±").split("±")
                number = split_by_plus_minus[0].rstrip()
                uncertainty_and_unit = split_by_plus_minus[1].lstrip().split(maxsplit=1)
                uncertainty = uncertainty_and_unit[0]
                unit = uncertainty_and_unit[1]
            else:
                split_by_space = string.split(maxsplit=1)
                # Also look for uncertainty shown in parentheses
                if "(" in split_by_space[0]:
                    number_and_uncertainty = (
                        split_by_space[0].replace(")", "(").split("(")
                    )
                    # Watch out for scientific notation of the form "1.234(56)e11"
                    # The above will always produce a three-membered list, where the
                    # third item is either "" or the exponent part
                    number = dec(number_and_uncertainty[0] + number_and_uncertainty[2])
                    # Have to make decimal places of uncertainty match those of number
                    uncertainty = dec(
                        (
                            0,
                            tuple(int(char) for char in number_and_uncertainty[1]),
                            number.as_tuple().exponent,
                        )
                    )
                else:
                    number = split_by_space[0]
                    uncertainty = None
                unit = split_by_space[1]
        except IndexError:
            raise ValueError(
                "String must contain both a number and a unit, separated by whitespace."
            )
        return cls(number, unit, uncertainty)
