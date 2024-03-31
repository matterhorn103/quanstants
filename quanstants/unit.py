from decimal import Decimal as dec

from .quantity import Quantity

# Dictionary to contain all the units, making them useable with unit.m notation
# To avoid long import times will probably need a different solution later
unit_reg = {}


class Unit:
    """Parent class for all units."""
    def __init__(
        self,
        symbol,
        name,
        dimension,
        components: tuple,
        canon_symbol=False,
        alt_names=None,
        ):
        self.symbol = symbol
        self.name = name
        self.dimension = dimension
        self.components = components
        self.alt_names = alt_names
        # Add to dictionary to allow lookup under the provided name
        unit_reg[self.name] = self
        # Also add under any alternative names e.g. meter vs metre
        if alt_names:
            for alt_name in alt_names:
                unit_reg[alt_name] = self
        # Also add under the symbol if it has been indicated via canon_symbol
        # that the symbol should uniquely refer to this unit
        if canon_symbol == True:
            unit_reg[self.symbol] = self

    def __str__(self):
        return f"Unit({self.symbol})"

    # Define logic for arithmetic operators, used for unit creation
    # Only allow num * Unit, not Unit * num, so use rmul
    # Quantity * Unit is defined by Quantity.__mul__()
    def __rmul__(self, other):
        if not isinstance(other, (int, float, dec)):
            return NotImplemented
        else:
            return Quantity(other, self)

    # Just define for Unit * Unit, nothing else
    def __mul__(self, other):
        if not isinstance(other, Unit):
            return NotImplemented
        else:
            return CompoundUnit(self.components + other.components)

    def __rdiv__(self, other):
        pass

    def __pow__(self, other):
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
            components=(self,),
            canon_symbol=True,
            alt_names=alt_names,
            )
    
    def base(self):
        return self
        

class DerivedUnit(Unit):
    """Units derived from and defined with SI units."""
    def __init__(
        self,
        symbol,
        name,
        value,
        canon_symbol=False,
        alt_names=None,
        ):
        super().__init__(
            symbol,
            name,
            dimension=None, # for now
            components=(self,),
            canon_symbol=canon_symbol,
            alt_names=alt_names,
            )
        self.value = value

    def base(self):
        return self.value.unit


class CompoundUnit(Unit):
    """An effective unit created through multiplication of non-compound units."""
    def __init__(self, *components):
        # Create symbol as concatenation of symbols of components, with spaces
        self.symbol = ""
        for index, component in enumerate(components):
            if index != 0:
                self.symbol += " "
            self.symbol += component.symbol
        # Don't define a name etc., just a symbol and the components
        super().__init__(components=components)