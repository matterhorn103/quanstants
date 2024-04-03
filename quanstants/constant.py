from decimal import Decimal as dec

from .quantity import Quantity

# Dictionary to contain all the constants, making them useable with constant.c notation
# To avoid long import times will probably need a different solution later
constant_reg = {}

class Constant(Quantity):
    def __init__(
        self,
        symbol,
        name,
        value,
        canon_symbol=False,
        alt_names=None,
        ):
        self.symbol = symbol
        self.name = name
        self.value = value
        self.alt_names = alt_names
        super().__init__(
            value.number,
            value.unit,
            )
        # Add to dictionary to allow lookup under the provided name
        constant_reg[self.name] = self
        # Any constant named "<x> constant" automatically gets "<x>"" as an alt name
        if self.name[-9:] == "_constant":
            if self.alt_names is None:
                self.alt_names = [self.name[:-9]]
            else:
                self.alt_names.append(self.name[:-9])
        # Also add under any alternative names (though try to stick to one, as
        # there could be a hundred permutations of each constant's name)
        if self.alt_names is not None:
            for alt_name in self.alt_names:
                constant_reg[alt_name] = self
        # Also add under the symbol if it has been indicated via canon_symbol
        # that the symbol should uniquely refer to this constant
        if canon_symbol:
            constant_reg[self.symbol] = self
    
    def __str__(self):
        return f"Constant({self.name} = {self.value})"
