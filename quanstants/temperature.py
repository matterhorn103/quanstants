from decimal import Decimal as dec
import types

from .config import quanfig
from .uncertainties import get_uncertainty
from .unit import Unit, Factor
from .unitreg import unit_reg, UnitReg
from .quantity import Quantity
from .si import *


# Define the most important unit of temperature, though as a BaseUnit not a TemperatureUnit
kelvin = BaseUnit("K", "kelvin", dimension="Θ")


class NotATemperatureError(Exception):
    pass


class TemperatureUnit(Unit):
    """A unit of temperature with a potentially non-linear relationship to the kelvin scale.

    `degree_value` should be the size of the unit itself.
    `zero_point` should be the value of 0° on this scale.
    Both should ideally be given as a numerical value with a type parseable by `Decimal` (including
    `str`), in which case the unit is assumed to be kelvin, or as a `Quantity`.

    As is always the case with relative temperatures, be careful of the trap presented by the double
    meaning of "20 °C" or "50 °F" arising from a single unit being used for both relative and absolute
    temperatures. In this package, absolute temperatures are always represented by `Quantity` objects,
    while relative temperatures on a scale are represented by `Temperature` objects.

    To create a _relative_ `Temperature`, i.e. a point on the respective temperature scale, use the
    `@` operator with a number on the left and the `TemperatureUnit` on the right, in a similar fashion
    to multiplication. The `@` can be thought of as representing "on" (the scale).

    Of key importance is that multiplication of a number and a `TemperatureUnit` will, as with any other
    unit, create a normal `Quantity` representing a multiple of the unit - in this case representing
    something like a temperature difference, not an actual temperature on the respective scale.

    For example:

    ```python
    >>> from quanstants import units as qu, Quantity
    >>> rel_temp = 50 @ qu.degreeCelsius
    >>> print(rel_temp)
    50 °C
    >>> rel_temp.base()
    Quantity(323.15, K)
    >>> abs_temp = 50 * qu.degreeCelsius
    >>> abs_temp.base()
    Quantity(50, K)
    >>> rel_temp + rel_temp
    Quantity(646.30, K)
    >>> abs_temp + abs_temp
    Quantity(100, °C)
    >>> rel_temp + abs_temp
    Temperature(100, °C)
    ```
    """

    __slots__ = ("_value", "_zero_point")

    def __init__(
        self,
        symbol: str,
        name: str,
        degree_value: str | int | float | dec | Quantity,
        zero_point: str | int | float | dec | Quantity,
        add_to_reg: bool = False,
        reg: UnitReg = unit_reg,
        canon_symbol: bool = False,
        alt_names: list | None = None,
    ):
        if isinstance(degree_value, Quantity) and (degree_value.base().unit == kelvin):
            self._value = degree_value.base()
        else:
            self._value = Quantity(degree_value, kelvin)
        if isinstance(zero_point, Quantity) and (zero_point.base().unit == kelvin):
            self._zero_point = zero_point.base()
        else:
            self._zero_point = Quantity(zero_point, kelvin)
        super().__init__(
            symbol=symbol,
            name=name,
            components=(Factor(self, 1),),
            dimension="Θ",
            add_to_reg=add_to_reg,
            reg=reg,
            canon_symbol=canon_symbol,
            alt_names=alt_names,
        )

    @property
    def value(self) -> Quantity:
        """Return the value of a degree in this scale, i.e. (x+1)° - (x)°, in kelvin."""
        return self._value

    @property
    def zero_point(self) -> Quantity:
        """Return the temperature at 0° on this scale, in kelvin."""
        return self._zero_point

    # (Ab)use the matrix multiplication operator to create a `Temperature`
    def __rmatmul__(self, other):
        if isinstance(other, (str, int, float, dec)):
            return Temperature(other, self)
        else:
            return NotImplemented

    def _to_kelvin(self, number: dec) -> dec:
        """Return the equivalent in kelvin of the specified temperature on this scale."""
        return (number + self.zero_point.number) * self.value.number

    def _from_kelvin(self, number: dec) -> dec:
        """Return the equivalent temperature on this scale of the specified number of kelvin."""
        return (number / self.value.number) - self.zero_point.number

    def from_absolute(self, other: Quantity):
        """Convert an absolute `Quantity` with temperature units to a relative `Temperature` with this unit.

        When `Quantity.on_scale()` is called on a quantity and the target unit is an instance of
        `TemperatureUnit`, this method of the target unit will be called.
        """
        if other.unit == kelvin:
            new_number = self._from_kelvin(other.number)
        elif other.unit.dimensional_exponents == {
            "T": 0,
            "L": 0,
            "M": 0,
            "I": 0,
            "Θ": 1,
            "N": 0,
            "J": 0,
        }:
            new_number = self._from_kelvin(other.base().number)
        else:
            raise NotATemperatureError(
                "Temperatures can only be converted from quantities with units of temperature."
            )

        if not other._uncertainty:
            new_uncertainty = dec("0")
        else:
            new_uncertainty = other.uncertainty.to(self)
        # Make sure precision isn't ridiculous for an otherwise exact conversion
        if str(new_number)[-5:] == "00000":
            new_number = dec(str(float(new_number)))
            if str(new_number)[-2:] == ".0":
                new_number = dec(int(new_number))
        return Temperature(new_number, self, new_uncertainty)

    def base(self):
        """Return the unit's value in base units as a Quantity."""
        return self.value

    def cancel(self):
        """Combine any like terms and return as a Quantity."""
        raise NotImplementedError

    def canonical(self):
        """Order terms into a reproducible order and return as a Quantity."""
        return 1 * self


class Temperature(Quantity):
    """A class representing relative temperatures on a scale rather than an absolute temperature in kelvin.

    As most scales have a different zero point, a temperature is first converted internally to kelvin
    before it can be manipulated mathematically.
    """

    __slots__ = ()

    def __init__(
        self,
        number: str | int | float | dec,
        unit: TemperatureUnit,
        uncertainty: str | int | float | dec | Quantity | None = None,
    ):
        super().__init__(
            number,
            unit,
            uncertainty,
        )

    def __repr__(self):
        as_quantity = super().__repr__()
        return as_quantity.replace("Quantity", "Temperature")

    def _to_kelvin(self):
        """Return the temperature as a normal `Quantity` object with units of kelvin.

        This is only necessary for use by the arithmetic functions below and does not need to be in the
        public API - `Temperature.base()` is essentially an alias, and users can call
        `Temperature.to(kelvin)` without an issue, as it goes via conversion of the temperature to kelvin
        with `Temperature.base()`.
        """
        if not self._uncertainty:
            return Quantity(self.unit._to_kelvin(self.number), kelvin)
        else:
            return Quantity(
                self.unit._to_kelvin(self.number),
                kelvin,
                self.uncertainty.to(kelvin),
            )

    # For addition and subtraction, convert to quantities in kelvin to do the maths as
    # then the uncertainty will be taken care of by `Quantity`'s dunder methods
    # Then convert back to a Temperature but only if appropriate
    def __add__(self, other, correlation=0):
        # Allow the addition of non-kelvin temperatures but return in kelvin to make it clear that
        # two absolute temperatures have been summed
        if isinstance(other, Temperature):
            return self._to_kelvin().__add__(
                other._to_kelvin(), correlation=correlation
            )
        # Allow Quantity with dimension of temperature to be added to Temperature
        elif isinstance(other, Quantity):
            if isinstance(other.unit, TemperatureUnit) or other.unit == kelvin:
                result = (
                    self._to_kelvin().__add__(other.to(kelvin), correlation=correlation)
                ).on_scale(self.unit)
                # Have to round if precision has increased due to conversion to kelvin
                if result.precision() < self.precision() and result.precision() < other.precision():
                    if other.precision() < self.precision():
                        return result.round_to_precision_of(other)
                    else:
                        return result.round_to_precision_of(self)
                else:
                    return result
            else:
                raise NotATemperatureError(
                    f"Can't add quantity in {other.unit} to temperature in {self.unit}."
                )
        else:
            return NotImplemented

    def __sub__(self, other, correlation=0):
        # Allow finding the difference between two temperatures
        # Unlike for __add__(), it is clear to the user here that a temperature difference has been
        # calculated, not a temperature
        if isinstance(other, Temperature):
            result = (
                self._to_kelvin().__sub__(other.to(kelvin), correlation=correlation)
            ).to(self.unit)
            # Have to round if precision has increased due to conversion to kelvin
            if result.precision() < self.precision() and result.precision() < other.precision():
                if other.precision() < self.precision():
                    return result.round_to_precision_of(other)
                else:
                    return result.round_to_precision_of(self)
            else:
                return result
        # Allow Quantity with dimension of temperature to be subtracted from Temperature
        elif isinstance(other, Quantity):
            if isinstance(other.unit, TemperatureUnit) or other.unit == kelvin:
                result = (
                    self._to_kelvin().__sub__(other.to(kelvin), correlation=correlation)
                ).on_scale(self.unit)
                # Have to round if precision has increased due to conversion to kelvin
                if result.precision() < self.precision() and result.precision() < other.precision():
                    if other.precision() < self.precision():
                        return result.round_to_precision_of(other)
                    else:
                        return result.round_to_precision_of(self)
                else:
                    return result
            else:
                raise NotATemperatureError(
                    f"Can't subtract quantity in {other.unit} from temperature in {self.unit}."
                )
        else:
            return NotImplemented

    def __neg__(self):
        return Temperature(-1 * self.number, self.unit, self._uncertainty)

    # For other mathematical operations, convert to kelvin and leave in kelvin
    def __mul__(self, other):
        return self._to_kelvin() * other

    def __rmul__(self, other):
        return other * self._to_kelvin()

    def __truediv__(self, other):
        return self._to_kelvin() / other

    def __rtruediv__(self, other):
        return other / self._to_kelvin()

    def __pow__(self, other):
        return self._to_kelvin() ** other

    # Equality functions of `Quantity` call `.base()` anyway, so are handled fine by super

    def with_uncertainty(self, uncertainty):
        """Return a new quantity with the provided uncertainty."""
        return Temperature(self.number, self.unit, uncertainty)

    def base(self):
        """Return the temperature as a Quantity expressed in kelvin."""
        return self._to_kelvin()

    def cancel(self):
        """Combine any like terms in the unit.

        Has no effect for a Temperature.
        """
        return self

    def fully_cancel(self):
        """Combine any like terms in the unit, with units of the same dimension converted and combined.

        Has no effect for a Temperature.
        """
        return self

    def canonical(self):
        """Express with its units in a canonical order.

        Has no effect for a Temperature.
        """
        return self

    def to(self, other):
        return self._to_kelvin().to(other)

    def on_scale(self, other):
        return self._to_kelvin().on_scale(other)
