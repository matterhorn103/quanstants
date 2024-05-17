from collections import namedtuple, Counter
from decimal import Decimal as dec
from fractions import Fraction as frac

from .config import quanfig
from .quantity import Quantity
from .unicode import generate_superscript

# Import the units namespace module, in which named units are registered
from . import units


# Function to turn a tuple or other iterable of factors into a symbol
def generate_symbol(
    components: tuple[tuple, ...],
    sort_by="sign",
    inverse=quanfig.INVERSE_UNIT,
) -> str:
    # Create symbol as concatenation of symbols of components, with spaces
    terms = []
    positive_terms = []
    negative_terms = []
    for factor in components:
        term = factor[0].symbol
        if factor[1] >= 0:
            term += generate_superscript(factor[1])
            if sort_by == "sign":
                positive_terms.append(term)
            else:
                terms.append(term)
        elif factor[1] < 0:
            if (inverse == "NEGATIVE_SUPERSCRIPT") or (sort_by != "sign"):
                term += generate_superscript(factor[1])
            elif (inverse == "SLASH") and (sort_by == "sign"):
                term += generate_superscript(-1 * factor[1])
            if sort_by == "sign":
                negative_terms.append(term)
            else:
                terms.append(term)
    if sort_by == "sign":
        if len(negative_terms) > 0:
            if inverse == "NEGATIVE_SUPERSCRIPT":
                return " ".join(positive_terms) + " " + " ".join(negative_terms)
            elif inverse == "SLASH":
                return " ".join(positive_terms) + "/" + " ".join(negative_terms)
        else:
            return " ".join(positive_terms)
    else:
        return " ".join(terms)


# Function to turn a tuple or other iterable of factors into a dimension
def generate_dimensional_exponents(components: tuple[tuple, ...]) -> dict:
    new_dimensional_exponents = {"L": 0, "M": 0, "T": 0, "I": 0, "Θ": 0, "N": 0, "J": 0}
    for unit, exponent in components:
        for dimension in new_dimensional_exponents.keys():
            if dimension in unit.dimensional_exponents:
                new_dimensional_exponents[dimension] += (
                    unit.dimensional_exponents[dimension] * exponent
                )
    return new_dimensional_exponents


# Function to allow sorting of compound base units into a canonical order
def get_priority(factor: tuple) -> int:
    priorities = {
        "m": 0,
        "kg": 1,
        "s": 2,
        "A": 3,
        "K": 4,
        "mol": 5,
        "cd": 6,
    }
    if isinstance(factor[0], BaseUnit) and factor[0].symbol in priorities:
        priority = priorities[factor[0].symbol]
    else:
        # Generate a priority based on the length and Unicode code points of the characters
        priority = 0
        for index, char in enumerate(factor[0].symbol):
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
    `components` is a tuple of factors, where factors are simple tuples of `(unit, exponent)`.
    If a unit only has a single base dimension without exponents, that dimension can be passed as
    `dimension` as one of the strings "X" (for dimensionless), "T" (time), "L" (length), "M" (mass),
    "I" (electric current), "Θ" (thermodynamic temperature), "N" (amount of substance), or "J" (luminous
    intensity).
    If a unit's dimension comprises multiple base dimensions or exponents, they should be passed as
    `dimensional_exponents` as a dictionary of the form `{"T": 1, "M": 2, ...}` (only those with
    non-zero exponents are required).
    `add_to_namespace` specifies whether the unit should be added to `quanstants.units` under the
    provided `name` and under any alternative names given as a list as `alt_names`.
    If `canon_symbol` is set to `True`, the unit will also be added to `quanstants.units` under
    `symbol`.
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
        "_value_base",
    )

    def __init__(
        self,
        symbol: str | None,
        name: str | None,
        components: tuple[tuple, ...],
        value: Quantity | None = None,
        dimension: str | None = None,
        dimensional_exponents: dict | None = None,
        add_to_namespace: bool = False,
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
            "L": 0,
            "M": 0,
            "T": 0,
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
        if add_to_namespace:
            self.add_to_namespace(add_symbol=canon_symbol)

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
    def components(self) -> tuple[tuple, ...]:
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
                    (unit, exponent * other)
                    for unit, exponent in self.components
                ),
            )
            return CompoundUnit(new_components)
        elif isinstance(other, str):
            new_components = tuple(
                (
                    (unit, exponent * frac(other))
                    for unit, exponent in self.components
                ),
            )
            return CompoundUnit(new_components)
        else:
            return NotImplemented

    # Hashing and equalities by default use the implementations of `Quantity`
    # Units are thus considered equal to quantities that have an equivalent value
    # These are fallback methods, for efficiency some subclasses override these
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

    def add_to_namespace(self, add_symbol=False):
        # Add to units namespace to allow lookup under a provided name
        if self.name is not None:
            units.add(self.name, self)
        # Also add under any alternative names e.g. meter vs metre
        if self.alt_names is not None:
            for alt_name in self.alt_names:
                units.add(alt_name, self)
        # Also add under the symbol if it has been indicated via canon_symbol
        # that the symbol should uniquely refer to this unit
        if (add_symbol) and (self.symbol != self.name):
            units.add(self.symbol, self)

    def is_dimensionless(self) -> bool:
        if self.dimensional_exponents == {
            "L": 0,
            "M": 0,
            "T": 0,
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

    # Some methods that subclasses might need to redefine
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
    
    def base(self):
        """Return the unit's value in base units as a `Quantity`.
        
        This is always returned in a fully cancelled, canonical form.
        """
        return self._value_base


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
        add_to_namespace: bool = True,
        canon_symbol: bool = True,
        alt_names: list[str] = None,
    ):
        super().__init__(
            symbol,
            name,
            components=((self, 1),),
            dimension=dimension,
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
            alt_names=alt_names,
        )
        self._value_base = self.value

    # Since base units are unique, they just hash their id
    # This should match hash(Quantity(1, <base unit>))
    def __hash__(self) -> int:
        return hash((1, (id(self), 1)))

    def __eq__(self, other):
        if other is self:
            return True
        else:
            return False

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

    __slots__ = ("_drop")

    def __init__(
        self,
        symbol: str | None = None,
        name: str | None = None,
        add_to_namespace: bool = True,
        canon_symbol: bool = True,
        alt_names: list[str] | None = None,
        drop: bool = False,
    ):
        super().__init__(
            symbol=symbol,
            name=name,
            dimension="X",
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
            alt_names=alt_names,
        )
        self._drop = drop

    def __mul__(self, other, concatenate_symbols: bool = False):
        if self._drop and isinstance(other, (Unit, Quantity)):
            return other
        else:
            return super().__mul__(other, concatenate_symbols=concatenate_symbols)

    def __rmul__(self, other):
        if self._drop and isinstance(other, (Unit, Quantity)):
            return other
        else:
            return super().__rmul__(other)

    def __truediv__(self, other):
        if self._drop and isinstance(other, (Unit, Quantity)):
            return other.inverse()
        else:
            return super().__truediv__(other)

    def __rtruediv__(self, other):
        if self._drop and isinstance(other, (Unit, Quantity)):
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
    add_to_namespace=True,
    canon_symbol=False,
    alt_names=None,
    drop=True,
)


class CompoundUnit(Unit):
    """An effective unit created through multiplication of non-compound units.

    Multiple units multiplied together are treated as a single `Unit` object with its constituent parts
    gathered under `components`.
    Generally, the constituent units are passed as a tuple of factors (tuples of the form
    `(unit, exponent)`).
    Alternatively, a tuple of `Unit` objects can be passed and the `components` attributes of each will
    be combined automatically.
    """

    __slots__ = ()

    def __init__(
        self,
        components: tuple[tuple, ...] | None = None,
        units: tuple[Unit] | None = None,
        name: str | None = None,
        add_to_namespace: bool = False,
        alt_names: list[str] | None = None,
        symbol_sort: str = "sign",
        symbol_inverse: str = quanfig.INVERSE_UNIT,
        concatenate_symbols: bool = False,
        is_canon_base: bool = False,
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
            add_to_namespace=add_to_namespace,
            alt_names=alt_names,
        )
        # Express the unit in terms of base units
        if is_canon_base:
            self._value_base = self._value
        else:
            self._value_base = None

    def __hash__(self) -> int:
        # Make the hashing faster by doing it directly, since we know that doing
        # self.value.base() would just give self._value_base, and we've possibly
        # already calculated that
        base_ids = ((id(u), e) for u, e in self.base().unit.components)
        return hash((self.base().number, *base_ids))

    def __eq__(self, other):
        return hash(self) == hash(other)

    def _cancel_to_unit(self, force_drop_unitless=False) -> Unit:
        """Does everything that `self.cancel() does, but returns a unit."""
        new_components_dict = {}
        unitless_components_dict = {}
        for unit, exponent in self.components:
            if isinstance(unit, UnitlessUnit):
                if force_drop_unitless or unit.drop:
                    continue
                else:
                    # Can't just use unit as dict key as all UnitlessUnits are equal, so use name
                    if unit.name in unitless_components_dict:
                        unitless_components_dict[unit.name][exponent] += exponent
                    else:
                        unitless_components_dict[unit.name] = {"unit": unit, "exponent": exponent}
            else:
                if unit in new_components_dict:
                    new_components_dict[unit] += exponent
                else:
                    new_components_dict[unit] = exponent
        new_components = tuple((u, e) for u, e in new_components_dict.items() if e != 0)
        if len(unitless_components_dict) > 0:
            new_components += tuple((d["unit"], d["exponent"]) for d in new_components_dict.values() if d["exponent"] != 0)
        if len(new_components) == 0:
            return unitless
        else:
            return CompoundUnit(new_components)

    def cancel(self, force_drop_unitless=False) -> Quantity:
        """Combine any like terms.
        
        Note that "like" means that the units are equivalent in value, not that they are
        the same Unit object.
        Terms of `Unitless` units which have `drop = True` will also be dropped; this is the case for
        `quanstants.units.unitless`, but not for `radian` or `steradian`.
        """
        return Quantity(1, self._cancel_to_unit(force_drop_unitless=force_drop_unitless))

    def fully_cancel(self) -> Quantity:
        """Combine any like terms, with units of the same dimension converted and also combined.
        
        Units of the same dimension are converted to whichever unit is a base unit, and otherwise to
        whichever occurs first.
        Any terms of `Unitless` units (i.e. equal to 1) will also be dropped.
        """
        result_number = 1
        new_components_dict = {}
        # First cancel like normal, this also gets rid of all UnitlessUnits
        cancelled = self._cancel_to_unit()
        if cancelled is unitless:
            return Quantity(1, unitless)
        # Check if first component needs to be converted before we add it to result
        first_unit, first_exponent = cancelled.components[0]
        if isinstance(first_unit, BaseUnit):
            new_components_dict[first_unit] = first_exponent
        else:
            first_matched = False
            for other in cancelled.components[1:]:
                if isinstance(other[0], BaseUnit):
                    if first_unit.dimensional_exponents == other[0].dimensional_exponents:
                        first_unit_in_base = first_unit.base()
                        result_number *= first_unit_in_base.number ** first_exponent
                        new_components_dict[first_unit] = first_exponent
                        first_matched = True
                        break
            if not first_matched:
                new_components_dict[first_unit] = first_exponent
        # Bring in each remaining component to the result, converting if necessary
        for unit, exponent in cancelled.components[1:]:
            component_matched = False
            for unit_already_in in new_components_dict.keys():
                if unit.dimensional_exponents == unit_already_in.dimensional_exponents:
                    converted_unit = unit.value.to(unit_already_in)
                    result_number *= converted_unit.number ** exponent
                    new_components_dict[unit_already_in] += exponent
                    component_matched = True
                    break
                else:
                    continue
            if not component_matched:
                new_components_dict[unit] = exponent
        # TODO Keep uncertainties
        new_components = tuple((u, e) for u, e in new_components_dict.items() if e != 0)
        if len(new_components) == 0:
            return Quantity(result_number, unitless)
        else:
            return Quantity(result_number, CompoundUnit(new_components))

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

    def base(self) -> Quantity:
        """Return the unit's value in base units as a Quantity.
        
        Do without creating any intermediate compound units.
        Drop unitless units, cancel like terms, and put in canonical order so that
        different units with equal values give _identical_ results
        """
        # Check to see if it has been pre-calculated
        if self._value_base is not None:
            return self._value_base
        result_number = 1
        base_components_dict = {}
        # Do this way to avoid creating a new compound unit at every step
        for unit, exponent in self.components:
            if isinstance(unit, UnitlessUnit):
                # Drop
                continue
            elif isinstance(unit, BaseUnit):
                if unit in base_components_dict:
                    base_components_dict[unit] += exponent
                else:
                    base_components_dict[unit] = exponent
            else:
                # Get the unit of the component expressed in terms of base units
                # Multiply the number of the result by the number of the expression
                # Add the components of the unit of the expression to the running dict
                if exponent == 1:
                    result_number *= unit.base().number
                    component_base_factors = unit.base().unit.components
                else:
                    result_number *= unit.base().number ** exponent
                    component_base_factors = tuple((u, (e * exponent)) for u, e in unit.base().unit.components)
                for base_unit, base_exponent in component_base_factors:
                    if base_unit in base_components_dict:
                        base_components_dict[base_unit] += base_exponent
                    else:
                        base_components_dict[base_unit] = base_exponent
        # TODO Uncertainty in units?
        # Turn into tuple, get rid of base units with exponent 0
        base_components = tuple((u, e) for u, e in base_components_dict.items() if e != 0)
        # If no units left, return as unitless quantity
        if len(base_components) == 0:
            return Quantity(result_number, unitless)
        # Put in order to create canonical form
        base_components = tuple(sorted(base_components, key=get_priority))
        self._value_base = Quantity(
            result_number,
            CompoundUnit(
                base_components,
                symbol_sort="unsorted",
                symbol_inverse="NEGATIVE_SUPERSCRIPT",
                concatenate_symbols=False,
                is_canon_base=True,
            )
        )
        return self._value_base
    

class DerivedUnit(Unit):
    """Units derived from and defined with SI units.

    `value` is a `Quantity` with both a number and a `Unit`, and optionally, an uncertainty.
    A `symbol` must be provided, but a `name` is optional.
    If a name is given and `add_to_namespace` is `True` (default), the unit will be added to
    `quanstants.units` under that name (note that trying to replace an existing unit with that name
    will raise an error).
    The `dimensional_exponents` are set to that of the provided value's unit(s).
    """

    __slots__ = ()

    def __init__(
        self,
        symbol: str,
        name: str,
        value: Quantity,
        add_to_namespace: bool = True,
        canon_symbol: bool = False,
        alt_names: list[str] | None = None,
    ):
        super().__init__(
            symbol,
            name,
            components=((self, 1),),
            value=value,
            dimensional_exponents=value.unit.dimensional_exponents,
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
            alt_names=alt_names,
        )
        self._value_base = self.value.base()

    def cancel(self) -> Quantity:
        return 1 * self

    def canonical(self) -> Quantity:
        return 1 * self
