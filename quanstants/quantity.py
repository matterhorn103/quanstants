from decimal import Decimal as dec
from decimal import localcontext
from fractions import Fraction as frac
import math

from .config import QuanstantsConfig

class MismatchedUnitsError(Exception):
    pass

class NotUnitlessError(Exception):
    pass

class Quantity:
    """A class that represents physical quantities.
    
    The quantity is expressed as a numerical value `number` and a unit of measurement `unit`, with an
    optional associated `uncertainty`, which is also a numerical value.
    `number` is any type that can be converted to `Decimal`, including strings.
    `unit` is any `BaseUnit` or `DerivedUnit`, or any `CompoundUnit` formed by multiplication thereof,
    or any instance of the special `Unitless` unit.
    `uncertainty` is also any type that can be converted to `Decimal`, including strings, however if
    `uncertainty` is not specified or given as the string `"(exact)"` or `None` (default), the
    quantity`s uncertainty is set to `"(exact)"`.

    Both `number` and `uncertainty` are stored internally as `Decimal` objects and provided values are
    first converted to `Decimal`. By making `Decimal` the default, quantities behave as the user would
    expect:
    * numbers can be represented exactly, and arithmetic results in other exact numbers;
    * significant figures are incorporated and significant trailing zeroes are not dropped;
    * quantities round according to users' expectations (see `Quantity.round()`).

    Mathematical operations are in general performed with the quantity considered to be the product of
    its number and unit. The implementation of arithmetic on the numerical part is typically that of
    the `Decimal` type.

    Arithmetic with instances of `Quantity` can be performed in the usual way with the operators `*` and
    `/`. The results will posess the correct number to the correct precision, the correct compound
    `+ - * / **`. Addition and subtraction will raise a `MismatchedUnitsError` if the units of the two
    do not match, which serves as a useful sanity check. Quantities with the same dimension,
    however, can be added and subtracted, and the result will have the unit of the first of the two
    values:

    ```
    >>> (4 * units.metre) + (50 * units.centimetre)
    Quantity(4.5, m)
    >>> (50 * units.centimetre) + (4 * units.metre)
    Quantity(450, cm)
    >>> (4 * units.metre) + (3 * units.kilogram)
    quanstants.quantity.MismatchedUnitsError: Can't add quantity in Unit(m) to quantity in Unit(kg).
    ```

    Similarly, (in)equalities between quantities with the same dimension are supported.

    Rounding can be performed either to an integer, a given number of decimal places, or a given number
    of significant figures.

    As for `Decimal`, the mathematical functions `.sqrt()`, `.exp()`, `.ln()`, and `.log10()` are
    available.

    By default, units of the results of arithmetic with quantities are not cancelled out automatically.
    This can be done on demand using `.cancel()`.

    Quantities can be easily expressed in terms of another unit using `.to()`. To express in terms of
    SI base units, `.base()` is provided.
    
    `.canonical()` returns the quantity with its units in a set, reproducible order.
    
    If `quanstants.CONVERT_FLOAT_AS_STR` is `True`, as it is by default, a provided `float` is first
    converted to a `str`, then to a `Decimal`. The result is that providing `number=5.2` gives a
    `Quantity` with the exact numerical value 5.2, as the user likely expects, as opposed to the true
    decimal representation of the binary float 5.2, which they likely don't:

    ```
    >>> 5.2 * units.kilogram
    Quantity(5.2, kg)
    >>> Quantity(5.2, units.kilogram)
    Quantity(5.2, kg)
    >>> Quantity(5.2, units.kilogram).number
    Decimal('5.2')
    >>> Decimal(5.2)
    Decimal('5.20000000000000017763568394002504646778106689453125')
    >>> Decimal("5.2")
    Decimal('5.2')
    >>> Decimal(str(5.2))
    Decimal('5.2')
    ```

    This behaviour lowers the barrier to entry for non-expert users. However, if for whatever reason it
    should be desirable to have floats be converted directly to decimals, set
    `quanstants.CONVERT_FLOAT_AS_STR=False`.
    """
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
        if QuanstantsConfig.CONVERT_FLOAT_AS_STR:
            self._number = dec(str(number))
        else:
            self._number = dec(number)
        self._unit = unit
        if (uncertainty is None) or (uncertainty == "(exact)"):
            self._uncertainty = "(exact)"
        elif QuanstantsConfig.CONVERT_FLOAT_AS_STR:
            self._uncertainty = dec(str(uncertainty))
        else:
            self._uncertainty = dec(uncertainty)
    
    @property
    def number(self):
        return self._number
    
    @property
    def unit(self):
        return self._unit
    
    @property
    def uncertainty(self):
        return self._uncertainty

    def __repr__(self):
        if self.uncertainty == "(exact)":
            return f"Quantity({self.number}, {self.unit.symbol})"
        else:
            return f"Quantity({self.number}, {self.unit.symbol}, uncertainty={self.uncertainty})"

    def __str__(self):
        if self.uncertainty == "(exact)":
            return f"{self.number} {self.unit.symbol}"
        else:
            return f"{self.number}({''.join([str(n) for n in self.uncertainty.as_tuple().digits])}) {self.unit.symbol}"
            
        
    def __int__(self):
        if self.dimension() != "(dimensionless)":
            raise NotUnitlessError("Cannot cast a non-dimensionless quantity to an integer!")
        else:
            dimensionless_quant = self.base().cancel()
            return int(dimensionless_quant.number)
    
    def __float__(self):
        if self.dimension() != "(dimensionless)":
            raise NotUnitlessError("Cannot cast a non-dimensionless quantity to a float!")
        else:
            dimensionless_quant = self.base().cancel()
            return float(dimensionless_quant.number)
    
    def __add__(self, other):
        if isinstance(other, Quantity):
            if self.unit == other.unit:
                return Quantity(self.number + other.number, self.unit)
            # Allow mixed units with the same dimension
            elif self.unit.dimensional_exponents == other.unit.dimensional_exponents:
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
            elif self.unit.dimensional_exponents == other.unit.dimensional_exponents:
                converted = other.to(self.unit)
                return Quantity(self.number - converted.number, self.unit)
            else:
                raise MismatchedUnitsError(f"Can't subtract quantity in {other.unit} from quantity in {self.unit}.")
        else:
            return NotImplemented

    def __mul__(self, other):
        if isinstance(other, (str, int, float, dec)):
            return Quantity(self.number * dec(str(other)), self.unit)
        elif isinstance(other, Quantity):
            return Quantity(self.number * other.number, self.unit * other.unit)
        # Check if it's a unit with duck typing
        elif hasattr(other, "base") and hasattr(other, "components"):
            return Quantity(self.number, self.unit * other)
        else:
            return NotImplemented
    
    def __rmul__(self, other):
        if isinstance(other, (str, int, float, dec)):
            return Quantity(dec(str(other)) * self.number, self.unit)
        # Check if it's a unit with duck typing
        elif hasattr(other, "base") and hasattr(other, "components"):
            return Quantity(self.number, other * self.unit)
        else:
            return NotImplemented
    
    def __truediv__(self, other):
        if isinstance(other, (str, int, float, dec)):
            return Quantity(self.number / dec(str(other)), self.unit)
        elif isinstance(other, Quantity):
            return Quantity(self.number / other.number, self.unit / other.unit)
        # Check if it's a unit with duck typing
        elif hasattr(other, "base") and hasattr(other, "components"):
            return Quantity(self.number, self.unit / other)
        else:
            return NotImplemented
    
    def __rtruediv__(self, other):
        if isinstance(other, (str, int, float, dec)):
            return Quantity(dec(str(other)) / self.number, self.unit)
        # Check if it's a unit with duck typing
        elif hasattr(other, "base") and hasattr(other, "components"):
            return Quantity(1 / self.number, other / self.unit)
        else:
            return NotImplemented
    
    # For now Unit only supports integer or fractional exponents
    def __pow__(self, other):
        if isinstance(other, int):
            return Quantity(self.number ** other, self.unit ** other)
        elif isinstance(other, frac):
            return Quantity(self.number ** dec(str(float(other))), self.unit ** other)
        else:
            return NotImplemented
    
    # Can only use a Quantity as an exponent if it is unitless
    def __rpow__(self, other):
        if self.dimension() != "(dimensionless)":
            raise NotUnitlessError("Cannot raise to the power of a non-dimensionless quantity!")
        else:
            dimensionless_quant = self.base().cancel()
            return other ** (dimensionless_quant.number)
    
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
        return Quantity(-1 * self.number, self.unit)
    
    def __pos__(self):
        return self
    
    def __round__(self, ndigits):
        """Alias for `Quantity.round()` to allow the use of the in-built `round()`."""
        return self.round(ndigits)
    
    def round(self, ndigits):
        """Return the quantity with the numerical part rounded by the set method.
        
        By default, rounds to the specified number of decimal places.

        Calls one of the other rounding methods depending on the value of `QuanstantsConfig.ROUND_TO`:
        "PLACES" (default) ⇒ `Quantity.places()`
        "FIGURES" ⇒ `Quantity.sigfigs()`
        "UNCERTAINTY" ⇒ `Quantity.round_to_uncertainty()`
        """
        if QuanstantsConfig.ROUND_TO == "FIGURES":
            return self.sigfigs(ndigits)
        elif QuanstantsConfig.ROUND_TO == "UNCERTAINTY":
            return self.round_to_uncertainty(ndigits)
        else:
            return self.places(ndigits)

    def places(self, ndigits=0):
        """Return the quantity with the numerical part rounded to the specified number of decimal places.
        
        The method used for rounding is that specified by the `quanstants.ROUNDING_MODE` variable,
        which takes any of the `decimal` module's rounding modes. The default is `"ROUND_HALF_UP"`, i.e.
        to nearest with ties going away from zero.

        Like siunitx, by default extra zeroes will be added to a short number to reach the desired
        number of decimal places. This can be turned off by changing `QuanstantsConfig.ROUND_PAD`.

        Note that the rounding is done within a `decimal.localcontext()`, which means that the mode
        specified by `quanstants.ROUNDING_MODE` does not override the current `decimal.Context()`
        and other `Decimal` instances will continue to round based on `decimal.getcontext().rounding`,
        which by default uses `"ROUND_HALF_EVEN"`.
        """
        current_places = self.number.as_tuple().exponent * -1
        # Don't round if padding is turned off and the number doesn't have enough places
        if (not QuanstantsConfig.ROUND_PAD) and (current_places < ndigits):
            return self
        # Set decimal rounding to the specified method, which by default is the traditionally
        # expected behaviour
        # Use in a local context so that user's context isn't overwritten
        with localcontext() as ctx:
            ctx.rounding=QuanstantsConfig.ROUNDING_MODE
            rounded = Quantity(round(self.number, ndigits), self.unit)
        return rounded
    
    def sigfigs(self, nsigfigs=1):
        """Return the quantity with the numerical part rounded to the specified number of significant figures.
        
        The method used for rounding is that specified by `QuanstantsConfig.ROUNDING_MODE`, which takes
        any of the `decimal` module's rounding modes. The default is `"ROUND_HALF_UP"`, i.e. to 
        nearest with ties going away from zero.

        Like siunitx, by default extra zeroes will be added to a short number to reach the desired
        number of significant figures. This can be turned off by changing `QuanstantsConfig.ROUND_PAD`.

        Note that the rounding is done within a `decimal.localcontext()`, which means that the mode
        specified by `QuanstantsConfig.ROUNDING_MODE` does not override the current `decimal.Context()`
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
                ctx.rounding=QuanstantsConfig.ROUNDING_MODE
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
        
        The method used for rounding is that specified by `QuanstantsConfig.ROUNDING_MODE`, which takes
        any of the `decimal` module's rounding modes. The default is `"ROUND_HALF_UP"`, i.e. to 
        nearest with ties going away from zero.

        Like siunitx, by default extra zeroes will be added to a short number to reach the same number
        of digits as the uncertainty. This can be turned off by changing `QuanstantsConfig.ROUND_PAD`.
        However, no padding is applied to the uncertainty, so the rounded quantity will never have a
        precision greater than the original uncertainty.

        Note that the rounding is done within a `decimal.localcontext()`, which means that the mode
        specified by `QuanstantsConfig.ROUNDING_MODE` does not override the current `decimal.Context()`
        and other `Decimal` instances will continue to round based on `decimal.getcontext().rounding`,
        which by default uses `"ROUND_HALF_EVEN"`.
        """
        # First round the uncertainty
        digits = self.uncertainty.as_tuple().digits
        # Sanity check for requested number of sigfigs
        # Also don't allow increasing the precision of the uncertainty
        if (nsigfigs < 1) or (nsigfigs > len(digits)):
            return self
        else:
            exponent = math.floor(self.uncertainty.log10())
            significand = self.uncertainty / dec(f"1E{exponent}")
            # Set decimal rounding to the specified method, which by default is the traditionally
            # expected behaviour
            # Use in a local context so that user's context isn't overwritten
            with localcontext() as ctx:
                ctx.rounding=QuanstantsConfig.ROUNDING_MODE
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
        if self.dimension() != "(dimensionless)":
            raise NotUnitlessError("Cannot raise to the power of a non-dimensionless quantity!")
        else:
            dimensionless_quant = self.base().cancel()
            return dimensionless_quant.number.exp()
    
    def ln(self):
        """Return the natural logarithm of the quantity, for dimensionless quantities only."""
        if self.dimension() != "(dimensionless)":
            raise NotUnitlessError("Cannot take the logarithm of a non-dimensionless quantity!")
        else:
            dimensionless_quant = self.base().cancel()
            return dimensionless_quant.number.ln()
    
    def log10(self):
        """Return the base-10 logarithm of the quantity, for dimensionless quantities only."""
        if self.dimension() != "(dimensionless)":
            raise NotUnitlessError("Cannot take the logarithm of a non-dimensionless quantity!")
        else:
            dimensionless_quant = self.base().cancel()
            return dimensionless_quant.number.log10()
    
    def dimension(self):
        """Return the dimension as a nice string."""
        return self.unit.dimension()

    def base(self):
        """Return the quantity expressed in terms of base units."""
        return self.number * self.unit.base()
    
    def cancel(self):
        """Combine any like terms in the unit."""
        return self.number * self.unit.cancel()
    
    def fully_cancel(self):
        """Combine any like terms in the unit, with units of the same dimension converted and combined."""
        return self.number * self.unit.fully_cancel()
    
    def canonical(self):
        """Express the quantity with its units in a canonical order."""
        return self.number * self.unit.canonical()
    
    def to(self, other):
        """Express the quantity in terms of another unit (or rarely, a quantity)."""
        # Allowing other to be a quantity means that quantities can be expressed in terms of
        # natural/atomic/Planck units, e.g. particle masses in units of MeV/c**2
        # Convert both args to quantities in base units, then divide, then cancel to get ratio
        result = (self.base() / other.base()).cancel()
        if isinstance(other, Quantity):
            return Quantity((result.number * other.number), result.unit._mul_with_concat(other.unit))
        else:
            return Quantity(result.number, result.unit._mul_with_concat(other.unit))
        