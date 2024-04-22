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
        if other.unit == kelvin:
            new_number = self._from_kelvin(other.number)
            new_uncertainty = None if other.uncertainty is None else self._from_kelvin(other.uncertainty)
        elif isinstance(other, Temperature):
            new_number = self._from_kelvin(other.unit._to_kelvin(other.number))
            new_uncertainty = None if other.uncertainty is None else self._from_kelvin(other.unit._to_kelvin(other.uncertainty))
        else:
            raise NotATemperatureError("Temperatures")
        return Temperature(new_number, self)


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
    


degreeCelsius = TemperatureUnit(
    "°C",
    "degreeCelsius",
    lambda T: T - dec("273.15"),
    lambda T: T + dec("273.15"),
    canon_symbol=True,
    alt_names=["degree_Celsius", "degreeC", "celsius", "degreeCentigrade", "degree_Centigrade", "centigrade"]
    )