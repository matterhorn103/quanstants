from collections import Counter
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


# Fastest if we have list and dict pregenerated
dimensions = ["L", "M", "T", "I", "Θ", "N", "J"]
empty_dimensional_dict = {"L": 0, "M": 0, "T": 0, "I": 0, "Θ": 0, "N": 0, "J": 0}

# Tried using Counters for this but addition via dict comprehension was much faster
# (286 ns vs 1.8 µs) as was an equality (36 ns vs 960 ns)
# Sadly UserDict is also slow (755 ns and 3 µs respectively)

# With dict comprehensions and the set of dimensions:
# add takes 518 ns,
# mul takes 439 ns,
# eq takes 35 ns

#class DimensionalExponents:
#    def __init__(self, *args, **kwargs):
#        self.dims = dict(*args, **kwargs)
#    
#    def __add__(self, other):
#        # 894 ns
#        self.dims = {d: self.dims[d] + other.dims[d] for d in dimensions}
#        return self
#
#    def __mul__(self, other):
#        # 5.4 µs
#        self.dims = {d: self.dims[d] * other for d in dimensions}
#        return self
    
class DimensionalExponents(dict):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

    def __add__(self, other):
        # 880 ns
        for d in dimensions:
            self[d] = self[d] + other[d]
        return self
        # 1.3 µs
        #return DimensionalExponents({k: self[k] + other[k] for k in dimensions})

    def __sub__(self, other):
        for d in dimensions:
            self[d] = self[d] - other[d]
        return self
    
    def __mul__(self, other):
        # 5.8 µs
        #if other == 0:
        #    return DimensionalExponents(empty_dimensional_dict)
        #elif isinstance(other, int):
        #    orig = self.copy()
        #    if other > 0:
        #        for i in range(other - 1):
        #            self += orig
        #    else:
        #        for i in range(other - 1):
        #            self -= orig
        #    return self
        # 5.3 µs!
        #for d in dimensions:
        #    self[d] = self[d] * other
        #return self
        # 1.3 µs, no idea why
        return DimensionalExponents({k: self[k] * other for k in dimensions})


# Function to turn a tuple or other iterable of factors into a dimension
def generate_dimensional_exponents(
        components: tuple[tuple, ...] | None = None,
        units: tuple | None = None,
    ) -> Counter:
    new_dimensional_exponents = DimensionalExponents(empty_dimensional_dict)
    if components:
        for unit, exponent in components:
            if exponent == 1:
                new_dimensional_exponents += unit.dimensional_exponents
            elif exponent == -1:
                new_dimensional_exponents -= unit.dimensional_exponents
            elif exponent == 0:
                continue
            else:
                new_dimensional_exponents += (unit.dimensional_exponents * exponent)
    elif units:
        for unit in units:
            new_dimensional_exponents += unit.dimensional_exponents
    #for unit, exponent in components:
    #    for dimension in new_dimensional_exponents.keys():
    #        if dimension in unit.dimensional_exponents:
    #            new_dimensional_exponents[dimension] += (
    #                unit.dimensional_exponents[dimension] * exponent
    #            )
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
    """Parent class for all units.
    
    Not intended for direct instantiation.
    This class defines variables and methods common to all units.

    One of `symbol` or `name` must be provided. If `symbol` is not provided, it will be
    set to the value of `name`, so that all units have a symbolic representation.
    `symbol` may be any Unicode string.
    `name` must contain only ASCII letters and digits, and underscores. It must in
    addition be a valid Python identifier, so it cannot start with a digit. The same
    rules apply to alternative names listed in `alt_names`.
    """

    __slots__ = (
        "_symbol",
        "_name",
        "_alt_names",
        "_value",
        "_value_base",
    )

    def __init__(
        self,
        symbol: str | None,
        name: str | None,
        value: Quantity | None = None,
        alt_names: list[str] | None = None,
        add_to_namespace: bool = False,
        canon_symbol: bool = False,
    ):
        self._symbol = symbol
        if symbol is None:
            # Symbol can't be canon if it wasn't even provided
            canon_symbol = False
            if name is not None:
                self._symbol = name
        # Don't raise error any more to allow lazy evaluation of symbol
        #else:
            #raise RuntimeError("Either a symbol or a name must be provided!")
        self._name = name
        self._value = value if value is not None else Quantity(1, self)
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
    def value(self) -> Quantity:
        return self._value
    
    @property
    def alt_names(self) -> list[str]:
        return self._alt_names
    
    def add_to_namespace(self, add_symbol=False):
        """Add to units namespace to allow lookup under the provided name(s)."""
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
    
    # The following are all methods that subclasses might need to redefine
    def is_dimensionless(self) -> bool:
        raise NotImplementedError

    def dimension(self) -> str:
        raise NotImplementedError

    def cancel(self):
        """Combine any like terms and return as a `Quantity`."""
        raise NotImplementedError

    def fully_cancel(self):
        """Combine any terms of the same dimension and return as a `Quantity`."""
        return self.cancel()

    def canonical(self):
        """Order terms into a reproducible order and return as a `Quantity`."""
        raise NotImplementedError
    
    def base(self):
        """Return the unit's value in base units as a `Quantity`.
        
        This is always returned in a fully cancelled, canonical form.
        """
        if not hasattr(self, "_value_base"):
            return self.value.base()
        else:
            return self._value_base


class LinearUnit(Unit):
    """Represents units on linear scales (i.e. most units).

    These units can be multiplied with numbers, units, and quantities in a linear
    fashion and can form part of compound units.
    
    If a unit only has a single base dimension without exponents, that dimension can
    be passed as `dimension` as one of the strings "X" (for dimensionless), "M" (mass),
    "L" (length), "T" (time), "I" (electric current), "Θ" (thermodynamic temperature),
    "N" (amount of substance), or "J" (luminous intensity).
    If a unit's dimension comprises multiple base dimensions or exponents, they should
    be passed as `dimensional_exponents` as a dict of the form `{"L": 1, "M": 2, ...}`,
    (only those with non-zero exponents are required), or as the analogous `Counter`.
    """

    __slots__ = ("_components", "_dimensional_exponents")

    def __init__(
        self,
        symbol: str | None,
        name: str | None,
        components: tuple[tuple, ...] | None = None,
        value: Quantity | None = None,
        dimension: str | None = None,
        dimensional_exponents: DimensionalExponents | None = None,
        alt_names: list[str] | None = None,
        add_to_namespace: bool = False,
        canon_symbol: bool = False,
    ):
        super().__init__(
            symbol=symbol,
            name=name,
            value=value,
            alt_names=alt_names,
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
        )

        #if isinstance(dimensional_exponents, Counter):
        #    self._dimensional_exponents = dimensional_exponents
        if dimensional_exponents:
            self._dimensional_exponents = dimensional_exponents
        elif dimension == "X":
            self._dimensional_exponents = DimensionalExponents(empty_dimensional_dict)
        elif isinstance(dimension, str) and len(dimension) == 1:
            self._dimensional_exponents = DimensionalExponents(empty_dimensional_dict)
            self._dimensional_exponents[dimension] = 1
        else:
            self._dimensional_exponents = None

        self._components = components

    @property
    def symbol(self) -> str:
        if self._symbol is None:
            self._symbol = generate_symbol(self.components)
        return self._symbol

    @property
    def dimensional_exponents(self) -> DimensionalExponents:
        if self._dimensional_exponents is None:
            self._dimensional_exponents = generate_dimensional_exponents(self.components)
        return self._dimensional_exponents

    @property
    def components(self) -> tuple[tuple, ...]:
        return self._components

    def __repr__(self):
        return f"{self.__class__.__name__}({self.symbol})"

    def __str__(self):
        return self.symbol

    # Units must always come at the end of expressions,
    # so do not define Unit * num or Unit / num
    # TODO * and / with a Quantity drops the uncertainty!
    def __mul__(self, other, concatenate_symbols: bool = False):
        if isinstance(other, UnitlessUnit):
            if concatenate_symbols and not other._drop_on_concat:
                return CompoundUnit(units=(self, other), concatenate_symbols=True)
            else:
                return self
        elif isinstance(other, LinearUnit):
            if concatenate_symbols:
                return CompoundUnit(units=(self, other), concatenate_symbols=True)
            else:
                return CompoundUnit(self.components + other.components, (self, other))
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
        elif isinstance(other, LinearUnit):
            return CompoundUnit(self.components + other.components_inverse())
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
        elif other == 0:
            return unitless
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
    # These are fallback methods, subclasses often override these for various reasons
    # Linear units must always hash and compare equal to quantities with an equal value
    def __hash__(self):
        return hash(self.value)

    def __eq__(self, other):
        if isinstance(other, (LinearUnit, Quantity)):
            # Compare the values (handled by `Quantity`)
            return self.value == other.value
        else:
            return NotImplemented

    def __gt__(self, other):
        if isinstance(other, (LinearUnit, Quantity)):
            # Compare the values (handled by `Quantity`)
            return self.value > other.value
        else:
            return NotImplemented

    def __ge__(self, other):
        if isinstance(other, (LinearUnit, Quantity)):
            # Compare the values (handled by `Quantity`)
            return self.value >= other.value
        else:
            return NotImplemented

    def components_inverse(self):
        """Return the inverse of the unit but just as its components."""
        #return self.inverse().components
        return tuple(((unit, -exponent) for unit, exponent in self.components),)

    def inverse(self):
        """Return the inverse of the unit as a CompoundUnit."""
        #return self ** -1
        return CompoundUnit(self.components_inverse())

    def is_dimensionless(self) -> bool:
        if self.dimensional_exponents == empty_dimensional_dict:
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


class BaseUnit(LinearUnit):
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
        dimension: str | None = None,
        alt_names: list[str] = None,
        add_to_namespace: bool = True,
        canon_symbol: bool = True,
    ):
        super().__init__(
            symbol,
            name,
            components=((self, 1),),
            dimension=dimension,
            alt_names=alt_names,
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
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
        """Combine any like terms and return as a `Quantity`.
        
        For a `BaseUnit`, simply returns a `Quantity` of unity times the `BaseUnit`.
        """
        return self.value

    def canonical(self) -> Quantity:
        """Order terms into a reproducible order and return as a `Quantity`.
        
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
        alt_names: list[str] | None = None,
        add_to_namespace: bool = True,
        canon_symbol: bool = True,
        drop: bool = False,
    ):
        super().__init__(
            symbol=symbol,
            name=name,
            dimension="X",
            alt_names=alt_names,
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
        )
        self._drop = drop

    def __mul__(self, other, concatenate_symbols: bool = False):
        if self._drop and isinstance(other, (LinearUnit, Quantity)):
            return other
        else:
            return super().__mul__(other, concatenate_symbols=concatenate_symbols)

    def __rmul__(self, other):
        if self._drop and isinstance(other, (LinearUnit, Quantity)):
            return other
        else:
            return super().__rmul__(other)

    def __truediv__(self, other):
        if self._drop and isinstance(other, (LinearUnit, Quantity)):
            return other.inverse()
        else:
            return super().__truediv__(other)

    def __rtruediv__(self, other):
        if self._drop and isinstance(other, (LinearUnit, Quantity)):
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
    alt_names=None,
    add_to_namespace=True,
    canon_symbol=False,
    drop=True,
)


class CompoundUnit(LinearUnit):
    """An effective unit created through multiplication of non-compound units.

    Multiple units multiplied together are treated as a single `Unit` object with its
    constituent parts gathered under `components`.
    Generally, the constituent units are passed as a tuple of factors (tuples of the
    form `(unit, exponent)`).
    Alternatively, a tuple of `Unit` objects can be passed and the `components`
    attributes of each will be combined automatically.
    """

    __slots__ = ("_symbol_sort", "_symbol_inverse")

    def __init__(
        self,
        components: tuple[tuple, ...] | None = None,
        units: tuple[Unit] | None = None,
        name: str | None = None,
        alt_names: list[str] | None = None,
        add_to_namespace: bool = False,
        symbol_sort: str = "sign",
        symbol_inverse: str = quanfig.INVERSE_UNIT,
        concatenate_symbols: bool = False,
        is_canon_base: bool = False,
    ):
        # If no components passed, first get components from list of units
        if components is None:
            # Repeated addition of tuples, starting with (), fastest method for low n (n < 10)
            components = sum([unit.components for unit in units], ())
        
        # Evaluate lazily, not immediately
        #if units is not None:
        #    dimensional_exponents = generate_dimensional_exponents(units=units)
        #else:
        #dimensional_exponents = generate_dimensional_exponents(components)

        if (units is not None) and (concatenate_symbols):
            # Maintain visual separation of combined units in symbol
            # Put each unit's symbol in parentheses if they contain a slash
            symbols = []
            for unit in units:
                if "/" in unit.symbol:
                    symbols.append("(" + unit.symbol + ")")
                else:
                    symbols.append(unit.symbol)
            symbol = " ".join(symbols)
        else:
            # Evaluate lazily
            symbol = None
        
        alt_names = alt_names if alt_names else None
        self._symbol_sort = symbol_sort
        self._symbol_inverse = symbol_inverse

        # Don't define a name etc., just a symbol and the components
        super().__init__(
            symbol=symbol,
            name=name,
            alt_names=alt_names,
            components=components,
            dimensional_exponents=None,
            add_to_namespace=add_to_namespace,
        )
        # Express the unit in terms of base units
        if is_canon_base:
            self._value_base = self._value
        else:
            self._value_base = None

    @property
    def symbol(self) -> str:
        if self._symbol is None:
            self._symbol = generate_symbol(
                self.components,
                self._symbol_sort,
                self._symbol_inverse
            )
        return self._symbol

    def __hash__(self) -> int:
        # Make the hashing faster by doing it directly, since we know that doing
        # self.value.base() would just give self._value_base, and we've possibly
        # already calculated that
        base_ids = ((id(u), e) for u, e in self.base().unit.components)
        return hash((self.base().number, *base_ids))

    def __eq__(self, other):
        return hash(self) == hash(other)

    def _cancel_to_unit(self, force_drop_unitless=False) -> LinearUnit:
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
        """Combine any like terms and return as a `Quantity`.
        
        Note that "like" means that the units are equivalent in value, not that they are
        the same Unit object.
        Terms of `Unitless` units which have `drop = True` will also be dropped; this is
        the case for `quanstants.units.unitless`, but not for `radian` or `steradian`.
        """
        return Quantity(1, self._cancel_to_unit(force_drop_unitless=force_drop_unitless))

    def fully_cancel(self) -> Quantity:
        """Combine any terms of the same dimension and return as a `Quantity`.
        
        Units of the same dimension are converted to whichever unit is a base unit, and
        otherwise to whichever occurs first.
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
        # Do without creating any intermediate compound units.
        # Check to see if it has been pre-calculated
        # Drop unitless units, cancel like terms, and put in canonical order so that
        # different units with equal values give _identical_ results.
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
    

class DerivedUnit(LinearUnit):
    """Units derived from and defined with SI units.

    `value` is a `Quantity` with both a number and a `Unit`, and optionally, an
    uncertainty.
    A `symbol` must be provided, but a `name` is optional.
    If a name is given and `add_to_namespace` is `True` (default), the unit will be
    added to `quanstants.units` under that name (note that trying to replace an existing
    unit with that name will raise an error).
    The `dimensional_exponents` are set to that of the provided value's unit(s).
    """

    __slots__ = ()

    def __init__(
        self,
        symbol: str,
        name: str,
        value: Quantity,
        alt_names: list[str] | None = None,
        add_to_namespace: bool = True,
        canon_symbol: bool = False,
        
    ):
        super().__init__(
            symbol,
            name,
            components=((self, 1),),
            value=value,
            dimensional_exponents=value.unit.dimensional_exponents,
            alt_names=alt_names,
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
        )
        self._value_base = self.value.base()

    def cancel(self) -> Quantity:
        return 1 * self

    def canonical(self) -> Quantity:
        return 1 * self
