from decimal import Decimal as dec
import math

from .config import quanfig
from .uncertainties import get_uncertainty
from .unit import Unit, unitless
from .quantity import Quantity, MismatchedUnitsError
from .prefix import Prefix, AlreadyPrefixedError
from .units.base import *


class LogarithmicUnit(Unit):
    """A unit on a logarithmic scale, typically relative to a reference point.
    
    A quantity can be expressed in the unit as:
    `prefactor * log(value / reference, log_base) * unit` 
    """
    
    # Most things typically possible with units shouldn't be allowed,
    # so don't subclass Unit as there's so much that would have to be
    # overridden (at least for now)

    __slots__ = (
        "_suffix",
        "_log_base",
        "_prefactor",
        "_reference",
    )

    def __init__(
        self,
        symbol: str | None,
        suffix: str | None,
        name: str | None,
        log_base: str | int | float | dec,
        prefactor: str | int | float | dec | None = None,
        reference: Quantity | None = None,
        alt_names: list | None = None,
        add_to_namespace: bool = False,
        canon_symbol: bool = False,
    ):
        self._suffix = suffix
        self._log_base = "e" if log_base == "e" or log_base is None else float(log_base) if isinstance(log_base, (str, dec, float)) else log_base
        self._prefactor = dec("1") if prefactor is None else prefactor if isinstance(prefactor, dec) else prefactor
        self._reference = reference
        super().__init__(
            symbol=symbol,
            name=name,
            value=LogarithmicQuantity(1, self),
            alt_names=alt_names,
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
        )
    
    @property
    def suffix(self) -> str:
        return self._suffix

    @property
    def log_base(self) -> str | int | float:
        return self._log_base
    
    @property
    def prefactor(self) -> int | float:
        return self._prefactor

    @property
    def reference(self) -> Quantity:
        if self._reference is None:
            return Quantity(1, unitless)
        else:
            return self._reference

    def __repr__(self):
        if self._reference is None:
            return f"LogarithmicUnit({self.symbol}, reference=1)"
        else:
            return f"LogarithmicUnit({self.symbol}, reference=({self.reference}))"

    def __str__(self):
        if quanfig.LOGARITHMIC_UNIT_STYLE == "SIMPLE" or self._reference is None:
            return self.symbol
        elif quanfig.LOGARITHMIC_UNIT_STYLE == "REFERENCE":
            return f"{self.symbol} ({self.reference})"
        elif quanfig.LOGARITHMIC_UNIT_STYLE == "SUFFIX":
            return f"{self.symbol}{self.suffix}"
    
    # Allow prefixes
    def __rmul__(self, other):
        if isinstance(other, Prefix):
            return PrefixedLogarithmicUnit
        else:
            return NotImplemented

    # Use U @ Q for referencing
    def __matmul__(self, other):
        if isinstance(other, Quantity):
            return self.with_reference(other)
        else:
            return NotImplemented

    # Use num @ U for quantity creation
    def __rmatmul__(self, other):
        if isinstance(other, (str, int, float, dec)):
            return LogarithmicQuantity(other, self)
        else:
            return NotImplemented
    
    def from_absolute(self, other: Quantity):
        """Convert an absolute `Quantity` to a relative `LogarithmicQuantity` with this unit.

        When `Quantity.on_scale()` is called on a quantity and the target unit is an instance of
        `LogarithmicUnit`, this method of the target unit will be called.
        """
        return LogarithmicQuantity.from_absolute(self, other)

    def with_reference(self, reference: Quantity):
        return type(self)(
            symbol=self.symbol,
            suffix=self.suffix,
            name=None,
            log_base=self.log_base,
            prefactor=self.prefactor,
            reference=reference,
            )
    

class PrefixedLogarithmicUnit(LogarithmicUnit):
    """A unit created through the combination of a `LogarithmicUnit` with a `Prefix`.

    For now acts almost exactly like a `LogarithmicUnit` except that it cannot be prefixed.
    """

    __slots__ = ("_prefix", "_unit")

    def __init__(
        self,
        prefix: Prefix,
        unit: LogarithmicUnit,
        alt_names: list | None = None,
        add_to_namespace: bool = False,
        canon_symbol: bool = False,
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
        # Determine new prefactor
        prefactor = unit.prefactor / prefix.multiplier
        super().__init__(
            symbol=concat_symbol,
            suffix=unit.suffix,
            name=concat_name,
            log_base=unit.log_base,
            prefactor=prefactor,
            reference=unit._reference,
            alt_names=concat_alt_names,
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
        )

    @property
    def prefix(self):
        return self._prefix

    @property
    def unit(self):
        return self._unit
    
    # Prevent prefixing again
    def __rmul__(self, other):
        if isinstance(other, Prefix):
            raise AlreadyPrefixedError
        else:
            super().__rmul__(other)


class LogarithmicQuantity(Quantity):
    """Represents quantities on a logarithmic scale relative to some reference quantity.
    
    `quanstants` does not support asymmetric uncertainties. If an uncertainty is
    provided, it must be an absolute quantity rather than a logarithmic one.

    If `number` is not passed, but `unit` is a referenced `LogarithmicUnit` and `value`
    is a `Quantity` with the same unit as that of `unit.reference`, the absolute value
    will be converted to a value on the scale.
    """

    __slots__ = ()

    def __init__(
        self,
        number: str | int | float | dec | None = None,
        unit: LogarithmicUnit | None = None,
        uncertainty: Quantity | None = None,
        value: Quantity | None = None,
    ):
        super().__init__(
            number=number,
            unit=unit,
            uncertainty=None,
            value=value,
        )
        # If no value was provided, overwrite now
        # The value of a logarithmic quantity should always be stored as an absolute
        # quantity in the units of the reference
        if value is None:
            self._value = self.to_absolute()
        self._uncertainty = dec("0") if uncertainty is None else uncertainty

    @property
    def uncertainty(self):
        # Override because here we store uncertainty as a quantity, not just a number
        if not self._uncertainty:
            return Quantity(0, unitless)
        else:
            return self._uncertainty
    
    def __repr__(self):
        as_quantity = super().__repr__()
        return as_quantity.replace("Quantity", "LogarithmicQuantity")
    
    def __str__(self):
        if not self._uncertainty:
            return f"{self.number} {self.unit}"
        else:
            return f"{self.number} {self.unit} ± {self.uncertainty}"
        
    # Equality functions of `Quantity` call `.base()` anyway, so super handles fine

    @classmethod
    def from_absolute(cls, unit: LogarithmicUnit, quantity: Quantity):
        """Convert a `Quantity` to the logarithmic scale defined by the given unit."""
        if quantity.unit.dimensional_exponents != unit.dimensional_exponents:
            raise MismatchedUnitsError("Ratio of quantity to reference value is not dimensionless!")
        elif unit.log_base == 10:
            new_number = unit.prefactor * math.log10((quantity / unit.reference).number)
        elif unit.log_base == "e":
            new_number = unit.prefactor * math.log((quantity / unit.reference).number)
        elif unit.log_base == 2:
            new_number = unit.prefactor * math.log2((quantity / unit.reference).number)
        else:
            new_number = unit.prefactor * math.log((quantity / unit.reference).number, unit.log_base)
        return cls(new_number, unit, quantity.uncertainty)

    def to_absolute(self):
        return self.unit.reference * self.unit.log_base ** (self.number / self.unit.prefactor)

    def cancel(self):
        """Combine any like terms in the unit.

        Has no effect for a LogarithmicQuantity.
        """
        return self

    def fully_cancel(self):
        """Combine any like terms in the unit, with units of the same dimension converted and combined.

        Has no effect for a LogarithmicQuantity.
        """
        return self

    def canonical(self):
        """Express with its units in a canonical order.

        Has no effect for a LogarithmicQuantity.
        """
        return self

    def base(self):
        """Return the absolute value expressed in terms of base units.
        
        The unit is always returned in a fully cancelled, canonical form.
        """
        return self.value.base()
    
    def to(self, other):
        return self.value.to(other)

    def on_scale(self, other):
        return self.value.on_scale(other)