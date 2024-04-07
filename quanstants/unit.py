from abc import ABC, abstractmethod
from collections import namedtuple
from decimal import Decimal as dec

from .quantity import Quantity
from .unicode import generate_superscript


# Namespace class to contain all the units, making them useable with unit.m notation
class UnitReg:
    pass

unit_reg = UnitReg

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
def generate_dimension(components):
    return "X"

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
        ):
        self.symbol = symbol
        self.name = name
        self.dimension = dimension
        self.components = components
        self.alt_names = alt_names
        # Add to registry to allow lookup under a provided name
        if self.name is not None:
            setattr(unit_reg, self.name, self)
        # Also add under any alternative names e.g. meter vs metre
        if self.alt_names is not None:
            for alt_name in self.alt_names:
                setattr(unit_reg, alt_name, self)
        # Also add under the symbol if it has been indicated via canon_symbol
        # that the symbol should uniquely refer to this unit
        if canon_symbol:
            setattr(unit_reg, self.symbol, self)

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

    # For now only allow integer exponents
    def __pow__(self, other):
        if other == 1:
            return self
        elif isinstance(other, int):
            # Tuple comprehensions don't exist so make a tuple from a generator
            new_components = tuple((Factor(component.unit, component.exponent * other) for component in self.components),)
            return CompoundUnit(new_components)
        else:
            return NotImplemented

    def inverse(self):
        """Return the inverse of the unit as a CompoundUnit."""
        # For now just reuse the __pow__ function
        return self**-1

    # Some abstract methods that subclasses need to define
    @abstractmethod
    def base(self):
        """Return the unit's value in base units as a Quantity."""
        pass

    @abstractmethod
    def cancel(self):
        """Combine any like terms."""
        pass


class Unitless(Unit):
    """Special unitless unit."""
    def __init__(self):
        super().__init__(
            symbol="(unitless)",
            name="unitless",
            dimension="X",
            components=(Factor(self, 1),),
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
            )
    
    def base(self):
        """Return the unit's value in base units as a Quantity."""
        return Quantity(1, self)
    
    def cancel(self):
        return self


class CompoundUnit(Unit):
    """An effective unit created through multiplication of non-compound units.
    
    Constituent units must be passed as a tuple of Factors, where each Factor
    is a namedtuple of a unit and an exponent.
    """
    def __init__(self, components):
        # Generate a symbol
        symbol = generate_symbol(components)
        dimension = generate_dimension(components)
        # Don't define a name etc., just a symbol and the components
        super().__init__(
            symbol=symbol,
            name=None,
            dimension=dimension,
            components=components,
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


class DerivedUnit(Unit):
    """Units derived from and defined with SI units.
    
    value is a Quantity with both a number and a Unit.
    A symbol must be provided, but a name is optional.
    If a name is given, the unit will be added to the unit registry under that
    name (note that this will replace any existing unit with that name).
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
        super().__init__(
            symbol,
            name,
            dimension=self.value.unit.dimension,
            components=(Factor(self, 1),),
            canon_symbol=canon_symbol,
            alt_names=alt_names,
            )

    def base(self):
        """Return the unit's value in base units as a Quantity."""
        return self.value.base()
    
    def cancel(self):
        return self
