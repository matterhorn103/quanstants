from decimal import Decimal as dec

from .config import quanfig
from .uncertainties import get_uncertainty
from .unit import LinearUnit, BaseUnit
from .quantity import Quantity


# Define the most important unit of temperature, though as a BaseUnit not a TemperatureUnit
kelvin = BaseUnit("K", "kelvin", dimension="Θ")


class NotATemperatureError(Exception):
    pass


class TemperatureUnit(LinearUnit):
    """A unit of temperature on a relative rather than absolute scale.

    `degree_value` is the size of the unit itself.
    `zero_point` is the value of 0° on this scale.
    Both should ideally be given as a numerical value with a type parseable by `Decimal`
    (including `str`) - in which case the unit is assumed to be kelvin - or, optionally,
    as a `Quantity`.

    As is always the case with relative temperatures, be careful of the trap presented
    by the double meaning of "20 °C" or "50 °F" arising from a single unit being used
    for both relative and absolute temperatures. In this package, absolute temperatures
    are always represented by `Quantity` objects, while relative temperatures on a scale
    are represented by `Temperature` objects.

    To create a _relative_ `Temperature`, i.e. a point on the respective temperature
    scale, use the `@` operator with a number on the left and the `TemperatureUnit` on
    the right, in a similar fashion to multiplication. The `@` can be thought of as
    representing "on" (the scale).

    Of key importance is that multiplication of a number and a `TemperatureUnit` will,
    as with any other unit, create a normal `Quantity` representing a multiple of the
    unit - in this case representing something like a temperature difference, not an actual temperature on the respective scale.
    ```
    """

    __slots__ = ("_degree_value", "_zero_point")

    def __init__(
        self,
        symbol: str,
        name: str,
        degree_value: str | int | float | dec | Quantity,
        zero_point: str | int | float | dec | Quantity,
        alt_names: list | None = None,
        add_to_namespace: bool = False,
        canon_symbol: bool = False,
    ):
        if isinstance(degree_value, Quantity) and (degree_value.base().unit == kelvin):
            self._degree_value = degree_value.base()
        else:
            self._degree_value = Quantity(degree_value, kelvin)
        if isinstance(zero_point, Quantity) and (zero_point.base().unit == kelvin):
            self._zero_point = zero_point.base()
        else:
            self._zero_point = Quantity(zero_point, kelvin)
        super().__init__(
            symbol=symbol,
            name=name,
            components=((self, 1),),
            value=self._degree_value,
            dimension="Θ",
            alt_names=alt_names,
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
        )
        self._value_base = self._degree_value

    # Not strictly necessary to override `super().value()` but aids clarity
    @property
    def value(self) -> Quantity:
        """Return the value of a degree in this scale, i.e. (x+1)° - (x)°, in kelvin."""
        return self._degree_value

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

    def from_absolute(self, other: Quantity):
        """Convert an absolute `Quantity` to a relative `Temperature` on this scale.

        When `Quantity.on_scale()` is called on a quantity and the target unit is an
        instance of `TemperatureUnit`, this method of the target unit will be called.
        """
        return Temperature.from_absolute(self, other)

    def cancel(self):
        """Combine any like terms and return as a Quantity."""
        raise NotImplementedError

    def canonical(self):
        """Order terms into a reproducible order and return as a Quantity."""
        return 1 * self


class Temperature(Quantity):
    """Represents relative temperatures on a scale."""

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

    # For addition and subtraction, convert to quantities in kelvin to do the maths as
    # then the uncertainty will be taken care of by `Quantity`'s dunder methods
    # Then convert back to a Temperature but only if appropriate
    def __add__(self, other, correlation=0):
        # Allow the addition of non-kelvin temperatures but return in kelvin to make it clear that
        # two absolute temperatures have been summed
        if isinstance(other, Temperature):
            return self.to_absolute().__add__(
                other.to_absolute(), correlation=correlation
            )
        # Allow Quantity with dimension of temperature to be added to Temperature
        elif isinstance(other, Quantity):
            if isinstance(other.unit, TemperatureUnit) or other.unit == kelvin:
                result = (
                    self.to_absolute().__add__(other.to(kelvin), correlation=correlation)
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
                self.to_absolute().__sub__(other.to(kelvin), correlation=correlation)
            ).to(self.unit)
        # Allow Quantity with dimension of temperature to be subtracted from Temperature
        elif isinstance(other, Quantity):
            if isinstance(other.unit, TemperatureUnit) or other.unit == kelvin:
                result = (
                    self.to_absolute().__sub__(other.to(kelvin), correlation=correlation)
                ).on_scale(self.unit)
            else:
                raise NotATemperatureError(
                    f"Can't subtract quantity in {other.unit} from temperature in {self.unit}."
                )
        else:
            return NotImplemented
        # Have to round if precision has increased due to conversion to kelvin
        if result.precision() < self.precision() and result.precision() < other.precision():
            if other.precision() < self.precision():
                return result.round_to_precision_of(other)
            else:
                return result.round_to_precision_of(self)
        else:
            return result
        

    def __neg__(self):
        return Temperature(-1 * self.number, self.unit, self._uncertainty)

    # For other mathematical operations, convert to kelvin and leave in kelvin
    def __mul__(self, other):
        return self.to_absolute() * other

    def __rmul__(self, other):
        return other * self.to_absolute()

    def __truediv__(self, other):
        return self.to_absolute() / other

    def __rtruediv__(self, other):
        return other / self.to_absolute()

    def __pow__(self, other):
        return self.to_absolute() ** other

    # Equality functions of `Quantity` call `.base()` anyway, so super handles fine

    @classmethod
    def from_absolute(cls, unit: TemperatureUnit, quantity: Quantity):
        """Convert an absolute `Quantity` to a relative `Temperature` on the given scale."""
        if quantity.unit == kelvin:
            new_number = (quantity.number / unit.value.number) - unit.zero_point.number
        elif quantity.unit.dimensional_exponents == {
            "T": 0,
            "L": 0,
            "M": 0,
            "I": 0,
            "Θ": 1,
            "N": 0,
            "J": 0,
        }:
            new_number = (quantity.base().number / unit.value.number) - unit.zero_point.number
        else:
            raise NotATemperatureError(
                "Temperatures can only be converted from quantities with units of temperature."
            )

        if not quantity._uncertainty:
            new_uncertainty = dec("0")
        else:
            new_uncertainty = quantity.uncertainty.to(unit)
        # Make sure precision isn't ridiculous for an otherwise exact conversion
        if str(new_number)[-5:] == "00000":
            new_number = dec(str(float(new_number)))
            if str(new_number)[-2:] == ".0":
                new_number = dec(int(new_number))
        return cls(new_number, unit, new_uncertainty)

    def to_absolute(self):
        """Return the temperature as a normal `Quantity` object with units of kelvin.

        This is only necessary for use by the arithmetic functions below and does not need to be in the
        public API - `Temperature.base()` is essentially an alias, and users can call
        `Temperature.to(kelvin)` without an issue, as it goes via conversion of the temperature to kelvin
        with `Temperature.base()`.
        """
        new_number = (self.number + self.unit.zero_point.number) * self.unit.value.number
        if not self._uncertainty:
            return Quantity(new_number, kelvin)
        else:
            return Quantity(
                new_number,
                kelvin,
                self.uncertainty.to(kelvin),
            )

    def with_uncertainty(self, uncertainty):
        return Temperature(self.number, self.unit, uncertainty)

    def cancel(self):
        """Combine any like terms in the unit.

        Has no effect for a `Temperature`.
        """
        return self

    def fully_cancel(self):
        """Combine any terms of the same dimension in the unit.

        Has no effect for a `Temperature`.
        """
        return self

    def canonical(self):
        """Express with its units in a canonical order.

        Has no effect for a `Temperature`.
        """
        return self
    
    def base(self):
        """Return the temperature as a `Quantity` expressed in kelvin."""
        return self.to_absolute()

    def to(self, other):
        return self.to_absolute().to(other)

    def on_scale(self, other):
        return self.to_absolute().on_scale(other)
