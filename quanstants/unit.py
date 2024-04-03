from abc import ABC, abstractmethod
from collections import namedtuple
from decimal import Decimal as dec

from .quantity import Quantity
from .unicode import generate_superscript


# Dictionary to contain all the units, making them useable with unit.m notation
# To avoid long import times will probably need a different solution later
unit_reg = {}

# Create a named tuple that is used to hold a unit with its exponent
Factor = namedtuple("Factor", ["unit", "exponent"])

# Function to turn an iterable of Factors into a symbol
def generate_symbol(*components):
    # Create symbol as concatenation of symbols of components, with spaces
    symbol = ""
    for index, factor in enumerate(components):
        if index != 0:
            symbol += " "
        symbol += factor.unit.symbol
        symbol += generate_superscript(factor.exponent)
    return symbol


class Unit(ABC):
    """Parent class for all units."""
    def __init__(
        self,
        symbol,
        name,
        dimension,
        components: tuple,
        canon_symbol=False,
        alt_names=None,
        in_base=None,
        ):
        self.symbol = symbol
        self.name = name
        self.dimension = dimension
        self.components = components
        self.alt_names = alt_names
        # Add to dictionary to allow lookup under the provided name
        unit_reg[self.name] = self
        # Also add under any alternative names e.g. meter vs metre
        if self.alt_names is not None:
            for alt_name in self.alt_names:
                unit_reg[alt_name] = self
        # Also add under the symbol if it has been indicated via canon_symbol
        # that the symbol should uniquely refer to this unit
        if canon_symbol:
            unit_reg[self.symbol] = self
        # If not specified, determine whether the unit is expressed in terms of base units
        if in_base:
            self._isinbase = True
        else:
            self._isinbase = True
            for component in self.components:
                self._isinbase *= component.unit.isinbase()

    def __str__(self):
        return f"Unit({self.symbol})"

    # Define logic for arithmetic operators, used for unit creation

    # Only allow num * Unit, not Unit * num
    # Quantity * Unit is defined by Quantity.__mul__()
    def __rmul__(self, other):
        if isinstance(other, (int, float, dec)):
            return Quantity(other, self)
        else:
            return NotImplemented

    # Just define for Unit * otherUnit, not Unit * num
    def __mul__(self, other):
        if isinstance(other, Unit):
            return CompoundUnit(self.components + other.components)
        else:
            return NotImplemented

    # Only allow num / Unit, not Unit / num
    # Quantity / Unit is defined by Quantity.__div__()
    def __rdiv__(self, other):
        if isinstance(other, (int, float, dec)):
            return Quantity(other, self.inverse())
        else:
            return NotImplemented
        
    # Just define for Unit / otherUnit, nothing else
    def __div__(self, other):
        if isinstance(other, Unit):
            return CompoundUnit(self.components + other.inverse().components)
        else:
            return NotImplemented

    def __pow__(self, other):
        if other == 1:
            return self
        elif isinstance(other, int):
            new_components = (Factor(component.unit, component.exponent * other) for component in self.components)
            return CompoundUnit(new_components)
        else:
            return NotImplemented

    def inverse(self):
        """Return the inverse of the unit as a CompoundUnit."""
        # For now just reuse the __pow__ function
        return self**-1

    def isinbase(self):
        return self._isinbase

    # Some abstract methods that subclasses need to define
    @abstractmethod
    def base(self):
        """Return the unit's value in base units as a Quantity."""
        pass


class BaseUnit(Unit):
    """SI base units and other base units that are not defined in terms of other units."""
    def __init__(
        self,
        symbol,
        name,
        dimension,
        alt_names=None,
        ):
        # All base units will be the canonical unit for that symbol
        super().__init__(
            symbol,
            name,
            dimension,
            components=(Factor(self, 1),),
            canon_symbol=True,
            alt_names=alt_names,
            in_base=True,
            )
    
    def base(self):
        """Return the unit's value in base units as a Quantity."""
        return Quantity(1, self)


class CompoundUnit(Unit):
    """An effective unit created through multiplication of non-compound units.
    
    Constituent units must be passed as a tuple of Factors, where each Factor
    is a namedtuple of a unit and an exponent.
    """
    def __init__(self, *components):
        # Generate a symbol
        symbol = generate_symbol(*components)
        # Don't define a name etc., just a symbol and the components
        super().__init__(
            symbol=symbol,
            name=None,
            dimension=None,
            components=components,
            )
    
    def base(self):
        """Return the unit's value in base units as a Quantity."""
        if self._isinbase():
            return Quantity(1, self)
        else:
            result = 1
            for component in self.components:
                # This will continue down iteratively within each component
                result *= (component.unit.base() ** component.exponent)
            return result


class DerivedUnit(Unit):
    """Units derived from and defined with SI units.
    
    value is a Quantity with both a number and a Unit.
    If symbol=None, the symbol will be the Unit of the value.
    """
    def __init__(
        self,
        symbol,
        name,
        value,
        canon_symbol=False,
        alt_names=None,
        ):
        self.value = value
        # If no custom symbol has been specified, use the one from the definition
        if symbol is None:
            symbol = self.value.unit.symbol
        super().__init__(
            symbol,
            name,
            dimension=None, # for now
            components=(Factor(self, 1),),
            canon_symbol=canon_symbol,
            alt_names=alt_names,
            )

    def base(self):
        """Return the unit's value in base units as a Quantity."""
        return self.value.base()
