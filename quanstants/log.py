from decimal import Decimal as dec

from .config import quanfig
from .uncertainties import get_uncertainty
from .unit import Unit
from .quantity import Quantity
from .si import *


class LogarithmicUnit:
    """A unit on a logarithmic scale, typically relative to a reference point.
    
    A quantity can be expressed in the unit as:
    `preexp * log(value / reference, log_base) * unit` 
    """
    
    # Most things typically possible with units shouldn't be allowed,
    # so don't subclass Unit as there's so much that would have to be
    # overridden (at least for now)

    __slots__ = (
        "_symbol",
        "_suffix",
        "_name",
        "_log_base",
        "_prefactor",
        "_reference",
        "_alt_names",
    )

    def __init__(
        self,
        symbol: str | None,
        suffix: str | None,
        name: str | None,
        log_base: str | int | float | dec,
        prefactor: str | int | float | dec,
        reference: Quantity | None = None,
        add_to_namespace: bool = True,
        canon_symbol: bool = False,
        alt_names: list | None = None,
    ):
        if symbol is not None:
            self._symbol = symbol
        elif name is not None:
            self._symbol = name
            # Symbol can't be canon if it wasn't even provided
            canon_symbol = False
        else:
            raise RuntimeError("Either a symbol or a name must be provided!")
        self._suffix = suffix
        self._name = name
        self._log_base = "e" if log_base == "e" or log_base is None else float(log_base) if isinstance(log_base, (str, dec, float)) else log_base
        self._prefactor = 1 if prefactor is None else float(prefactor) if isinstance(prefactor, (str, dec, float)) else prefactor
        self._reference = reference
        self._alt_names = tuple(alt_names) if alt_names is not None else None
        if add_to_namespace:
            self.add_to_namespace(add_symbol=canon_symbol)
        
    add_to_namespace = Unit.add_to_namespace

    @property
    def symbol(self) -> str:
        return self._symbol
    
    @property
    def suffix(self) -> str:
        return self._suffix

    @property
    def name(self) -> str | None:
        return self._name

    @property
    def log_base(self) -> str | int | float:
        return self._log_base
    
    @property
    def prefactor(self) -> int | float:
        return self._prefactor

    @property
    def reference(self) -> Quantity:
        return self._reference

    @property
    def alt_names(self) -> list[str]:
        return self._alt_names

    def __repr__(self):
        return f"LogarithmicUnit({self})"

    def __str__(self):
        if quanfig.LOGARITHMIC_UNIT_STYLE == "SIMPLE" or self.reference is None:
            return self.symbol
        elif quanfig.LOGARITHMIC_UNIT_STYLE == "REFERENCE":
            return f"{self.symbol} ({self.reference})"
        elif quanfig.LOGARITHMIC_UNIT_STYLE == "SUFFIX":
            return f"{self.symbol}{self.suffix}"