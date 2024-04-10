from decimal import Decimal as dec

from .quantity import Quantity


class ConstantAlreadyDefinedError(Exception):
    pass

# Namespace class to contain all the constants, making them useable with constant.c notation
class ConstantReg:
    def add(self, name, constant):
        if hasattr(self, name):
            raise ConstantAlreadyDefinedError
        setattr(self, name, constant)

constant_reg = ConstantReg()

class Constant(Quantity):
    def __init__(
        self,
        symbol: str | None,
        name: str,
        value: Quantity,
        canon_symbol: str = False,
        alt_names: list = None,
    ):
        self.symbol = symbol
        self.name = name
        self.value = value
        self.alt_names = alt_names
        super().__init__(
            value.number,
            value.unit,
            uncertainty=value.uncertainty,
        )
        # Add to registry to allow lookup under the provided name
        constant_reg.add(self.name, self)
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
                constant_reg.add(alt_name, self)
        # Also add under the symbol if it has been indicated via canon_symbol
        # that the symbol should uniquely refer to this constant
        if canon_symbol:
            constant_reg.add(self.symbol, self)
    
    def __str__(self):
        return f"Constant({self.name} = {self.value})"
