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
        self._symbol = symbol
        self._name = name
        self._value = value
        # Any constant named "<x> constant" automatically gets "<x>"" as an alt name
        if name[-9:] == "_constant":
            if alt_names is None:
                alt_names = [name[:-9]]
            else:
                alt_names.append(name[:-9])
        self._alt_names = alt_names
        super().__init__(
            value.number,
            value.unit,
            uncertainty=value.uncertainty,
        )
        self.add_to_reg(add_symbol=canon_symbol)
    
    @property
    def symbol(self):
        return self._symbol
    
    @property
    def name(self):
        return self._name
    
    @property
    def value(self):
        return self._value
    
    @property
    def alt_names(self):
        return self._alt_names
    
    def __str__(self):
        return f"Constant({self.name} = {self.value})"
    
    def add_to_reg(self, add_symbol=False):
        # Add to registry to allow lookup under the provided name
        constant_reg.add(self.name, self)
        
        # Also add under any alternative names (though try to stick to one, as
        # there could be a hundred permutations of each constant's name)
        if self.alt_names is not None:
            for alt_name in self.alt_names:
                constant_reg.add(alt_name, self)
        # Also add under the symbol if it has been indicated via canon_symbol
        # that the symbol should uniquely refer to this constant
        if add_symbol:
            constant_reg.add(self.symbol, self)
