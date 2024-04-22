from decimal import Decimal as dec
import types

from .unit import Unit, Factor
from .quantity import Quantity
from .si import *


# Define the most important unit of temperature, though as a BaseUnit not a TemperatureUnit
kelvin = BaseUnit("K", "kelvin", dimension="Θ")


class NotATemperatureError(Exception):
    pass


class TemperatureUnit(Unit):
    """A unit of temperature with a potentially non-linear relationship to the kelvin scale.
    
    `formula_from_kelvin` should be an expression for the temperature in the unit as a function
    of the temperature in kelvin, which should be represented as the letter T.
    `formula_to_kelvin` should be the inverse of `formula_from_kelvin`.
    """
    def __init__(
        self,
        symbol: str,
        name: str,
        formula_from_kelvin: types.LambdaType,
        formula_to_kelvin: types.LambdaType,
        add_to_reg: bool = True,
        canon_symbol: bool = False,
        alt_names: list | None = None,
    ):
        self._from_kelvin = formula_from_kelvin
        self._to_kelvin = formula_to_kelvin
        super().__init__(
            symbol=symbol,
            name=name,
            components=(Factor(self, 1),),
            dimension="Θ",
            add_to_reg=add_to_reg,
            canon_symbol=canon_symbol,
            alt_names=alt_names,
        )
    
    def from_temperature(self, other: Quantity):
        """Convert another quantity with temperature units to a Temperature with this unit.
        
        When `Quantity.to()` is called on a quantity and the target unit is an instance of
        `TemperatureUnit`, this method of the target unit is called instead.

        """
        if other.unit == kelvin:
            new_number = self._from_kelvin(other.number)
            if other.uncertainty == "(exact)":
                new_uncertainty = "(exact)"
            else:
                new_uncertainty = self._from_kelvin(other.uncertainty)
        elif isinstance(other, Temperature):
            new_number = self._from_kelvin(other.unit._to_kelvin(other.number))
            if other.uncertainty == "(exact)":
                new_uncertainty = "(exact)"
            else:
                new_uncertainty = self._from_kelvin(other.unit._to_kelvin(other.uncertainty))
        else:
            raise NotATemperatureError("Temperatures")
        return Temperature(new_number, self, new_uncertainty)
    
    def base(self):
        """Return the unit's value in base units as a Quantity."""
        raise NotImplementedError

    def cancel(self):
        """Combine any like terms and return as a Quantity."""
        raise NotImplementedError
    
    def fully_cancel(self):
        """Combine any like terms and return as a Quantity, with units of the same dimension converted and also combined."""
        return self.cancel()
    
    def canonical(self):
        """Order terms into a reproducible order and return as a Unit."""
        raise NotImplementedError


class Temperature(Quantity):
    """Quantities representing temperatures where the unit is one other than kelvin.
    
    As most scales have a different zero point, a temperature is first converted internally to kelvin
    before it can be manipulated mathematically.
    """
    def __init__(
        self,
        number: str | int | float | dec,
        unit: TemperatureUnit,
        uncertainty: str | int | float | dec | None = None,
    ):
        super().__init__(
            number,
            unit,
            uncertainty,
        )
    
    def _to_kelvin(self):
        """Return the temperature as a normal `Quantity` object with units of kelvin.
        
        This is only necessary for use by the arithmetic functions below and does not need to be in the
        public API - `Temperature.base()` is essentially an alias, and users can call
        `Temperature.to(kelvin)` without an issue, as it goes via conversion of the temperature to kelvin
        with `Temperature.base()`.
        """
        if self.uncertainty == "(exact)":
            return Quantity(self.unit._to_kelvin(self.number), kelvin)
        else:
            return Quantity(
                self.unit._to_kelvin(self.number),
                kelvin,
                self.unit._to_kelvin(self.uncertainty),
            )

    # Addition and subtraction work fine as defined in super()
    # For other mathematical functions, need to first convert to kelvin

    def __mul__(self, other):
        return super().__mul__(self._to_kelvin(), other)
    
    def __rmul__(self, other):
        return super().__rmul__(self._to_kelvin(), other)
    
    def __truediv__(self, other):
        return super().__truediv__(self._to_kelvin(), other)
    
    def __rtruediv__(self, other):
        return super().__rtruediv__(self._to_kelvin(), other)
    
    def __pow__(self, other):
        return super().__pow__(self._to_kelvin(), other)
    
    def __eq__(self, other):
        return super().__eq__(self._to_kelvin(), other)
    
    def __gt__(self, other):
        return super().__gt__(self._to_kelvin(), other)

    def __ge__(self, other):
        return super().__ge__(self._to_kelvin(), other)

    def base(self):
        """Return the quantity expressed in terms of base units."""
        return self._to_kelvin()
    
    def cancel(self):
        """Combine any like terms in the unit."""
        return self
    
    def fully_cancel(self):
        """Combine any like terms in the unit, with units of the same dimension converted and combined."""
        return self
    
    def canonical(self):
        """Express the quantity with its units in a canonical order."""
        return self


degreeCelsius = TemperatureUnit(
    "°C",
    "degreeCelsius",
    lambda T: T - dec("273.15"),
    lambda T: T + dec("273.15"),
    canon_symbol=True,
    alt_names=["degree_Celsius", "degreeC", "celsius", "degreeCentigrade", "degree_Centigrade", "centigrade"]
    )