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
def generate_dimensional_exponents(components):
    new_dimensional_exponents = {"T": 0, "L": 0, "M": 0, "I": 0, "Θ": 0, "N": 0, "J": 0}
    for factor in components:
        for dimension in new_dimensional_exponents.keys():
            if dimension in factor.unit.dimensional_exponents:
                new_dimensional_exponents[dimension] += factor.unit.dimensional_exponents[dimension] * factor.exponent
    return new_dimensional_exponents

# Function to allow sorting of compound base units into a canonical order
def base_priority(Factor):
    priorities = {"s": 0, "m": 1, "kg": 2, "A": 3, "K": 4, "mol": 5, "cd": 6, "rad": 7, "sr": 8}
    if Factor.unit.symbol in priorities:
        return priorities[Factor.unit.symbol]
    else:
        return 9


class Unit:
    """Parent class for all units.
    
    Not intended for direct instantiation
    At minimum, `components` and one of `dimension` or `dimensional_exponents` must be specified.
    `components` is a tuple of `Factor`s. `Factor` is a `namedtuple` found in this module.
    If a unit only has a single base dimension without exponents, that dimension can be passed as
    `dimension` as one of the strings "X" (for dimensionless), "T" (time), "L" (length), "M" (mass),
    "I" (electric current), "Θ" (thermodynamic temperature), "N" (amount of substance), or "J" (luminous
    intensity).
    If a unit's dimension comprises multiple base dimensions or exponents, they should be passed as
    `dimensional_exponents` as a dictionary of the form `{"T": 1, "M": 2, ...}` (only those with
    non-zero exponents are required).
    `add_to_reg` specifies whether the unit should be added to `unit_reg` as an attribute under the
    provided `name` and under any alternative names given as a list as `alt_names`.
    If `canon_symbol` is set to `True`, the unit will also be added to `unit_reg` under `symbol`.
    """
    def __init__(
        self,
        symbol: str | None,
        name: str | None,
        components: tuple,
        dimension: str | None = None,
        dimensional_exponents: dict | None = None,
        add_to_reg=False,
        canon_symbol=False,
        alt_names=None,
    ):
        self.symbol = symbol
        self.name = name
        # Start with a dimensionless unit and add any provided ones
        self.dimensional_exponents = {"T": 0, "L": 0, "M": 0, "I": 0, "Θ": 0, "N": 0, "J": 0}
        if dimension == "X":
            pass
        elif isinstance(dimension, str) and len(dimension) == 1:
            self.dimensional_exponents[dimension] += 1
        else:
            for dim in self.dimensional_exponents.keys():
                if dim in dimensional_exponents:
                    self.dimensional_exponents[dim] += dimensional_exponents[dim]
        self.components = components
        self.canon_symbol = canon_symbol
        self.alt_names = alt_names
        if add_to_reg:
            self.add_to_reg()

    def __str__(self):
        return f"Unit({self.symbol})"

    # Define logic for arithmetic operators, used for unit creation

    # Only allow num * Unit, not Unit * num
    # Quantity * Unit is defined by Quantity.__mul__()
    def __rmul__(self, other):
        if isinstance(other, (str, int, float, dec)):
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
        elif isinstance(other, Unit):
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
    
    def add_to_reg(self):
        # Add to registry to allow lookup under a provided name
        if self.name is not None:
            unit_reg.add(self.name, self)
        # Also add under any alternative names e.g. meter vs metre
        if self.alt_names is not None:
            for alt_name in self.alt_names:
                unit_reg.add(alt_name, self)
        # Also add under the symbol if it has been indicated via canon_symbol
        # that the symbol should uniquely refer to this unit
        if (self.canon_symbol) and (self.symbol != self.name):
            unit_reg.add(self.symbol, self)

    def dimension(self):
        """Return the dimension as a nice string."""
        result = ""
        for dimension, exponent in self.dimensional_exponents.items():
            if exponent != 0:
                result += dimension
                if exponent != 1:
                    result += generate_superscript(exponent)
        if len(result) > 0:
            return result
        else:
            return "(dimensionless)"
    
    # Some methods that subclasses need to redefine
    def base(self):
        """Return the unit's value in base units as a Quantity."""
        raise NotImplementedError

    def cancel(self):
        """Combine any like terms and return as a Quantity."""
        raise NotImplementedError
    
    # Likely necessary to redefine but defaults to cancel() if not
    def fully_cancel(self):
        """Combine any like terms and return as a Quantity, with units of the same dimension converted and also combined."""
        return self.cancel()
    
    def canonical(self):
        """Order any base terms and return as a Unit."""
        raise NotImplementedError


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
        return 1 * self

    def cancel(self):
        """Return unity as a unitless Quantity."""
        return 1 * self
    
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
        """Return a Quantity of unity times itself."""
        return 1 * self
    
    def cancel(self):
        """Return a Quantity of unity times itself."""
        return 1 * self
    
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
        dimensional_exponents = generate_dimensional_exponents(components)
        # Don't define a name etc., just a symbol and the components
        super().__init__(
            symbol=symbol,
            name=None,
            components=components,
            dimensional_exponents=dimensional_exponents,
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
        """Combine any like terms."""
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
            return 1 * Unitless()
        return 1 * CompoundUnit(new_components)
    
    def fully_cancel(self):
        """Combine any like terms, with units of the same dimension converted and also combined."""
        # First cancel like normal
        cancelled = self.cancel()
        # Check if we need to convert first factor
        first = cancelled.components[0]
        first_matched = False
        for other in cancelled.components[1:]:
            if (first.dimension() != "(unitless)") and ((first.dimensional_exponents == other.dimensional_exponents) or (first.dimensional_exponents == {dim: exp * -1 for dim, exp in other.dimensional_exponents.items()})):
                if isinstance(other, BaseUnit):
                    product = first.unit.base() ** first.exponent
                    first_matched = True
                break
            else:
                continue
        if not first_matched:
            product = first.unit ** first.exponent
        for component in cancelled.components[1:]:
            component_matched = False
            for other in product.components:
                if (component.dimension() != "(unitless)") and ((component.dimensional_exponents == other.dimensional_exponents) or (component.dimensional_exponents == {dim: exp * -1 for dim, exp in other.dimensional_exponents.items()})):
                    if isinstance(other, BaseUnit):
                        product *= component.unit.base() ** component.exponent
                    elif isinstance(component, BaseUnit):
                        product *= component.unit ** component.exponent
                    else:
                        product *= (1 * component.unit).to(other.unit)
                    break
                else:
                    continue
            if not component_matched:
                product *= component.unit ** component.exponent
        # Finally cancel again
        return product.cancel()     
                
    def canonical(self):
        ordered_components = tuple(sorted(self.components, key=base_priority))
        return CompoundUnit(ordered_components)            


class DerivedUnit(Unit):
    """Units derived from and defined with SI units.
    
    `value` is a `Quantity` with both a number and a `Unit`, and optionally, an uncertainty.
    A `symbol` must be provided, but a `name` is optional.
    If a name is given and `add_to_reg` is `True` (default), the unit will be added to the
    unit registry under that name (note that trying to replace an existing unit with that name
    will raise an error).
    The `dimensional_exponents` are set to that of the provided value's unit(s).
    """
    def __init__(
        self,
        symbol: str,
        name: str,
        value: Quantity,
        add_to_reg: bool = True,
        canon_symbol: bool = False,
        alt_names: list | None = None,
    ):
        self.value = value
        super().__init__(
            symbol,
            name,
            components=(Factor(self, 1),),
            dimensional_exponents=self.value.unit.dimensional_exponents,
            add_to_reg=add_to_reg,
            canon_symbol=canon_symbol,
            alt_names=alt_names,
        )

    def base(self):
        """Return the unit's value in base units as a Quantity."""
        return self.value.base()
    
    def cancel(self):
        return 1 * self
    
    def canonical(self):
        return self
