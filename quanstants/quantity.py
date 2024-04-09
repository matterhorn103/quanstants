from decimal import Decimal as dec
from decimal import localcontext
import math

from .config import *

class MismatchedUnitsError(Exception):
    pass

class NotUnitlessError(Exception):
    pass

class Quantity:
    def __init__(
        self,
        number: str | int | float | dec,
        unit,
        uncertainty: str | int | float | dec | None = None,
    ):
        # Use Decimal type internally, exclusively
        # By default convert to string then to dec so that the resulting dec is the same as what
        # the user *thinks* the float is, not of the actual binary float value e.g. str(5.2) gives 
        # "5.2" but dec(5.2) gives Decimal('5.20000000000000017763568394002504646778106689453125')
        # and we want to give the user what they think they have -- Decimal('5.2')
        if CONVERT_FLOAT_AS_STR:
            self.number = dec(str(number))
        else:
            self.number = dec(number)
        self.unit = unit
        if (uncertainty is None) or (uncertainty == "(exact)"):
            self.uncertainty = "(exact)"
        elif CONVERT_FLOAT_AS_STR:
            self.uncertainty = dec(str(uncertainty))
        else:
            self.uncertainty = dec(uncertainty)
    
    def __repr__(self):
        if self.uncertainty != "(exact)":
            return f"Quantity({self.number}, {self.unit.symbol}, uncertainty={self.uncertainty})"
        else:
            return f"Quantity({self.number}, {self.unit.symbol})"

    def __str__(self):
        if self.uncertainty != "(exact)":
            return f"{self.number}({''.join([str(n) for n in self.uncertainty.as_tuple().digits])}) {self.unit.symbol}"
        else:
            return f"{self.number} {self.unit.symbol}"
    
    def __float__(self):
        return float(self.number)
    
    def __add__(self, other):
        if isinstance(other, Quantity):
            if self.unit == other.unit:
                return Quantity(self.number + other.number, self.unit)
            # Allow mixed units with the same dimension
            elif self.unit.dimensions == other.unit.dimensions:
                converted = other.to(self.unit)
                return Quantity(self.number + converted.number, self.unit)
            else:
                raise MismatchedUnitsError(f"Can't add quantity in {other.unit} to quantity in {self.unit}.")
        else:
            return NotImplemented
    
    def __sub__(self, other):
        if isinstance(other, Quantity):
            if self.unit == other.unit:
                return Quantity(self.number - other.number, self.unit)
            # Allow mixed units with the same dimension
            elif self.unit.dimensions == other.unit.dimensions:
                converted = other.to(self.unit)
                return Quantity(self.number - converted.number, self.unit)
            else:
                raise MismatchedUnitsError(f"Can't subtract quantity in {other.unit} from quantity in {self.unit}.")
        else:
            return NotImplemented

    def __mul__(self, other):
        if isinstance(other, (int, float, dec)):
            return Quantity(self.number * dec(str(other)), self.unit)
        elif isinstance(other, Quantity):
            return Quantity(self.number * other.number, self.unit * other.unit)
        # Check if it's a unit with duck typing
        elif hasattr(other, "base") and hasattr(other, "components"):
            # Note that Q * U only needs to be defined to allow syntax like 3 * units.m * units.s**-2
            return Quantity(self.number, self.unit * other)
        else:
            return NotImplemented
    
    def __rmul__(self, other):
        if isinstance(other, (int, float, dec)):
            return Quantity(dec(str(other)) * self.number, self.unit)
        else:
            return NotImplemented
    
    def __truediv__(self, other):
        if isinstance(other, (int, float, dec)):
            return Quantity(self.number / dec(str(other)), self.unit)
        elif isinstance(other, Quantity):
            return Quantity(self.number / other.number, self.unit / other.unit)
        # Check if it's a unit with duck typing
        elif hasattr(other, "base") and hasattr(other, "components"):
            return Quantity(self.number, self.unit / other)
        else:
            return NotImplemented
    
    def __rtruediv__(self, other):
        if isinstance(other, (int, float, dec)):
            return Quantity(dec(str(other)) / self.number, self.unit)
        else:
            return NotImplemented
    
    # For now Unit only supports integer exponents
    def __pow__(self, other):
        if isinstance(other, int):
            return Quantity(self.number ** other, self.unit ** other)
        else:
            return NotImplemented
    
    def __eq__(self, other):
        if isinstance(other, Quantity):
            # Convert both to canonical representations
            a = self.canonical()
            b = other.canonical()
            # Have to use dimensions as a sanity check in case different units have the same symbol
            if (a.number == b.number) and (a.unit.symbol == b.unit.symbol) and (a.unit.dimensions == b.unit.dimensions):
                return True
            else:
                return False
        else:
            return NotImplemented
    
    def __gt__(self, other):
        if isinstance(other, Quantity):
            # Convert both to canonical representations
            a = self.canonical()
            b = other.canonical()
            if (a.unit.dimensions != b.unit.dimensions):
                raise MismatchedUnitsError
            if (a.number > b.number) and (a.unit.symbol == b.unit.symbol) and (a.unit.dimensions == b.unit.dimensions):
                return True
            else:
                return False
        else:
            return NotImplemented

    def __ge__(self, other):
        if isinstance(other, Quantity):
            # Convert both to canonical representations
            a = self.canonical()
            b = other.canonical()
            if (a.unit.dimensions != b.unit.dimensions):
                raise MismatchedUnitsError
            if (a.number >= b.number) and (a.unit.symbol == b.unit.symbol) and (a.unit.dimensions == b.unit.dimensions):
                return True
            else:
                return False
        else:
            return NotImplemented

    def __neg__(self):
        return Quantity(-1 * self.number, self.unit)
    
    def __pos__(self):
        return self
    
    def __round__(self, ndigits=0):
        return self.round(ndigits)
    
    def round(self, ndigits=0):
        # Set decimal rounding to the traditionally expected method
        # Use in a local context so that user's context isn't overwritten
        with localcontext() as ctx:
            ctx.rounding=ROUNDING_MODE
            rounded = Quantity(round(self.number, ndigits), self.unit)
        return rounded
    
    def sigfig(self, nsigfigs=1):
        digits = self.number.as_tuple().digits
        if nsigfigs <= len(digits):
            exponent = math.floor(self.number.log10())
            significand = self.number / dec(f"1E{exponent}")
            # Set decimal rounding to the traditionally expected method
            # Use in a local context so that user's context isn't overwritten
            with localcontext() as ctx:
                ctx.rounding=ROUNDING_MODE
                rounded_significand = round(significand, nsigfigs - 1)
            return Quantity(rounded_significand * dec(f"1E{exponent}"), self.unit)
        elif nsigfigs > len(digits):
            # Add significant zeroes
            n_digits_to_add = nsigfigs - len(digits)
            new_digits = list(digits)
            for i in n_digits_to_add:
                new_digits.append(0)
            new_exponent = self.number.as_tuple().exponent - n_digits_to_add
            return Quantity(dec((self.number.as_tuple().sign, new_digits, new_exponent)), self.unit)
    
    def exp(self):
        if self.dimensionality() != "(dimensionless)":
            raise NotUnitlessError("Cannot raise to the power of a non-dimensionless quantity!")
        else:
            dimensionless_quant = self.base().cancel()
            return dimensionless_quant.number.exp()
    
    def ln(self):
        if self.dimensionality() != "(dimensionless)":
            raise NotUnitlessError("Cannot take the logarithm of a non-dimensionless quantity!")
        else:
            dimensionless_quant = self.base().cancel()
            return dimensionless_quant.number.ln()
    
    def log10(self):
        if self.dimensionality() != "(dimensionless)":
            raise NotUnitlessError("Cannot take the logarithm of a non-dimensionless quantity!")
        else:
            dimensionless_quant = self.base().cancel()
            return dimensionless_quant.number.log10()
    
    def dimensionality(self):
        """Return the dimensionality as a nice string."""
        return self.unit.dimensionality()

    def base(self):
        """Return the quantity expressed in terms of base units."""
        # Do as multiplication because the Quantity returned by unit.base() might have number != 1
        return self.number * self.unit.base()
    
    def cancel(self):
        """Combine any like terms in the unit."""
        return Quantity(self.number, self.unit.cancel())
    
    def canonical(self):
        """Express the quantity in base units in a canonical order."""
        cancelled = self.base().cancel()
        return Quantity(cancelled.number, cancelled.unit.canonical())
    
    def to(self, unit):
        """Express the quantity in terms of another unit."""
        # Convert both to quantities in base units, then divide, then cancel to get ratio
        result = (self.base() / unit.base()).cancel()
        return Quantity(result.number, result.unit * unit)
        