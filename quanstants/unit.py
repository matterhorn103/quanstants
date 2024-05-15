from collections import namedtuple
from decimal import Decimal as dec
from fractions import Fraction as frac

from .config import quanfig
from .quantity import Quantity
from .unicode import generate_superscript

# Import the main instance of `UnitReg`, which is the default registry for new units
from .unitreg import unit_reg, UnitReg

# Create a named tuple that is used to hold a unit with its exponent
Factor = namedtuple("Factor", ["unit", "exponent"])


# Function to turn a tuple or other iterable of Factors into a symbol
def generate_symbol(
    components: tuple[Factor, ...],
    sort_by="sign",
    inverse=quanfig.INVERSE_UNIT,
) -> str:
    # Create symbol as concatenation of symbols of components, with spaces
    terms = []
    positive_terms = []
    negative_terms = []
    for factor in components:
        term = factor.unit.symbol
        if factor.exponent >= 0:
            term += generate_superscript(factor.exponent)
            if sort_by == "sign":
                positive_terms.append(term)
            else:
                terms.append(term)
        elif factor.exponent < 0:
            if (inverse == "NEGATIVE_SUPERSCRIPT") or (sort_by != "sign"):
                term += generate_superscript(factor.exponent)
            elif (inverse == "SLASH") and (sort_by == "sign"):
                term += generate_superscript(-1 * factor.exponent)
            if sort_by == "sign":
                negative_terms.append(term)
            else:
                terms.append(term)
    if sort_by == "sign":
        if len(negative_terms) > 0:
            if inverse == "NEGATIVE_SUPERSCRIPT":
                return " ".join(positive_terms) + " " + " ".join(negative_terms)
            elif inverse == "SLASH":
                return " ".join(positive_terms) + " / " + " ".join(negative_terms)
        else:
            return " ".join(positive_terms)
    else:
        return " ".join(terms)


# Function to turn a tuple or other iterable of Factors into a dimension
def generate_dimensional_exponents(components: tuple[Factor, ...]) -> dict:
    new_dimensional_exponents = {"T": 0, "L": 0, "M": 0, "I": 0, "Θ": 0, "N": 0, "J": 0}
    for factor in components:
        for dimension in new_dimensional_exponents.keys():
            if dimension in factor.unit.dimensional_exponents:
                new_dimensional_exponents[dimension] += (
                    factor.unit.dimensional_exponents[dimension] * factor.exponent
                )
    return new_dimensional_exponents


# Function to allow sorting of compound base units into a canonical order
def get_priority(factor: Factor) -> int:
    priorities = {
        "s": 0,
        "m": 1,
        "kg": 2,
        "A": 3,
        "K": 4,
        "mol": 5,
        "cd": 6,
        "rad": 7,
        "sr": 8,
    }
    if factor.unit.symbol in priorities:
        priority = priorities[factor.unit.symbol]
    else:
        # Generate a priority based on the length and Unicode code points of the characters
        priority = 0
        for index, char in enumerate(factor.unit.symbol):
            priority += ord(char) * 10 ** (index)
    return priority


class Unit:
    """Parent class for all units, not intended for direct instantiation.

    Either `symbol` or `name` must be provided. If `symbol` is not provided, it will be set to the
    value of `name`, so that all units have a symbolic representation that is used in printed
    representations.
    `symbol` may be any Unicode string.
    `name` must contain only ASCII letters and digits, and underscores. It must in addition be a valid
    Python identifier, so it cannot start with a digit.
    At minimum, `components` and one of `dimension` or `dimensional_exponents` must be specified.
    `components` is a tuple of `Factor`s. `Factor` is a `namedtuple` found in this module.
    If a unit only has a single base dimension without exponents, that dimension can be passed as
    `dimension` as one of the strings "X" (for dimensionless), "T" (time), "L" (length), "M" (mass),
    "I" (electric current), "Θ" (thermodynamic temperature), "N" (amount of substance), or "J" (luminous
    intensity).
    If a unit's dimension comprises multiple base dimensions or exponents, they should be passed as
    `dimensional_exponents` as a dictionary of the form `{"T": 1, "M": 2, ...}` (only those with
    non-zero exponents are required).
    `add_to_reg` specifies whether the unit should be added to `reg` as an attribute under the
    provided `name` and under any alternative names given as a list as `alt_names`.
    If `canon_symbol` is set to `True`, the unit will also be added to `reg` under `symbol`.
    The default unit registry is accessible as `quanstants.units`.
    """

    # Using slots keeps the memory footprint down as there is no __dict__
    # It also helps immutability as users can't add attributes
    __slots__ = (
        "_symbol",
        "_name",
        "_components",
        "_value",
        "_dimensional_exponents",
        "_alt_names",
    )

    def __init__(
        self,
        symbol: str | None,
        name: str | None,
        components: tuple[Factor, ...],
        value: Quantity | None = None,
        dimension: str | None = None,
        dimensional_exponents: dict | None = None,
        reg: UnitReg = unit_reg,
        add_to_reg: bool = False,
        canon_symbol: bool = False,
        alt_names: list[str] | None = None,
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
        # Start with a dimensionless unit and add any provided ones
        self._dimensional_exponents = {
            "T": 0,
            "L": 0,
            "M": 0,
            "I": 0,
            "Θ": 0,
            "N": 0,
            "J": 0,
        }
        if dimension == "X":
            pass
        elif isinstance(dimension, str) and len(dimension) == 1:
            self._dimensional_exponents[dimension] += 1
        else:
            for dim in self._dimensional_exponents.keys():
                if dim in dimensional_exponents:
                    self._dimensional_exponents[dim] += dimensional_exponents[dim]
        self._components = components
        self._value = Quantity(1, self) if value is None else value
        self._alt_names = tuple(alt_names) if alt_names is not None else None
        if add_to_reg:
            self.add_to_reg(reg=reg, add_symbol=canon_symbol)

    @property
    def symbol(self) -> str:
        return self._symbol

    @property
    def name(self) -> str | None:
        return self._name

    @property
    def dimensional_exponents(self) -> dict:
        return self._dimensional_exponents

    @property
    def components(self) -> tuple[Factor, ...]:
        return self._components
    
    @property
    def value(self) -> Quantity:
        return self._value

    @property
    def alt_names(self) -> list[str]:
        return self._alt_names

    def __str__(self):
        return f"Unit({self.symbol})"

    # Units must always come at the end of expressions,
    # so do not define Unit * num or Unit / num
    def __mul__(self, other, concatenate_symbols: bool = False):
        if isinstance(other, UnitlessUnit):
            if concatenate_symbols and not other._drop_on_concat:
                return CompoundUnit(components=None, units=(self, other), concatenate_symbols=True)
            else:
                return self
        elif isinstance(other, Unit):
            if concatenate_symbols:
                return CompoundUnit(components=None, units=(self, other), concatenate_symbols=True)
            else:
                return CompoundUnit(self.components + other.components)
        elif isinstance(other, Quantity):
            return Quantity(other.number, self.__mul__(other.unit, concatenate_symbols=concatenate_symbols))
        else:
            return NotImplemented
    
    def __rmul__(self, other):
        if isinstance(other, (str, int, float, dec)):
            return Quantity(other, self)
        elif isinstance(other, Quantity):
            return Quantity(other.number, other.unit * self)
        else:
            return NotImplemented

    def __truediv__(self, other):
        if isinstance(other, UnitlessUnit):
            return self
        elif isinstance(other, Unit):
            return CompoundUnit(self.components + other.inverse().components)
        elif isinstance(other, Quantity):
            return Quantity(1 / other.number, self / other.unit)
        else:
            return NotImplemented    

    def __rtruediv__(self, other):
        if isinstance(other, (str, int, float, dec)):
            return Quantity(other, self.inverse())
        elif isinstance(other, Quantity):
            return Quantity(other.number, other.unit / self)
        else:
            return NotImplemented

    # For now only allow integer and fractional exponents, or string representations of fractions
    def __pow__(self, other):
        if other == 1:
            return self
        elif isinstance(other, (int, frac)):
            # Tuple comprehensions don't exist so make a tuple from a generator
            new_components = tuple(
                (
                    Factor(component.unit, component.exponent * other)
                    for component in self.components
                ),
            )
            return CompoundUnit(new_components)
        elif isinstance(other, str):
            new_components = tuple(
                (
                    Factor(component.unit, component.exponent * frac(other))
                    for component in self.components
                ),
            )
            return CompoundUnit(new_components)
        else:
            return NotImplemented

    # Hashing and equalities use the implementations of `Quantity`
    # Units are thus considered equal to quantities that have an equivalent value
    def __hash__(self):
        return hash(self.value)

    def __eq__(self, other):
        if isinstance(other, (Unit, Quantity)):
            # Compare the values (handled by `Quantity`)
            return self.value == other.value
        else:
            return NotImplemented

    def __gt__(self, other):
        if isinstance(other, (Unit, Quantity)):
            # Compare the values (handled by `Quantity`)
            return self.value > other.value
        else:
            return NotImplemented

    def __ge__(self, other):
        if isinstance(other, (Unit, Quantity)):
            # Compare the values (handled by `Quantity`)
            return self.value >= other.value
        else:
            return NotImplemented

    def inverse(self):
        """Return the inverse of the unit as a CompoundUnit."""
        # For now just reuse the __pow__ function
        return self**-1

    def add_to_reg(self, reg: UnitReg = unit_reg, add_symbol=False):
        # Add to specified registry to allow lookup under a provided name
        if self.name is not None:
            reg.add(self.name, self)
        # Also add under any alternative names e.g. meter vs metre
        if self.alt_names is not None:
            for alt_name in self.alt_names:
                reg.add(alt_name, self)
        # Also add under the symbol if it has been indicated via canon_symbol
        # that the symbol should uniquely refer to this unit
        if (add_symbol) and (self.symbol != self.name):
            reg.add(self.symbol, self)

    def is_dimensionless(self) -> bool:
        if self.dimensional_exponents == {
            "T": 0,
            "L": 0,
            "M": 0,
            "I": 0,
            "Θ": 0,
            "N": 0,
            "J": 0,
        }:
            return True
        else:
            return False

    def dimension(self) -> str:
        """Return the dimension as a nice string."""
        if self.is_dimensionless():
            return "(dimensionless)"
        else:
            result = ""
            for dimension, exponent in self.dimensional_exponents.items():
                if exponent != 0:
                    result += dimension
                    if exponent != 1:
                        result += generate_superscript(exponent)
            return result

    # Some methods that subclasses need to redefine
    def base(self):
        """Return the unit's value in base units as a `Quantity`."""
        raise NotImplementedError

    def cancel(self):
        """Combine any like terms and return as a `Quantity`."""
        raise NotImplementedError

    # Sometimes necessary to redefine but defaults to cancel() if not
    def fully_cancel(self):
        """Combine any like terms and return as a `Quantity`, with units of the same dimension converted and also combined."""
        return self.cancel()

    def canonical(self):
        """Order terms into a reproducible order and return as a `Quantity`."""
        raise NotImplementedError


class BaseUnit(Unit):
    """SI base units and other base units that are not defined in terms of other units.
    
    The key property of a `BaseUnit` is that a request to express it in base units, to
    cancel it, or to express it canonically, simply returns a `Quantity` of unity times
    the `BaseUnit`.
    """

    __slots__ = ()

    def __init__(
        self,
        symbol: str,
        name: str,
        dimension: str,
        reg: UnitReg = unit_reg,
        add_to_reg: bool = True,
        canon_symbol: bool = True,
        alt_names: list[str] = None,
    ):
        super().__init__(
            symbol,
            name,
            components=(Factor(self, 1),),
            dimension=dimension,
            reg=reg,
            add_to_reg=add_to_reg,
            canon_symbol=canon_symbol,
            alt_names=alt_names,
        )

    def base(self) -> Quantity:
        """Return the unit's value in base units as a Quantity.
        
        For a `BaseUnit`, simply returns a `Quantity` of unity times the `BaseUnit`.
        """
        return self.value

    def cancel(self) -> Quantity:
        """Combine any like terms and return as a Quantity.
        
        For a `BaseUnit`, simply returns a `Quantity` of unity times the `BaseUnit`.
        """
        return self.value

    def canonical(self) -> Quantity:
        """Order terms into a reproducible order and return as a Quantity.
        
        For a `BaseUnit`, simply returns a `Quantity` of unity times the `BaseUnit`.
        """
        return self.value


class UnitlessUnit(BaseUnit):
    """Special dimensionless units that are numerically equal to 1.
    
    Derives from `BaseUnit` and acts similar in most ways, but in arithmetic and
    equalities behaves like unity.
    Note that a unitless unit is not simply dimensionless, and not all dimensionless
    units are instances of `UnitlessUnit`.
    For example, the degree is defined as a dimensionless `DerivedUnit` defined in terms
    of radian, and percent is defined as a dimensionless `DerivedUnit` defined in terms
    of unitless, while both `quanstants.units.radian` and `quanstants.units.unitless`
    are instances of `UnitlessUnit`.
    """

    __slots__ = ("_drop_on_concat")

    def __init__(
        self,
        symbol: str | None = None,
        name: str | None = None,
        reg: UnitReg = unit_reg,
        add_to_reg: bool = True,
        canon_symbol: bool = True,
        alt_names: list[str] | None = None,
        drop_on_concat: bool = False,
    ):
        super().__init__(
            symbol=symbol,
            name=name,
            dimension="X",
            reg=reg,
            add_to_reg=add_to_reg,
            canon_symbol=canon_symbol,
            alt_names=alt_names,
        )
        self._drop_on_concat = drop_on_concat

    # Make sure that UnitlessUnit * Unit and UnitlessUnit / Unit return just the other Unit
    # The exception being if concatenation is requested
    def __mul__(self, other, concatenate_symbols: bool = False):
        if concatenate_symbols and not self._drop_on_concat:
            return super().__mul__(other, concatenate_symbols=concatenate_symbols)
        elif isinstance(other, (Unit, Quantity)):
            return other
        else:
            return super().__mul__(other)

    def __rmul__(self, other):
        if isinstance(other, (Unit, Quantity)):
            return other
        else:
            return super().__rmul__(other)

    def __truediv__(self, other):
        if isinstance(other, (Unit, Quantity)):
            return other.inverse()
        else:
            return super().__truediv__(other)

    def __rtruediv__(self, other):
        if isinstance(other, (Unit, Quantity)):
            return other
        else:
            return super().__rtruediv__(other)

    # UnitlessUnits are numerically equal to 1, so raising to a power has no effect
    def __pow__(self, other):
        return self

    # UnitlessUnits also need to evaluate to equal to 1, because they hash to 1 (they
    # are unique in this respect - no other units are equal to a numerical value)
    def __hash__(self):
        return 1

    def __eq__(self, other):
        return 1 == other
    
    def __gt__(self, other):
        return 1 > other
    
    def __ge__(self, other):
        return 1 >= other


# Instantiate the main UnitlessUnit instance which is the one typically used internally
unitless = UnitlessUnit(
    symbol="(unitless)",
    name="unitless",
    add_to_reg=True,
    canon_symbol=False,
    alt_names=None,
    drop_on_concat=True,
)


class DerivedUnit(Unit):
    """Units derived from and defined with SI units.

    `value` is a `Quantity` with both a number and a `Unit`, and optionally, an uncertainty.
    A `symbol` must be provided, but a `name` is optional.
    If a name is given and `add_to_reg` is `True` (default), the unit will be added to the
    unit registry under that name (note that trying to replace an existing unit with that name
    will raise an error).
    The `dimensional_exponents` are set to that of the provided value's unit(s).
    """

    __slots__ = ()

    def __init__(
        self,
        symbol: str,
        name: str,
        value: Quantity,
        add_to_reg: bool = True,
        reg: UnitReg = unit_reg,
        canon_symbol: bool = False,
        alt_names: list[str] | None = None,
    ):
        super().__init__(
            symbol,
            name,
            components=(Factor(self, 1),),
            value=value,
            dimensional_exponents=value.unit.dimensional_exponents,
            add_to_reg=add_to_reg,
            reg=reg,
            canon_symbol=canon_symbol,
            alt_names=alt_names,
        )

    def base(self) -> Quantity:
        """Return the unit's value in base units as a Quantity."""
        return self.value.base()

    def cancel(self) -> Quantity:
        return 1 * self

    def canonical(self) -> Quantity:
        return 1 * self


class CompoundUnit(Unit):
    """An effective unit created through multiplication of non-compound units.

    Multiple units multiplied together are treated as a single `Unit` object with its constituent parts
    gathered under `components`.
    Generally, the constituent units are passed as a tuple of `Factor` objects.
    Alternatively, a tuple of `Unit` objects can be passed and the `components` attributes of each will
    be combined automatically.
    """

    __slots__ = ("_defined_in_base")

    def __init__(
        self,
        components: tuple[Factor, ...] | None = None,
        units: tuple[Unit] | None = None,
        name: str | None = None,
        add_to_reg: bool = False,
        reg: UnitReg = unit_reg,
        alt_names: list[str] | None = None,
        symbol_sort: str = "sign",
        symbol_inverse: str = quanfig.INVERSE_UNIT,
        concatenate_symbols: bool = False,
    ):
        # If no components passed, first get components from list of units
        if components is None:
            # Repeated addition of tuples, starting with (), fastest method for low n (n < 10)
            components = sum([unit.components for unit in units], ())

        # Generate a new symbol based on passed options
        if (units is None) or (not concatenate_symbols):
            symbol = generate_symbol(components, symbol_sort, symbol_inverse)
        else:
            # Maintain visual separation of combined units in symbol
            # Put each unit's symbol in parentheses if they contain a slash
            symbols = []
            for unit in units:
                if "/" in unit.symbol:
                    symbols.append("(" + unit.symbol + ")")
                else:
                    symbols.append(unit.symbol)
            symbol = " ".join(symbols)
            
        dimensional_exponents = generate_dimensional_exponents(components)
        alt_names = alt_names if alt_names else None
        # Don't define a name etc., just a symbol and the components
        super().__init__(
            symbol=symbol,
            name=name,
            components=components,
            dimensional_exponents=dimensional_exponents,
            add_to_reg=add_to_reg,
            reg=reg,
            alt_names=alt_names,
        )
        # Determine whether the unit is expressed in terms of base units
        defined_in_base = True
        for component in self.components:
            defined_in_base *= isinstance(component.unit, BaseUnit)
        self._defined_in_base = bool(defined_in_base)

    @property
    def defined_in_base(self):
        return self._defined_in_base

    def base(self) -> Quantity:
        """Return the unit's value in base units as a Quantity."""
        if self.defined_in_base:
            return Quantity(1, self)
        else:
            result_number = 1
            new_components = tuple()
            for component in self.components:
                component_unit_base = component.unit.base()
                # Do this way to avoid creating a new compound unit at every step
                result_number *= component_unit_base.number ** component.exponent
                new_components += (component_unit_base.unit ** component.exponent).components
                # TODO Uncertainty in units?
            return Quantity(result_number, CompoundUnit(new_components))

    def cancel(self) -> Quantity:
        """Combine any like terms."""
        new_components_list = []
        for factor in self.components:
            unit_in_list = False
            for index, new_factor in enumerate(new_components_list):
                if new_factor.unit.name == factor.unit.name:
                    new_components_list[index] = Factor(
                        factor.unit, factor.exponent + new_factor.exponent
                    )
                    unit_in_list = True
                else:
                    continue
            if not unit_in_list:
                new_components_list.append(factor)
        new_components = tuple(
            factor for factor in new_components_list if factor.exponent != 0
        )
        if len(new_components) == 0:
            return 1 * unitless
        else:
            return 1 * CompoundUnit(new_components)

    def fully_cancel(self) -> Quantity:
        """Combine any like terms, with units of the same dimension converted and also combined.
        
        Units of the same dimension are converted to whichever unit is a base unit, and otherwise to
        whichever occurs first.
        Any superfluous terms of `Unitless` units (i.e. equal to 1) will also be dropped.
        """
        # First cancel like normal
        cancelled = self.cancel().unit
        # Find first non-unitless factor
        first = None
        i, i_max = 0, len(cancelled.components) - 1
        while first is None:
            if i > i_max:
                # All components are unitless so overall unit is unitless
                return 1 * unitless
            if isinstance(cancelled.components[i].unit, UnitlessUnit):
                i += 1
            else:
                first = cancelled.components[i]
        # Check if first component needs to be converted before we add it to result
        first_matched = False
        for other in cancelled.components[i+1:]:
            if first.unit.dimensional_exponents == other.unit.dimensional_exponents:
                if isinstance(other, BaseUnit):
                    result = first.unit.base() ** first.exponent
                else:
                    result = first.unit ** first.exponent
                first_matched = True
                break
            else:
                continue
        if not first_matched:
            result = first.unit ** first.exponent
        # Bring in each remaining component to the result, converting if necessary
        for component in cancelled.components[i+1:]:
            component_matched = False
            for other in result.components:
                if component.unit.dimensional_exponents == other.unit.dimensional_exponents:
                    result *= ((1 * component.unit).to(other.unit)) ** component.exponent
                    component_matched = True
                    break
                else:
                    continue
            if not component_matched:
                result *= component.unit ** component.exponent
        # Drop any unitless units (not dimensionless ones)
        new_components = tuple(
            factor for factor in result.unit.components if not isinstance(factor.unit, UnitlessUnit)
        )
        if len(new_components) == 0:
            result = result.number * unitless
        else:
            result = result.number * CompoundUnit(new_components)
        # Finally cancel again
        return result.cancel()

    def canonical(self) -> Quantity:
        """Order terms into a reproducible order and return as a Quantity."""
        ordered_components = tuple(sorted(self.components, key=get_priority))
        # Now that the components have the canonical order, make sure the order of units in the
        # generated symbol is the same by passing appropriate settings
        return 1 * CompoundUnit(
            ordered_components,
            symbol_sort="unsorted",
            symbol_inverse="NEGATIVE_SUPERSCRIPT",
            concatenate_symbols=False,
        )
