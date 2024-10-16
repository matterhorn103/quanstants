from decimal import Decimal as dec

from .quantity import Quantity
from .unit import DerivedUnit
from .format import format_quantity
from .config import quanfig

from . import constants

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

    `add_to_namespace` specifies whether the constant should be added to `quanstants.constants` under the
    provided `name` and under any alternative names given as a list as `alt_names`.
    If `canon_symbol` is set to `True`, the constant will also be added to `quanstants.constants` under `symbol`.

    Constants possess the convenient additional method `.as_unit()`, which returns a `DerivedUnit` with
    the same symbol and value as the constant. This allows the creation of quantities like:
    `300 * (qu.MeV / qc.c.as_unit()**2)`
    """

    __slots__ = ("_symbol", "_name", "_alt_names")

    def __init__(
        self,
        symbol: str | None = None,
        name: str | None = None,
        number: str | int | float | dec | None = None,
        unit=None,
        uncertainty: str | int | float | dec | None = None,
        value: str | Quantity | None = None,
        alt_names: list = None,
        add_to_namespace: bool = True,
        canon_symbol: str = False,
        **kwargs,
    ):
        if symbol is not None:
            self._symbol = symbol
        elif name is not None:
            self._symbol = name
            # Symbol can't be canon if it wasn't even provided
            canon_symbol = False
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
        if add_to_namespace:
            self.add_to_namespace(add_symbol=canon_symbol)

    # Always access properties via self.x not self._x for consistency
    # self._x is slightly faster, but even for time-critical operations it makes v little difference
    # e.g. for Quantity(2, m) * Quantity(3.4, s**-1) the time saving was only 1.5% (off ~10 μs)
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
        as_string = self.__str__()
        if self._name is not None:
            return f"Constant({as_string})"
        else:
            return f"Constant({as_string})"

    def __str__(self):
        # Don't use method of super() as we don't want to ever truncate a constant
        normal_str = format_quantity(
            self,
            truncate=0,
            group=quanfig.GROUP_DIGITS,
            group_which=quanfig.GROUP_DIGITS_STYLE,
            group_sep=quanfig.GROUP_SEPARATOR,
            uncertainty_style=quanfig.UNCERTAINTY_STYLE,
        )
        if self._name is not None:
            return f"{self.name} = " + normal_str
        else:
            return f"{self.symbol} = " + normal_str

    def add_to_namespace(self, add_symbol=False):
        # Add to namespace to allow lookup under the provided name
        if self.name is not None:
            constants.add(self.name, self)
        # Also add under any alternative names (though try to stick to one, as
        # there could be a hundred permutations of each constant's name)
        if self.alt_names is not None:
            for alt_name in self.alt_names:
                constants.add(alt_name, self)
        # Also add under the symbol if it has been indicated via canon_symbol
        # that the symbol should uniquely refer to this constant
        if (add_symbol) and (self.symbol != self.name):
            constants.add(self.symbol, self)

    def as_unit(self):
        """Return a `DerivedUnit` with the same value and symbol as the constant."""
        constant_as_unit = DerivedUnit(
            self.symbol,
            name=None,
            value=self.value,
            add_to_namespace=False,
        )
        return constant_as_unit
