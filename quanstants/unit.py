from decimal import Decimal as dec
from fractions import Fraction as frac

from .config import quanfig
from .quantity import Quantity
from .abstract_unit import AbstractUnit
from .dimensions import Dimensions, generate_dimensions
from .unicode import generate_symbol


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


class Unit(AbstractUnit):
    """Represents units on linear scales (i.e. most units).

    These units can be multiplied with numbers, units, and quantities in a linear
    fashion and can form part of compound units.
    
    If a unit only has a single base dimension without exponents, that dimension can
    be passed as `dimensions` as one of the strings "X" (for dimensionless), "M" (mass),
    "L" (length), "T" (time), "I" (electric current), "Θ" (thermodynamic temperature),
    "N" (amount of substance), or "J" (luminous intensity).
    If a unit's dimensions comprise multiple base dimensions or exponents, they should
    be passed as `dimensions` a `Dimensions` object or as an equivalent dict of the form
    `{"L": 1, "M": 2, ...}`, in which all seven base dimensions must be specified.
    """

    __slots__ = ("_components", "_dimensions", "_value", "_value_base")

    def __init__(
        self,
        symbol: str | None,
        name: str | None,
        components: tuple[tuple, ...] | None = None,
        value: Quantity | None = None,
        dimensions: Dimensions | dict | str | None = None,
        alt_names: list[str] | None = None,
        add_to_namespace: bool = False,
        canon_symbol: bool = False,
        **kwargs,
    ):
        super().__init__(
            symbol=symbol,
            name=name,
            alt_names=alt_names,
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
            **kwargs,
        )

        self._value = value if value is not None else Quantity(1, self)

        if isinstance(dimensions, Dimensions):
            self._dimensions = dimensions
        elif isinstance(dimensions, dict):
            self._dimensions = Dimensions(dimensions)
        elif dimensions == "X":
            self._dimensions = Dimensions()
        elif isinstance(dimensions, str) and len(dimensions) == 1:
            self._dimensions = Dimensions()
            self._dimensions[dimensions] = 1
        else:
            self._dimensions = None

        self._components = components

    @property
    def symbol(self) -> str:
        if self._symbol is None:
            self._symbol = self._name if self._name else "(no symbol)"
        return self._symbol
    
    @property
    def value(self) -> Quantity:
        return self._value

    @property
    def dimensions(self) -> Dimensions:
        if self._dimensions is None:
            self._dimensions = generate_dimensions(self.components)
        return self._dimensions

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
        elif isinstance(other, Unit):
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
        elif isinstance(other, Unit):
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

    def components_inverse(self):
        """Return the inverse of the unit but just as its components."""
        #return self.inverse().components
        return tuple(((unit, -exponent) for unit, exponent in self.components),)

    def inverse(self):
        """Return the inverse of the unit as a CompoundUnit."""
        #return self ** -1
        return CompoundUnit(self.components_inverse())

    def is_dimensionless(self) -> bool:
        if self.dimensions == Dimensions._dimensionless:
            return True
        else:
            return False
    
    def _cancel_to_unit(self, force_drop_unitless: bool = False):
        """Does everything that `self.cancel() does, but returns a `Unit`."""
        raise NotImplementedError

    def cancel(self, force_drop_unitless: bool = False) -> Quantity:
        """Combine any like terms and return as a `Quantity`.
        
        Note that "like" means that the units are equivalent in value, not that they are
        the same Unit object.

        Terms of `Unitless` units which have `drop = True` will also be dropped; this is
        the case for `quanstants.units.unitless`, but not for `radian` or `steradian`.
        Passing `force_drop_unitless = True` will cause these to be dropped too.
        """
        return Quantity(1, self._cancel_to_unit(force_drop_unitless=force_drop_unitless))

    def fully_cancel(self) -> Quantity:
        """Combine any terms with the same dimensions and return as a `Quantity`.
        
        Component units with the same dimensions are converted to whichever unit is a
        base unit, or otherwise to whichever occurs first.

        Any terms of `Unitless` units (i.e. equal to 1) will also be dropped.
        In contrast to `cancel()`, this means even those for which `drop = False`, like
        `radian` and `steradian`, will be dropped.
        """
        return self.cancel()

    def canonical(self):
        """Order terms into a reproducible order and return as a `Quantity`."""
        raise NotImplementedError

    def base(self) -> Quantity:
        """Return the unit's value in base units as a `Quantity`.
        
        This is always returned in a fully cancelled, canonical form.
        """
        # Check for cached value
        if not hasattr(self, "_value_base"):
            self._value_base = self.value.base()
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
        dimensions: str | None = None,
        alt_names: list[str] = None,
        add_to_namespace: bool = True,
        canon_symbol: bool = True,
        **kwargs,
    ):
        super().__init__(
            symbol,
            name,
            components=((self, 1),),
            dimensions=dimensions,
            alt_names=alt_names,
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
            **kwargs,
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
        
    def _cancel_to_unit(self) -> Unit:
        return self

    def canonical(self) -> Quantity:
        return self.value


class UnitlessUnit(BaseUnit):
    """Special dimensionless units that are numerically equal to 1.
    
    Acts similarly to a BaseUnit in some ways, but in arithmetic and equalities behaves
    like unity.
    
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
            dimensions=Dimensions(),
            alt_names=alt_names,
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
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
    
    def _cancel_to_unit(self, force_drop_unitless: bool = False) -> Unit:
        if self._drop:
            return unitless
        else:
            return self
    
    def fully_cancel(self) -> Quantity:
        return unitless._value
    
    def base(self) -> Quantity:
        #return Quantity(1, unitless)
        # No need to create a new quantity object for this
        return unitless._value


# Instantiate the main UnitlessUnit instance which is the one typically used internally
unitless = UnitlessUnit(
    symbol="(unitless)",
    name="unitless",
    alt_names=None,
    add_to_namespace=True,
    canon_symbol=False,
    drop=True,
)


class CompoundUnit(Unit):
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
        units: tuple[AbstractUnit] | None = None,
        name: str | None = None,
        alt_names: list[str] | None = None,
        add_to_namespace: bool = False,
        symbol_sort: str = "sign",
        symbol_inverse: str = None,
        concatenate_symbols: bool = False,
        is_canon_base: bool = False,
    ):
        # If no components passed, first get components from list of units
        if components is None:
            # Repeated addition of tuples, starting with (), fastest method for low n (n < 10)
            components = sum([unit.components for unit in units], ())
        
        # Evaluate lazily, not immediately
        # NOTE This same logic is now done in LinearUnit.__init__()
        #if units is not None:
        #    dimensions = generate_dimensions(units=units)
        #else:
        #dimensions = generate_dimensions(components)

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
        self._symbol_inverse = quanfig.INVERSE_UNIT if symbol_inverse is None else symbol_inverse

        # Don't define a name etc., just a symbol and the components
        super().__init__(
            symbol=symbol,
            name=name,
            alt_names=alt_names,
            components=components,
            dimensions=None,
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
        base = self.base()
        base_ids = ((id(u), e) for u, e in base.unit.components)
        return hash((base.number, *base_ids))

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
        """Combine any like terms and return as a `Quantity`.
        
        Note that "like" means that the units are equivalent in value, not that they are
        the same Unit object.

        Terms of `Unitless` units which have `drop = True` will also be dropped; this is
        the case for `quanstants.units.unitless`, but not for `radian` or `steradian`.
        Passing `force_drop_unitless = True` will cause these to be dropped too.
        """
        return Quantity(1, self._cancel_to_unit(force_drop_unitless=force_drop_unitless))

    def fully_cancel(self) -> Quantity:
        """Combine any terms with the same dimensions and return as a `Quantity`.
        
        Component units with the same dimensions are converted to whichever unit is a
        base unit, or otherwise to whichever occurs first.

        Any terms of `Unitless` units (i.e. equal to 1) will also be dropped.
        In contrast to `cancel()`, this means even those for which `drop = False`, like
        `radian` and `steradian`, will be dropped.
        """
        result_number = 1
        new_components_dict = {}
        # First cancel like normal, this also gets rid of all UnitlessUnits
        cancelled = self._cancel_to_unit(force_drop_unitless=True)
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
                    if first_unit.dimensions == other[0].dimensions:
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
                if unit.dimensions == unit_already_in.dimensions:
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
    

class DerivedUnit(Unit):
    """Units derived from and defined with SI units.

    `value` is a `Quantity` with both a number and a `Unit`, and optionally, an
    uncertainty.
    A `symbol` must be provided, but a `name` is optional.
    If a name is given and `add_to_namespace` is `True` (default), the unit will be
    added to `quanstants.units` under that name (note that trying to replace an existing
    unit with that name will raise an error).
    The `dimensions` are set to that of the provided value's unit(s).
    """

    __slots__ = ("_space_rule")

    def __init__(
        self,
        symbol: str,
        name: str,
        value: Quantity,
        alt_names: list[str] | None = None,
        add_to_namespace: bool = True,
        canon_symbol: bool = False,
        preceding_space: bool = True,
        _space_rule: str | None = None,
    ):
        # Passing a quanfig variable (str) allows preceding_space to be determined by it
        if _space_rule is not None:
            preceding_space = getattr(quanfig, _space_rule)
        super().__init__(
            symbol,
            name,
            components=((self, 1),),
            value=value,
            dimensions=value.unit.dimensions,
            alt_names=alt_names,
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
            preceding_space=preceding_space,
        )
        self._value_base = self.value.base()
    
    def _cancel_to_unit(self, force_drop_unitless: bool = False) -> Unit:
        return self

    def canonical(self) -> Quantity:
        return Quantity(1, self)
