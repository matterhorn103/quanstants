from decimal import Decimal as dec

from ..quantities import Quantity


# Dictionary to contain all the units, making them useable with unit.m notation
# To avoid long import times will probably need a different solution later
unit_reg = {}

class Unit:
    def __init__(self, symbol, name, dimension, canon_symbol=False, alt_names=None):
        self.symbol = symbol
        self.name = name
        self.alt_names = None
        self.dimension = dimension
        # Add to dictionary to allow lookup under the provided name
        unit_reg[self.name] = self
        # Also add under any alternative names e.g. meter vs metre
        if alt_names:
            for alt_name in alt_names:
                unit_reg[alt_name] = self
        # Also add under the symbol, but only if it has been indicated
        # that the symbol should uniquely refer to this unit
        if canon_symbol == True:
            unit_reg[self.symbol] = self
    
    def __repr__(self):
        return f"quanstants.units.{self.name}"

    def __str__(self):
        return f"Unit({self.name})"


class BaseUnit(Unit):
    def __init__(self, symbol, name, dimension, alt_names=None):
        # All base units will be the canonical unit for that symbol
        super().__init__(symbol, name, dimension, canon_symbol=True, alt_names=alt_names)

    def value(self):
        return Quantity(1, self)


class DerivedUnit(Unit):
    def __init__(self, symbol, name, dimension, value, canon_symbol=False, alt_names=None):
        super().__init__(symbol, name, dimension, canon_symbol, alt_names)

        self.value = value
    
    def value(self):
        return self.value