from .quantity import Quantity
from .unit import Unit, DerivedUnit

class ConstantAlreadyDefinedError(Exception):
    pass

# Namespace class to contain all the constants, making them useable with constant.c notation
class ConstantReg:
    def __init__(self):
        self.total_names = 0
        self.total_constants = 0
    def add(self, name, constant):
        if hasattr(self, name):
            raise ConstantAlreadyDefinedError
        self.total_names += 1
        if constant not in self.__dict__.values():
            self.total_constants += 1
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
    
    # Override * and / dunder methods of Quantity to allow the creation of units from constants
    def __mul__(self, other):
        if isinstance(other, Unit):
            return self.as_unit() * other
        else:
            return super().__mul__(self, other)
    
    def __rmul__(self, other):
        if isinstance(other, Unit):
            return other * self.as_unit()
        else:
            return super().__rmul__(self, other)
    
    def __truediv__(self, other):
        if isinstance(other, Unit):
            return self.as_unit() / other
        else:
            return super().__truediv__(self, other)
    
    def __rtruediv__(self, other):
        if isinstance(other, Unit):
            return other / self.as_unit()
        else:
            return super().__rtruediv__(self, other)
    
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
    
    def as_unit(self):
        """Return a `DerivedUnit` with the same value and symbol as the constant."""
        constant_as_unit = DerivedUnit(
            self.symbol,
            name=None,
            value=self.value,
            add_to_reg=False,
        )
        return constant_as_unit
