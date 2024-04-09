from abc import ABC, abstractmethod
from collections import namedtuple
from decimal import Decimal as dec
from fractions import Fraction as frac

from .quantity import Quantity
from .unicode import generate_superscript


class UnitAlreadyDefinedError(Exception):
    pass

# Namespace class to contain all the units, making them useable with unit.m notation
class UnitReg:
    def add(self, name, unit):
        if hasattr(self, name):
            raise UnitAlreadyDefinedError(f"{name} is already defined!")
        setattr(self, name, unit)

unit_reg = UnitReg()

# Create a named tuple that is used to hold a unit with its exponent
Factor = namedtuple("Factor", ["unit", "exponent"])

# Function to turn a tuple or other iterable of Factors into a symbol
def generate_symbol(components):
    # Create symbol as concatenation of symbols of components, with spaces
    symbol = ""
    for index, factor in enumerate(components):
        if index != 0:
            symbol += " "
        symbol += factor.unit.symbol
        symbol += generate_superscript(factor.exponent)
    return symbol

# Function to turn a tuple or other iterable of Factors into a dimension
def generate_dimensions(components):
    new_dimensions = {"T": 0, "L": 0, "M": 0, "I": 0, "Θ": 0, "N": 0, "J": 0}
    for factor in components:
        for dimension in new_dimensions.keys():
            if dimension in factor.unit.dimensions:
                new_dimensions[dimension] += factor.unit.dimensions[dimension] * factor.exponent
    return new_dimensions

# Function to allow sorting of compound base units into a canonical order
def base_priority(Factor):
    priorities = {"s": 0, "m": 1, "kg": 2, "A": 3, "K": 4, "mol": 5, "cd": 6, "rad": 7, "sr": 8}
    if Factor.unit.symbol in priorities:
        return priorities[Factor.unit.symbol]
    else:
        return 9


class Unit(ABC):
    """Parent class for all units."""
    def __init__(
        self,
        symbol,
        name,
        components: tuple,
        dimension: str | None = None,
        dimensions: dict = None,
        add_to_reg=False,
        canon_symbol=False,
        alt_names=None,
    ):
        self.symbol = symbol
        self.name = name
        # Start with a dimensionless unit and add any provided ones
        self.dimensions = {"T": 0, "L": 0, "M": 0, "I": 0, "Θ": 0, "N": 0, "J": 0}
        if dimension == "X":
            pass
        elif isinstance(dimension, str) and len(dimension) == 1:
            self.dimensions[dimension] += 1
        else:
            for dim in self.dimensions.keys():
                if dim in dimensions:
                    self.dimensions[dim] += dimensions[dim]
        self.components = components
        self.alt_names = alt_names
        if add_to_reg:
            # Add to registry to allow lookup under a provided name
            if self.name is not None:
                unit_reg.add(self.name, self)
            # Also add under any alternative names e.g. meter vs metre
            if self.alt_names is not None:
                for alt_name in self.alt_names:
                    unit_reg.add(alt_name, self)
            # Also add under the symbol if it has been indicated via canon_symbol
            # that the symbol should uniquely refer to this unit
            if (canon_symbol) and (self.symbol != self.name):
                unit_reg.add(self.symbol, self)

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

    # Just define for Unit * otherUnit or Unit * UnitlessUnit, not Unit * num
    def __mul__(self, other):
        if isinstance(other, Unitless):
            return self
        elif isinstance(other, Unit):
            return CompoundUnit(self.components + other.components)
        else:
            return NotImplemented

    # Only allow num / Unit, not Unit / num
    # Quantity / Unit is defined by Quantity.__div__()
    def __rtruediv__(self, other):
        if isinstance(other, (int, float, dec)):
            return Quantity(other, self.inverse())
        else:
            return NotImplemented
        
    # Just define for Unit / otherUnit and Unit / Unitless
    def __truediv__(self, other):
        if isinstance(other, Unitless):
            return self
        if isinstance(other, Unit):
            return CompoundUnit(self.components + other.inverse().components)
        else:
            return NotImplemented

    # For now only allow integer and fractional exponents
    def __pow__(self, other):
        if other == 1:
            return self
        elif isinstance(other, (int, frac)):
            # Tuple comprehensions don't exist so make a tuple from a generator
            new_components = tuple((Factor(component.unit, component.exponent * other) for component in self.components),)
            return CompoundUnit(new_components)
        else:
            return NotImplemented

    def inverse(self):
        """Return the inverse of the unit as a CompoundUnit."""
        # For now just reuse the __pow__ function
        return self**-1
    
    def dimensionality(self):
        """Return the dimensionality as a nice string."""
        result = ""
        for dimension, exponent in self.dimensions.items():
            if exponent != 0:
                result += dimension
                if exponent != 1:
                    result += generate_superscript(exponent)
        if len(result) > 0:
            return result
        else:
            return "(dimensionless)"
    
    # Some abstract methods that subclasses need to define
    @abstractmethod
    def base(self):
        """Return the unit's value in base units as a Quantity."""
        pass

    @abstractmethod
    def cancel(self):
        """Combine any like terms."""
        pass

    @abstractmethod
    def canonical(self):
        """Order any base terms."""
        pass


class Unitless(Unit):
    """Special unitless unit."""
    def __init__(self, add_to_reg=False):
        super().__init__(
            symbol="(unitless)",
            name="unitless",
            components=(Factor(self, 1),),
            dimension="X",
            add_to_reg=add_to_reg,
            canon_symbol=False,
            alt_names=None,
        )
    
    # Make sure that Unitless * Unit and Unitless / Unit return just the other Unit
    def __mul__(self, other):
        if isinstance(other, Unit):
            return other
        else:
            return NotImplemented
    
    def __truediv__(self, other):
        if isinstance(other, Unit):
            return other.inverse()
        else:
            return NotImplemented

    def base(self):
        """Return unity as a unitless Quantity."""
        return Quantity(1, self)

    def cancel(self):
        return self
    
    def canonical(self):
        return self


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
            components=(Factor(self, 1),),
            dimension=dimension,
            add_to_reg=True,
            canon_symbol=True,
            alt_names=alt_names,
        )
    
    def base(self):
        """Return the unit's value in base units as a Quantity."""
        return Quantity(1, self)
    
    def cancel(self):
        return self
    
    def canonical(self):
        return self


class CompoundUnit(Unit):
    """An effective unit created through multiplication of non-compound units.
    
    Multiple units multiplied together are treated as a single Unit object with
    its constituent parts gathered under `components`.
    Constituent units must be passed as a tuple of `Factors`, a `namedtuple` of
    a unit and an exponent.
    """
    def __init__(self, components, add_to_reg=False):
        # Generate a symbol
        symbol = generate_symbol(components)
        dimensions = generate_dimensions(components)
        # Don't define a name etc., just a symbol and the components
        super().__init__(
            symbol=symbol,
            name=None,
            components=components,
            dimensions=dimensions,
            add_to_reg=add_to_reg,
        )
        # Determine whether the unit is expressed in terms of base units
        self.defined_in_base = True
        for component in self.components:
            self.defined_in_base *= isinstance(component.unit, BaseUnit)
        self.defined_in_base = bool(self.defined_in_base)
    
    def base(self):
        """Return the unit's value in base units as a Quantity."""
        if self.defined_in_base:
            return Quantity(1, self)
        else:
            result = 1
            for component in self.components:
                # This will continue down iteratively within each component
                result *= (component.unit.base() ** component.exponent)
            return result
    
    def cancel(self):
        """Return a new unit with equivalent value but with like terms combined."""
        new_components_list = []
        for factor in self.components:
            unit_in_list = False
            for index, new_factor in enumerate(new_components_list):
                if new_factor.unit.name == factor.unit.name:
                    new_components_list[index] = Factor(factor.unit, factor.exponent + new_factor.exponent)
                    unit_in_list = True
                else:
                    continue
            if not unit_in_list:
                new_components_list.append(factor)
        new_components = tuple(factor for factor in new_components_list if factor.exponent != 0)
        if len(new_components) == 0:
            return Unitless()
        return CompoundUnit(new_components)
    
    def canonical(self):
        ordered_components = tuple(sorted(self.components, key=base_priority))
        return CompoundUnit(ordered_components)


class DerivedUnit(Unit):
    """Units derived from and defined with SI units.
    
    value is a Quantity with both a number and a Unit, and optionally, an uncertainty.
    A symbol must be provided, but a name is optional.
    If a name is given, the unit will be added to the unit registry under that
    name (note that this will replace any existing unit with that name).
    The dimension is set to that of the provided value's unit(s).
    """
    def __init__(
        self,
        symbol,
        name,
        value,
        add_to_reg=True,
        canon_symbol=False,
        alt_names=None,
    ):
        self.value = value
        super().__init__(
            symbol,
            name,
            components=(Factor(self, 1),),
            dimensions=self.value.unit.dimensions,
            add_to_reg=add_to_reg,
            canon_symbol=canon_symbol,
            alt_names=alt_names,
        )

    def base(self):
        """Return the unit's value in base units as a Quantity."""
        return self.value.base()
    
    def cancel(self):
        return self
    
    def canonical(self):
        return self
