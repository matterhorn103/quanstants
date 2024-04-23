from decimal import Decimal as dec

from .quantity import Quantity
from .unit import DerivedUnit


class ConstantAlreadyDefinedError(Exception):
    pass

# Namespace class to contain all the constants, making them useable with constant.c notation
class ConstantReg:
    def add(self, name: str, constant):
        """Add a `Constant` object to the registry under the provided name.
        
        This method provides a safe way to add constants to the registry.
        Names in the registry's namespace cannot be overwritten in this way, and attempting to add a
        constant under a name that is already defined will raise a `ConstantAlreadyDefinedError`.
        If it is necessary to redefine a name, do it by setting the attribute in the normal way i.e.
        `ConstantReg.already_assigned_name = new_value`.
        """
        if hasattr(self, name):
            raise ConstantAlreadyDefinedError
        setattr(self, name, constant)
    
    def list_names(self):
        """Return a list of all constant names in the namespace, in human-readable format i.e. as strings.
        
        Essentially just return the value of `self.__dict__.keys()` but with anything that isn't a constant
        filtered out.
        Note that a) the values returned are variable names as strings, not the `Constant` objects
        themselves and b) a constant is typically listed multiple times under different names, as well as
        under its symbol if it has `canon_symbol=True`.
        """
        filtered_names = {name for name in self.__dict__.keys() if name[0] != "_"}
        return list(filtered_names)
    
    def list_constants(self):
        """Return a list of all `Constant` objects currently in the registry.
        
        Unlike `list_names()`, the values are `Constant` objects not strings, and each constant is only
        listed once, regardless of how many names it is registered under in the registry.
        """
        filtered_names = self.list_names()
        # Using the set approach doesn't work because Constants are currently unhashable - can this be fixed?
        # unique_constants = {self.__dict__[name] for name in filtered_names}
        # return list(unique_constants)
        unique_constants = []
        for name in filtered_names:
            if self.__dict__[name] not in unique_constants:
                unique_constants.append(self.__dict__[name])
        return unique_constants

# Instantiate the main constant registry, which all constants will be added to by default
constant_reg = ConstantReg()


class Constant(Quantity):
    """A class that represents defined, named physical quantities, with a fixed value determined by experiment.
    
    Essentially a `Quantity` object with a symbol and/or name, and in most respects behaves identically.

    Like a `Unit`, requires either a `symbol` or a `name` to be provided. If `symbol` is not provided,
    it will be set to the value of `name`, so that all constants have a symbolic representation that is
    used in printed representations.
    `name` must contain only ASCII letters and digits, and underscores. It must in addition be a valid
    Python identifier, so it cannot start with a digit.
    `symbol` may be any Unicode string.

    Like a `Quantity`, requires either a number and unit to be provided, with an optional uncertainty,
    or a value containing all two or three.
    See the docs for `Quantity` for more details on acceptable input for these parameters.

    `add_to_reg` specifies whether the constant should be added to `reg` as an attribute under the
    provided `name` and under any alternative names given as a list as `alt_names`.
    If `canon_symbol` is set to `True`, the constant will also be added to `reg` under `symbol`.
    The default constant registry is accessible as `quanstants.constants`.

    Constants possess the convenient additional method `.as_unit()`, which returns a `DerivedUnit` with
    the same symbol and value as the constant. This allows the creation of quantities like:
    `300 * (qu.MeV / qc.c.as_unit()**2)`
    """
    def __init__(
        self,
        symbol: str | None,
        name: str | None,
        number: str | int | float | dec | None = None,
        unit = None,
        uncertainty: str | int | float | dec | None = None,
        value: str | Quantity | None = None,
        add_to_reg: bool = True,
        reg: ConstantReg = constant_reg,
        canon_symbol: str = False,
        alt_names: list = None,
    ):
        if symbol is not None:
            self._symbol = symbol
        elif name is not None:
            self._symbol = name
            # Symbol can't be canon if it wasn't even provided
            canon_symbol = False
        else:
            raise RuntimeError("Either a symbol or a name must be provided!")
        self._name = name
        # Any constant named "<x> constant" automatically gets "<x>"" as an alt name
        if name is not None:
            if name[-9:] == "_constant":
                if alt_names is None:
                    alt_names = [name[:-9]]
                else:
                    alt_names.append(name[:-9])
        self._alt_names = alt_names
        super().__init__(
            number,
            unit,
            uncertainty,
            value,
        )
        if add_to_reg:
            self.add_to_reg(reg=reg, add_symbol=canon_symbol)
    
    @property
    def symbol(self):
        return self._symbol
    
    @property
    def name(self):
        return self._name
    
    @property
    def value(self):
        return Quantity(self.number, self.unit, self.uncertainty)
    
    @property
    def alt_names(self):
        return self._alt_names
    
    def __repr__(self):
        return f"Constant({self.__str__()})"

    def __str__(self):
        value_as_string = super().__str__()
        if self.name is not None:
            return f"{self.name} = {value_as_string}"
        else:
            return f"{self.symbol} = {value_as_string}"
    
    def add_to_reg(self, reg: ConstantReg = constant_reg, add_symbol=False):
        # Add to specified registry to allow lookup under the provided name
        if self.name is not None:
            reg.add(self.name, self)
        # Also add under any alternative names (though try to stick to one, as
        # there could be a hundred permutations of each constant's name)
        if self.alt_names is not None:
            for alt_name in self.alt_names:
                reg.add(alt_name, self)
        # Also add under the symbol if it has been indicated via canon_symbol
        # that the symbol should uniquely refer to this constant
        if (add_symbol) and (self.symbol != self.name):
            reg.add(self.symbol, self)
    
    def as_unit(self):
        """Return a `DerivedUnit` with the same value and symbol as the constant."""
        constant_as_unit = DerivedUnit(
            self.symbol,
            name=None,
            value=self.value,
            add_to_reg=False,
        )
        return constant_as_unit
