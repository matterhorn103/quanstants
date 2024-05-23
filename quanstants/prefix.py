from decimal import Decimal as dec

from .config import quanfig
from .quantity import Quantity
from .unit import BaseUnit, DerivedUnit
from .units.base import kilogram

from . import prefixes

class AlreadyPrefixedError(Exception):
    pass


class Prefix:
    """An object representing a (usually metric) prefix.

    Combines with a `BaseUnit` or `DerivedUnit` to form a new `PrefixedUnit`.
    Prefixes are automatically added under both symbol and name to the prefix namespace, which
    is accessible via `quanstants.prefixes`.
    """

    __slots__ = ("_symbol", "_name", "_multiplier")

    def __init__(
        self,
        symbol,
        name,
        multiplier: str | int | float | dec,
    ):
        self._symbol = symbol
        self._name = name
        if quanfig.CONVERT_FLOAT_AS_STR:
            self._multiplier = dec(str(multiplier))
        else:
            self._multiplier = dec(multiplier)
        prefixes.add(symbol, self)
        prefixes.add(name, self)

    @property
    def symbol(self):
        return self._symbol

    @property
    def name(self):
        return self._name

    @property
    def multiplier(self):
        return self._multiplier

    def __mul__(self, other):
        if isinstance(other, (BaseUnit, DerivedUnit)):
            # Special behaviour for kilo + gram
            if (self.name == "kilo") and (other.name == "gram"):
                return kilogram
            # Make sure the user is not trying to add a second prefix to a prefixed unit
            if isinstance(other, PrefixedUnit):
                raise AlreadyPrefixedError
            # Create a new unit, don't add to `prefixes` to avoid overwrites
            return PrefixedUnit(
                prefix=self,
                unit=other,
                add_to_namespace=False,
                canon_symbol=False,
            )
        else:
            return NotImplemented


class PrefixedUnit(DerivedUnit):
    """A unit created through the combination of a `BaseUnit` or `DerivedUnit` with a `Prefix`.

    For now acts almost exactly like a `DerivedUnit` except that it cannot be prefixed.
    TODO: automatically adjust the prefix upon request (so that e.g. 2000 kJ becomes 2 MJ).
    """

    __slots__ = ("_prefix", "_unit")

    def __init__(
        self,
        prefix: Prefix,
        unit: BaseUnit | DerivedUnit,
        add_to_namespace: bool = False,
        canon_symbol: bool = False,
        alt_names: list | None = None,
    ):
        self._prefix = prefix
        self._unit = unit

        # Create prefixed symbol and name
        concat_symbol = prefix.symbol + unit.symbol
        if (prefix.name is not None) and (unit.name is not None):
            concat_name = prefix.name + unit.name
        else:
            concat_name = None
        # Automatically prefix any alternative names of the unit and add to list of alt names
        if (prefix.name is not None) and (unit.alt_names is not None):
            concat_alt_names = [] if alt_names is None else alt_names
            for alt_name in unit.alt_names:
                concat_alt_name = prefix.name + alt_name
                concat_alt_names.append(concat_alt_name)
        else:
            concat_alt_names = None if alt_names is None else alt_names
        super().__init__(
            symbol=concat_symbol,
            name=concat_name,
            value=Quantity(prefix.multiplier, unit),
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
            alt_names=concat_alt_names,
        )

    @property
    def prefix(self):
        return self._prefix

    @property
    def unit(self):
        return self._unit
