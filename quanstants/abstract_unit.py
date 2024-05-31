from abc import ABCMeta, abstractmethod

# Import the units namespace module, in which named units are registered
from . import units


class AbstractUnit(metaclass=ABCMeta):
    """Parent class for all units of all flavours.
    
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
    )

    def __init__(
        self,
        symbol: str | None,
        name: str | None,
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
        self._name = name
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
    def alt_names(self) -> list[str]:
        return self._alt_names
    
    @abstractmethod
    def is_dimensionless(self) -> bool:
        pass
    
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
