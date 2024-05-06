from decimal import Decimal as dec
from decimal import localcontext
from fractions import Fraction as frac
import math
from typing import Self

from .config import quanfig
from .format import group_digits
from .uncertainties import get_uncertainty
from .unitreg import unit_reg


class MismatchedUnitsError(Exception):
    pass

class NotDimensionlessError(Exception):
    pass

class Quantity:
    """A class that represents physical quantities.
    
    The quantity is expressed as a numerical value `number` and a unit of measurement `unit`, with an
    optional associated `uncertainty`, which is also a numerical value.
    `number` is any type that can be converted to `Decimal`, including strings.
    `unit` is any `BaseUnit` or `DerivedUnit`, or any `CompoundUnit` formed by multiplication thereof,
    or any instance of the special `Unitless` unit.
    `uncertainty` is also any type that can be converted to `Decimal`, including strings, however if
    `uncertainty` is not specified or given as `0` or `None` (default) or anything else "falsy" then
    the quantity`s uncertainty is set to `0`.
    `uncertainty` may also be a `Quantity` itself, which is useful if it was calculated separately.

    Normally, `number` and `unit` are both required. To create a unitless quantity, the special unitless
    unit (found in the main registry i.e. under `quanstants.units.unitless`) should be provided.

    Alternatively, all three may be left as `None` and `value` maybe specified instead as either a
    string or another `Quantity`. Passing a string simply invokes `Quantity.parse(value)`.

    If only a single string is passed to `number`, and no unit is provided, the string will be parsed
    by `Quantity.parse()`, so if it contains both a number and a unit, an appropriate `Quantity` will
    be created.

    However, while both the above work, the preferred method for quantity creation from a single string
    is to call `Quantity.parse(string)`.

    Both `number` and `uncertainty` are stored internally as `Decimal` objects and provided values are
    first converted to `Decimal`. By making `Decimal` the default, quantities behave as the user would
    expect:
    * numbers can be represented exactly, and arithmetic results in other exact numbers;
    * significant figures are incorporated and significant trailing zeroes are not dropped;
    * quantities round according to users' expectations (see `Quantity.round()`).

    Mathematical operations are in general performed with the quantity considered to be the product of
    its number and unit. The implementation of arithmetic on the numerical part(s) is typically that of
    the `Decimal` type.

    Arithmetic with instances of `Quantity` can be performed in the usual way with the operators
    `+ - * / **`. The results will posess the correct number to the correct precision, the correct
    compound unit, and the correct uncertainty.

    Addition and subtraction will raise a `MismatchedUnitsError` if the units of the two
    do not match, which serves as a useful sanity check. Quantities with the same dimension,
    however, can be added and subtracted, and the result will have the unit of the first of the two
    values:

    ```python
    >>> from quanstants import units as qu
    >>> (4 * qu.metre) + (50 * qu.centimetre)
    Quantity(4.5, m)
    >>> (50 * qu.centimetre) + (4 * qu.metre)
    Quantity(450, cm)
    >>> (4 * qu.metre) + (3 * qu.kilogram)
    quanstants.quantity.MismatchedUnitsError: Can't add quantity in Unit(m) to quantity in Unit(kg).
    ```

    Similarly, (in)equalities between quantities with the same dimension are supported.

    Quantities can be raised to either integer or fractional powers (with `fractions.Fraction`).
    Quantities are not themselves valid exponents unless they are dimensionless.

    As is the case for `Decimal`, the mathematical functions `.sqrt()`, `.exp()`, `.ln()`, and
    `.log10()` are available. Once again, however, these can only be used on dimensionless quantities.
    Whether a quantity is dimensionless can be checked with `.is_dimensionless()`.

    Rounding can be performed either to an integer, a given number of decimal places, or a given number
    of significant figures.

    By default, units of the results of arithmetic with quantities are not cancelled out automatically.
    This can be done on demand using `.cancel()`.

    Quantities can be easily expressed in terms of another unit using `.to()`. To express in terms of
    SI base units, `.base()` is provided.
    
    `.canonical()` returns the quantity with its units in a set, reproducible order.

    Note that when uncertainties are calculated automatically it is done under the assumption that two
    quantities are completely uncorrelated with correlation = covariance = 0. This can lead to errors.
    If you know the correlation between two quantities, you can get the correct uncertainty by calling
    the appropriate dunder method directly and passing the correlation as a second, optional argument
    e.g. if `a` and `b` are correlated quantities and you want `c = a * b`, use `c = a.__mul__(b, 0.9)`
    instead.
    
    If `quanstants.quanfig.CONVERT_FLOAT_AS_STR` is `True`, as it is by default, all provided `float`
    values are first converted to `str`, then to `Decimal`. The result is that providing `number=5.2`
    gives a `Quantity` with the exact numerical value 5.2, as the user likely expects, as opposed to
    the true decimal representation of the binary float 5.2, which they likely don't:

    ```python
    >>> from quanstants import units as qu, Quantity
    >>> 5.2 * qu.kilogram
    Quantity(5.2, kg)
    >>> Quantity(5.2, qu.kilogram)
    Quantity(5.2, kg)
    >>> Quantity(5.2, qu.kilogram).number
    Decimal('5.2')
    >>> Decimal(5.2) # Compare with the above
    Decimal('5.20000000000000017763568394002504646778106689453125')
    >>> Decimal("5.2")
    Decimal('5.2')
    >>> Decimal(str(5.2))
    Decimal('5.2')
    ```

    This behaviour lowers the barrier to entry for non-expert users. However, if for whatever reason it
    should be desirable to have floats be converted directly to decimals, set
    `quanstants.quanfig.CONVERT_FLOAT_AS_STR=False`.
    """
    def __init__(
        self,
        number: str | int | float | dec | None = None,
        unit = None,
        uncertainty: str | int | float | dec | Self | None = None,
        value: str | Self | None = None,
    ):
        # Making a Quantity from a single string should be done with `Quantity.parse()`, but allow for the
        # fact that people might also just try to pass a string straight to `Quantity()`
        if (isinstance(number, str)) and (unit is None) and (uncertainty is None) and (value is None):
            parsed = self.parse(number)
            number, unit, uncertainty = parsed.number, parsed.unit, parsed.uncertainty
        # Accept a string or other Quantity
        if (number is None) and (unit is None) and (value is not None):
            if isinstance(value, str):
                parsed = self.parse(value)
                number, unit, uncertainty = parsed.number, parsed.unit, parsed.uncertainty
            else:
                number, unit, uncertainty = value.number, value.unit, value.uncertainty
        if quanfig.CONVERT_FLOAT_AS_STR:
            self._number = dec(str(number))
        else:
            self._number = dec(number)
        if isinstance(unit, str):
            self._unit = unit_reg.parse(unit)
        elif unit is None:
            # Have to do the import here to avoid circular import
            from .unit import unitless
            self._unit = unitless
        else:
            self._unit = unit
        if not uncertainty:
            self._uncertainty = dec("0")
        elif isinstance(uncertainty, Quantity):
            if uncertainty.unit != self._unit:
                self._uncertainty = uncertainty.to(self._unit).number
            else:
                self._uncertainty = uncertainty.number
        elif quanfig.CONVERT_FLOAT_AS_STR:
            self._uncertainty = dec(str(uncertainty))
        else:
            self._uncertainty = dec(uncertainty)
    
    # Always access properties via self.x not self._x for consistency
    # self._x is slightly faster, but even for time-critical operations it makes v little difference
    # e.g. for Quantity(2, m) * Quantity(3.4, s**-1) the time saving was only 1.5% (off ~10 µs)
    @property
    def number(self):
        return self._number
    
    @property
    def unit(self):
        return self._unit
    
    # Note that the uncertainty is returned to the user as a Quantity, while internally usually the
    # decimal value should be accessed with _uncertainty
    @property
    def uncertainty(self):
        return Quantity(self._uncertainty, self._unit)

    def __repr__(self):
        if not self._uncertainty:
            return f"Quantity({group_digits(self.number)}, {self.unit.symbol})"
        else:
            return f"Quantity({group_digits(self.number)}, {self.unit.symbol}, uncertainty={group_digits(self._uncertainty)})"

    def __str__(self):
        if not self._uncertainty:
            return f"{group_digits(self.number)} {self.unit.symbol}"
        # Check that uncertainty is not more precise than the number via the exponent
        # More negative (smaller) exponent means more precise
        elif (self.number.as_tuple().exponent <= self._uncertainty.as_tuple().exponent) and (quanfig.UNCERTAINTY_STYLE == "PARENTHESES"):
            return f"{group_digits(self.number)}({''.join([str(n) for n in self._uncertainty.as_tuple().digits])}) {self.unit.symbol}"
        else:
            return f"{group_digits(self.number)} ± {group_digits(self._uncertainty)} {self.unit.symbol}"
            
    def __int__(self):
        if not self.is_dimensionless():
            raise NotDimensionlessError("Cannot cast a non-dimensionless quantity to an integer!")
        else:
            dimensionless_quant = self.base().cancel()
            return int(dimensionless_quant.number)
    
    def __float__(self):
        if not self.is_dimensionless():
            raise NotDimensionlessError("Cannot cast a non-dimensionless quantity to a float!")
        else:
            dimensionless_quant = self.base().cancel()
            return float(dimensionless_quant.number)
    
    def __add__(self, other, correlation=0):
        if isinstance(other, Quantity):
            if self.unit == other.unit:
                new_number = self.number + other.number
                new_uncertainty = get_uncertainty(new_number, "add", self, quantityB=other, correlation=correlation)
            # Allow mixed units with the same dimension
            elif self.unit.dimensional_exponents == other.unit.dimensional_exponents:
                converted = other.to(self.unit)
                new_number = self.number + converted.number
                new_uncertainty = get_uncertainty(new_number, "add", self, quantityB=converted, correlation=correlation)
            else:
                raise MismatchedUnitsError(f"Can't add quantity in {other.unit} to quantity in {self.unit}.")
            return Quantity(new_number, self.unit, new_uncertainty)
        else:
            return NotImplemented
    
    def __sub__(self, other, correlation=0):
        if isinstance(other, Quantity):
            if self.unit == other.unit:
                new_number = self.number - other.number
                new_uncertainty = get_uncertainty(new_number, "sub", self, quantityB=other, correlation=correlation)
            # Allow mixed units with the same dimension
            elif self.unit.dimensional_exponents == other.unit.dimensional_exponents:
                converted = other.to(self.unit)
                new_number = self.number - converted.number
                new_uncertainty = get_uncertainty(new_number, "sub", self, quantityB=converted, correlation=correlation)
            else:
                raise MismatchedUnitsError(f"Can't subtract quantity in {other.unit} from quantity in {self.unit}.")
            return Quantity(new_number, self.unit, new_uncertainty)
        else:
            return NotImplemented

    def __mul__(self, other, correlation=0):
        if isinstance(other, (str, int, float, dec)):
            if quanfig.CONVERT_FLOAT_AS_STR:
                other = dec(str(other))
            else:
                other = dec(other)
            new_number = self.number * other
            new_uncertainty = get_uncertainty(new_number, "mul", self, numberx=other, correlation=correlation)
            return Quantity(new_number, self.unit, new_uncertainty)
        elif isinstance(other, Quantity):
            new_number = self.number * other.number
            new_uncertainty = get_uncertainty(new_number, "mul", self, quantityB=other, correlation=correlation)
            result = Quantity(new_number, self.unit * other.unit, new_uncertainty)
            if quanfig.AUTO_CANCEL:
                return result.cancel()
            else:
                return result
        # Check if it's a unit with duck typing
        elif hasattr(other, "base") and hasattr(other, "components"):
            result = Quantity(self.number, self.unit * other)
            if quanfig.AUTO_CANCEL:
                return result.cancel()
            else:
                return result
        else:
            return NotImplemented
    
    def __rmul__(self, other, correlation=0):
        if isinstance(other, (str, int, float, dec)):
            if quanfig.CONVERT_FLOAT_AS_STR:
                other = dec(str(other))
            else:
                other = dec(other)
            new_number = other * self.number
            new_uncertainty = get_uncertainty(new_number, "mul", self, numberx=other, correlation=correlation)
            return Quantity(new_number, self.unit, new_uncertainty)
        # Check if it's a unit with duck typing
        elif hasattr(other, "base") and hasattr(other, "components"):
            result = Quantity(self.number, other * self.unit)
            if quanfig.AUTO_CANCEL:
                return result.cancel()
            else:
                return result
        else:
            return NotImplemented
    
    def __truediv__(self, other, correlation=0):
        if isinstance(other, (str, int, float, dec)):
            if quanfig.CONVERT_FLOAT_AS_STR:
                other = dec(str(other))
            else:
                other = dec(other)
            new_number = self.number / other
            new_uncertainty = get_uncertainty(new_number, "truediv", self, numberx=other, correlation=correlation)
            return Quantity(new_number, self.unit, new_uncertainty)
        elif isinstance(other, Quantity):
            new_number = self.number / other.number
            new_uncertainty = get_uncertainty(new_number, "truediv", self, quantityB=other, correlation=correlation)
            result = Quantity(new_number, self.unit / other.unit, new_uncertainty)
            if quanfig.AUTO_CANCEL:
                return result.cancel()
            else:
                return result
        # Check if it's a unit with duck typing
        elif hasattr(other, "base") and hasattr(other, "components"):
            return Quantity(self.number, self.unit / other)
        else:
            return NotImplemented
    
    def __rtruediv__(self, other, correlation=0):
        if isinstance(other, (str, int, float, dec)):
            if quanfig.CONVERT_FLOAT_AS_STR:
                other = dec(str(other))
            else:
                other = dec(other)
            new_number = other / self.number
            new_uncertainty = get_uncertainty(new_number, "rtruediv", self, numberx=other, correlation=correlation)
            return Quantity(new_number, self.unit.inverse(), new_uncertainty)
        # Check if it's a unit with duck typing
        elif hasattr(other, "base") and hasattr(other, "components"):
            return Quantity(1 / self.number, other / self.unit)
        else:
            return NotImplemented
    
    # For now Unit only supports integer or fractional exponents
    def __pow__(self, other):
        if isinstance(other, int):
            new_number = self.number ** other
            new_uncertainty = get_uncertainty(new_number, "pow", self, numberx=other)
            return Quantity(new_number, self.unit ** other, new_uncertainty)
        elif isinstance(other, frac):
            frac_as_dec = dec(str(float(other)))
            new_number = self.number ** frac_as_dec
            new_uncertainty = get_uncertainty(new_number, "pow", self, numberx=frac_as_dec)
            return Quantity(new_number, self.unit ** other, new_uncertainty)
        else:
            return NotImplemented
    
    # Can only use a Quantity as an exponent if it is unitless
    def __rpow__(self, other, correlation=0):
        if not self.is_dimensionless():
            raise NotDimensionlessError("Cannot raise to the power of a non-dimensionless quantity!")
        else:
            dimensionless_quant = self.base().cancel()
        if isinstance(other, (str, int, float, dec)):
            if quanfig.CONVERT_FLOAT_AS_STR:
                other = dec(str(other))
            else:
                other = dec(other)
            new_number = other ** dimensionless_quant.number
            new_uncertainty = get_uncertainty(new_number, "rpow", self, numberx=other, correlation=correlation)
            return Quantity(new_number, None, new_uncertainty)
        elif isinstance(other, Quantity):
            # For now don't support as Unit only supports integer or fractional exponents, and the
            # chances of dimensionless_quant.number being those is very low
            return NotImplemented
        else:
            return NotImplemented
    
    def __eq__(self, other):
        if isinstance(other, Quantity):
            # Convert both to canonical base unit representations
            a = self.base().cancel().canonical()
            b = other.base().cancel().canonical()
            # Have to use dimension as a sanity check in case different units have the same symbol
            if (a.number == b.number) and (a.unit.symbol == b.unit.symbol) and (a.unit.dimensional_exponents == b.unit.dimensional_exponents):
                return True
            else:
                return False
        elif other == 0:
            if self.number == 0:
                return True
            else:
                return False
        else:
            return NotImplemented
    
    def __gt__(self, other):
        if isinstance(other, Quantity):
            # Convert both to canonical base unit representations
            a = self.base().cancel().canonical()
            b = other.base().cancel().canonical()
            if (a.unit.dimensional_exponents != b.unit.dimensional_exponents):
                raise MismatchedUnitsError
            elif (a.number > b.number) and (a.unit.symbol == b.unit.symbol) and (a.unit.dimensional_exponents == b.unit.dimensional_exponents):
                return True
            else:
                return False
        else:
            return NotImplemented

    def __ge__(self, other):
        if isinstance(other, Quantity):
            # Convert both to canonical base unit representations
            a = self.base().cancel().canonical()
            b = other.base().cancel().canonical()
            if (a.unit.dimensional_exponents != b.unit.dimensional_exponents):
                raise MismatchedUnitsError
            elif (a.number >= b.number) and (a.unit.symbol == b.unit.symbol) and (a.unit.dimensional_exponents == b.unit.dimensional_exponents):
                return True
            else:
                return False
        else:
            return NotImplemented

    def __neg__(self):
        return Quantity(-1 * self.number, self.unit, -1 * self._uncertainty)
    
    def __pos__(self):
        return self
    
    def __round__(self, ndigits):
        """Alias for `Quantity.round()` to allow the use of the in-built `round()`."""
        return self.round(ndigits)
    
    def round(self, ndigits):
        """Return the quantity with the numerical part rounded by the set method.
        
        By default, rounds to the specified number of decimal places.

        Calls one of the other rounding methods depending on the value of `quanfig.ROUND_TO`:
        "PLACES" (default) ⇒ `Quantity.places()`
        "FIGURES" ⇒ `Quantity.sigfigs()`
        "UNCERTAINTY" ⇒ `Quantity.round_to_uncertainty()`
        """
        if quanfig.ROUND_TO == "FIGURES":
            return self.sigfigs(ndigits)
        elif quanfig.ROUND_TO == "UNCERTAINTY":
            return self.round_to_uncertainty(ndigits)
        else:
            return self.places(ndigits)

    def places(self, ndigits=0):
        """Return the quantity with the numerical part rounded to the specified number of decimal places.
        
        The method used for rounding is that specified by the `quanstants.ROUNDING_MODE` variable,
        which takes any of the `decimal` module's rounding modes. The default is `"ROUND_HALF_UP"`, i.e.
        to nearest with ties going away from zero.

        Like siunitx, by default extra zeroes will be added to a short number to reach the desired
        number of decimal places. This can be turned off by changing `quanfig.ROUND_PAD`.

        Note that the rounding is done within a `decimal.localcontext()`, which means that the mode
        specified by `quanstants.ROUNDING_MODE` does not override the current `decimal.Context()`
        and other `Decimal` instances will continue to round based on `decimal.getcontext().rounding`,
        which by default uses `"ROUND_HALF_EVEN"`.
        """
        current_places = self.number.as_tuple().exponent * -1
        # Don't round if padding is turned off and the number doesn't have enough places
        if (not quanfig.ROUND_PAD) and (current_places < ndigits):
            return self
        # Set decimal rounding to the specified method, which by default is the traditionally
        # expected behaviour
        # Use in a local context so that user's context isn't overwritten
        with localcontext() as ctx:
            ctx.rounding=quanfig.ROUNDING_MODE
            rounded = Quantity(round(self.number, ndigits), self.unit)
        return rounded
    
    def sigfigs(self, nsigfigs=1):
        """Return the quantity with the numerical part rounded to the specified number of significant figures.
        
        The method used for rounding is that specified by `quanfig.ROUNDING_MODE`, which takes
        any of the `decimal` module's rounding modes. The default is `"ROUND_HALF_UP"`, i.e. to 
        nearest with ties going away from zero.

        Like siunitx, by default extra zeroes will be added to a short number to reach the desired
        number of significant figures. This can be turned off by changing `quanfig.ROUND_PAD`.

        Note that the rounding is done within a `decimal.localcontext()`, which means that the mode
        specified by `quanfig.ROUNDING_MODE` does not override the current `decimal.Context()`
        and other `Decimal` instances will continue to round based on `decimal.getcontext().rounding`,
        which by default uses `"ROUND_HALF_EVEN"`.
        """
        # Sanity check for requested number of sigfigs
        if nsigfigs < 1:
            return self
        digits = self.number.as_tuple().digits
        if nsigfigs <= len(digits):
            exponent = math.floor(self.number.log10())
            significand = self.number / dec(f"1E{exponent}")
            # Set decimal rounding to the specified method, which by default is the traditionally
            # expected behaviour
            # Use in a local context so that user's context isn't overwritten
            with localcontext() as ctx:
                ctx.rounding=quanfig.ROUNDING_MODE
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
    
    def round_to_uncertainty(self, nsigfigs=1):
        """Round the uncertainty to the specified number of digits, then return the quantity with the numerical part rounded to the same precision.
        
        The method used for rounding is that specified by `quanfig.ROUNDING_MODE`, which takes
        any of the `decimal` module's rounding modes. The default is `"ROUND_HALF_UP"`, i.e. to 
        nearest with ties going away from zero.

        Like siunitx, by default extra zeroes will be added to a short number to reach the same number
        of digits as the uncertainty. This can be turned off by changing `quanfig.ROUND_PAD`.
        However, no padding is applied to the uncertainty, so the rounded quantity will never have a
        precision greater than the original uncertainty.

        Note that the rounding is done within a `decimal.localcontext()`, which means that the mode
        specified by `quanfig.ROUNDING_MODE` does not override the current `decimal.Context()`
        and other `Decimal` instances will continue to round based on `decimal.getcontext().rounding`,
        which by default uses `"ROUND_HALF_EVEN"`.
        """
        # Check that the quantity even has an uncertainty
        if not self._uncertainty:
            return self
        # First round the uncertainty
        digits = self._uncertainty.as_tuple().digits
        # Sanity check for requested number of sigfigs
        # Also don't allow increasing the precision of the uncertainty
        if (nsigfigs < 1) or (nsigfigs > len(digits)):
            return self
        else:
            exponent = math.floor(self._uncertainty.log10())
            significand = self._uncertainty / dec(f"1E{exponent}")
            # Set decimal rounding to the specified method, which by default is the traditionally
            # expected behaviour
            # Use in a local context so that user's context isn't overwritten
            with localcontext() as ctx:
                ctx.rounding=quanfig.ROUNDING_MODE
                rounded_significand = round(significand, nsigfigs - 1)
            rounded_uncertainty = rounded_significand * dec(f"1E{exponent}")
        # Now round the number to the same precision
        uncertainty_places = rounded_uncertainty.as_tuple().exponent * -1
        return self.places(uncertainty_places)
    
    def sqrt(self):
        """Return the square root of the quantity, equivalent to `Quantity**Fraction(1, 2)`."""
        return self**frac(1, 2)

    def exp(self):
        """Return the value of e raised to the power of the quantity, for dimensionless quantities only."""
        if not self.is_dimensionless():
            raise NotDimensionlessError("Cannot raise to the power of a non-dimensionless quantity!")
        else:
            dimensionless_quant = self.base().cancel()
            new_number = dimensionless_quant.number.exp()
            new_uncertainty = get_uncertainty(new_number, "exp", self)
            return Quantity(new_number, dimensionless_quant.unit, new_uncertainty)
    
    def ln(self):
        """Return the natural logarithm of the quantity, for dimensionless quantities only."""
        if not self.is_dimensionless():
            raise NotDimensionlessError("Cannot take the logarithm of a non-dimensionless quantity!")
        else:
            dimensionless_quant = self.base().cancel()
            new_number = dimensionless_quant.number.ln()
            new_uncertainty = get_uncertainty(new_number, "ln", self)
            return Quantity(new_number, dimensionless_quant.unit, new_uncertainty)
    
    def log10(self):
        """Return the base-10 logarithm of the quantity, for dimensionless quantities only."""
        if not self.is_dimensionless():
            raise NotDimensionlessError("Cannot take the logarithm of a non-dimensionless quantity!")
        else:
            dimensionless_quant = self.base().cancel()
            new_number = dimensionless_quant.number.log10()
            new_uncertainty = get_uncertainty(new_number, "log10", self)
            return Quantity(new_number, dimensionless_quant.unit, new_uncertainty)
    
    def is_dimensionless(self):
        """Check if unit is dimensionless."""
        return self.unit.is_dimensionless()

    def dimension(self):
        """Return the dimension as a nice string."""
        return self.unit.dimension()

    def with_uncertainty(self, uncertainty):
        """Return a new quantity with the provided uncertainty."""
        return Quantity(self.number, self.unit, uncertainty)
    
    def plusminus(self, uncertainty):
        """Alias for `.with_uncertainty()`."""
        return self.with_uncertainty(uncertainty)

    def base(self):
        """Return the quantity expressed in terms of base units."""
        if not self._uncertainty:
            return self.number * self.unit.base()
        else:
            return (self.number * self.unit.base()).with_uncertainty(self.uncertainty.base().number)
    
    def cancel(self):
        """Combine any like terms in the unit."""
        return (self.number * self.unit.cancel()).with_uncertainty(self.uncertainty.number)
    
    def fully_cancel(self):
        """Combine any like terms in the unit, with units of the same dimension converted and combined."""
        if not self._uncertainty:
            return self.number * self.unit.fully_cancel()
        else:
            return (self.number * self.unit.fully_cancel()).with_uncertainty(self.uncertainty.fully_cancel().number)
    
    def canonical(self):
        """Express the quantity with its units in a canonical order."""
        if not self._uncertainty:
            return self.number * self.unit.canonical()
        else:
            return (self.number * self.unit.canonical()).with_uncertainty(self.uncertainty.canonical().number)
    
    def to(self, other):
        """Express the quantity in terms of another unit."""
        if isinstance(other, str):
            # Allow parsing of unit string first
            other = unit_reg.parse(other)
        if self.number == 0:
            if self._uncertainty == 0:
                return Quantity(0, other)
            else:
                return Quantity(0, other, self.uncertainty.to(other))
        else:
            # Convert both args to quantities in base units, then divide, then cancel to get ratio
            result = (self.base() / other.base()).cancel()
            if not self._uncertainty:
                return Quantity(result.number, result.unit._mul_with_concat(other))
            else:
                return Quantity(result.number, result.unit._mul_with_concat(other), self.uncertainty.to(other))

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
        The unit string is parsed by `quanstants.units.parse()` (where `quanstants.units` is the main
        instance of `quanstants.unitreg.UnitReg`), so it must follow the same rules as for that.
        """
        # Split at first whitespace into number part and unit part
        # TODO parse uncertainty too
        split_string = string.split(maxsplit=1)
        if len(split_string) < 2:
            raise ValueError("String must contain both a number and a unit, separated by whitespace.")
        else:
            return cls(split_string[0], split_string[1], None)