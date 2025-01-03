from decimal import Decimal as dec
from fractions import Fraction as frac
import math
from typing import Self

from .config import quanfig
from .dimensions import Dimensions
from .exceptions import MismatchedUnitsError, NotDimensionlessError
from .format import group_digits
from .abstract_quantity import AbstractQuantity
from .uncertainties import get_uncertainty

from . import units


class Quantity(AbstractQuantity):
    """A class that represents absolute physical quantities.
    """

    __slots__ = ()

    # kwargs is for the things shown via comments that should be hidden from public API
    def __init__(
        self,
        number: str | int | float | dec | None = None,
        unit=None,
        uncertainty: str | int | float | dec | Self | None = None,
        value: str | Self | None = None,
        _pending_cancel: bool = False,
        **kwargs,
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
        super().__init__(
            number,
            unit,
            uncertainty,
            _pending_cancel=_pending_cancel,
            **kwargs,
        )


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
    def uncertainty(self) -> Self:
        return Quantity(
            self._uncertainty,
            self._unit,
            _pending_cancel=self._pending_cancel,
        )
    
    @property
    def value(self) -> Self:
        """Return the value of the object as a `Quantity`.
        
        For a `Quantity`, just returns itself.
        Defined so that all quantities, units, constants etc. can be guaranteed to have a representation
        as a `Quantity` available.
        """
        return self

    def __int__(self) -> int:
        if not self.is_dimensionless():
            raise NotDimensionlessError(
                "Cannot cast a non-dimensionless quantity to an integer!"
            )
        else:
            dimensionless_quant = self.base()
            return int(dimensionless_quant.number)

    def __float__(self) -> float:
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

    def __add__(self, other, correlation=0) -> Self:
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
                _pending_cancel=self._pending_cancel,
            )
        else:
            return NotImplemented

    def __sub__(self, other, correlation=0) -> Self:
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
                _pending_cancel=self._pending_cancel,
            )
        else:
            return NotImplemented

    def __mul__(self, other, correlation=0) -> Self:
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
                _pending_cancel=self._pending_cancel,
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
                _pending_cancel=quanfig.AUTO_CANCEL,
            )
        else:
            return NotImplemented

    def __rmul__(self, other, correlation=0) -> Self:
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
                _pending_cancel=self._pending_cancel,
            )
        else:
            return NotImplemented

    def __truediv__(self, other, correlation=0) -> Self:
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
                _pending_cancel=self._pending_cancel,
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
                _pending_cancel=quanfig.AUTO_CANCEL,
            )
        else:
            return NotImplemented

    def __rtruediv__(self, other, correlation=0) -> Self:
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
                _pending_cancel=self._pending_cancel,
            )
        else:
            return NotImplemented

    # For now Unit only supports integer or fractional exponents
    def __pow__(self, other) -> Self:
        if isinstance(other, int):
            new_number = self.number**other
            new_uncertainty = get_uncertainty(new_number, "pow", self, numberx=other)
            return Quantity(
                new_number,
                self._unit**other,
                new_uncertainty,
                _pending_cancel=self._pending_cancel,
            )
        elif isinstance(other, frac):
            frac_as_dec = dec(other._numerator) / dec(other.denominator)
            new_number = self.number**frac_as_dec
            new_uncertainty = get_uncertainty(
                new_number, "pow", self, numberx=frac_as_dec
            )
            return Quantity(
                new_number,
                self._unit**other,
                new_uncertainty,
                _pending_cancel=self._pending_cancel,
            )
        else:
            return NotImplemented

    # Can only use a Quantity as an exponent if it is dimensionless
    def __rpow__(self, other, correlation=0) -> Self:
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
            return Quantity(
                new_number,
                new_unit,
                new_uncertainty,
                _pending_cancel=other._pending_cancel,
            )
        else:
            return NotImplemented

    def sqrt(self) -> Self:
        """Return the square root of the quantity, equivalent to `Quantity**Fraction(1, 2)`."""
        return self ** frac(1, 2)

    def exp(self) -> Self:
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

    def ln(self) -> Self:
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

    def log(self, base=None) -> Self:
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

    def log10(self) -> Self:
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

    def __hash__(self) -> int:
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

    def __eq__(self, other) -> bool:
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

    def __gt__(self, other) -> bool:
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

    def __ge__(self, other) -> bool:
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

    def __neg__(self) -> Self:
        return Quantity(
            -1 * self.number,
            self._unit,
            self._uncertainty,
            )

    def __pos__(self) -> Self:
        return self

    def resolution(self) -> Self:
        return Quantity(
            10 ** self.number.as_tuple().exponent,
            self._unit,
            _pending_cancel=self._pending_cancel,
        )

    def dimensions(self) -> Dimensions:
        """Return the dimensions of the unit."""
        return self._unit.dimension_string()

    def cancel(self) -> Self:
        """Combine any like terms in the unit."""
        return Quantity(
            self.number,
            self._unit._cancel_to_unit(),
            self._uncertainty,
        )
    
    def _inplace_cancel(self) -> None:
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

    def fully_cancel(self) -> Self:
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

    def canonical(self) -> Self:
        """Express the quantity with its units in a canonical order."""
        return Quantity(
            self.number,
            self.unit.canonical().unit,
            self._uncertainty,
        )
    
    def base(self) -> Self:
        # Need to be careful here as unit.base() might return a Quantity with a number
        # other than 1
        base = self._unit.base()
        return Quantity(
            self.number * base.number,
            base.unit,
            self._uncertainty * base.number,
        )

    def to(self, other) -> Self:
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
