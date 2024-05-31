from abc import ABCMeta, abstractmethod
from decimal import localcontext
from decimal import Decimal as dec
from fractions import Fraction as frac
import math
from typing import Self

from .config import quanfig
from .exceptions import MismatchedUnitsError
from .format import group_digits
from . import units


class AbstractQuantity(metaclass=ABCMeta):
    """Parent class for all quantities of all flavours, both absolute and relative.

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

    __slots__ = ("_number", "_unit", "_uncertainty", "_value", "_pending_cancel")

    def __init__(
        self,
        number: str | int | float | dec | None = None,
        unit=None,
        uncertainty: str | int | float | dec | Self | None = None,
    ):

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
        elif isinstance(uncertainty, AbstractQuantity):
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
        
        if not hasattr(self, "_pending_cancel"):
            self._pending_cancel = False


    # These properties should be overridden by subclasses, but unfortunately it is no
    # longer possible to create abstract properties

    @property
    def number(self):
        return self._number

    @property
    def unit(self):
        return self._unit

    @property
    def uncertainty(self):
        return self._uncertainty

    @property
    def value(self):
        return self._value
    
    @abstractmethod
    def resolution(self):
        """Return the resolution of the quantity, ignoring the uncertainty.
        
        The resolution is the size of a unit of the smallest significant figure.

        Termed "resolution" to avoid confusion between this and the computer science
        concept of "precision", which typically refers to the number of digits.
        """
        raise NotImplementedError
    
    @abstractmethod
    def cancel(self):
        raise NotImplementedError

    @abstractmethod
    def fully_cancel(self):
        raise NotImplementedError

    @abstractmethod
    def canonical(self):
        raise NotImplementedError
    
    @abstractmethod
    def base(self):
        raise NotImplementedError

    @abstractmethod
    def to(self, other):
        raise NotImplementedError

    @abstractmethod
    def on_scale(self, other):
        raise NotImplementedError

    def __repr__(self):
        if not self._uncertainty:
            return f"{type(self).__name__}({group_digits(self.number)}, {self.unit})"
        else:
            return f"{type(self).__name__}({group_digits(self.number)}, {self.unit}, uncertainty={group_digits(self._uncertainty)})"

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

    def is_dimensionless(self):
        """Check if unit is dimensionless."""
        return self._unit.is_dimensionless()
