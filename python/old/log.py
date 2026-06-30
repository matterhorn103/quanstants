from decimal import Decimal as dec
import math

from .config import quanfig
from .uncertainties import get_uncertainty
from .abstract_unit import AbstractUnit
from .unit import unitless
from .abstract_quantity import AbstractQuantity
from .quantity import Quantity
from .prefix import Prefix
from .exceptions import AlreadyPrefixedError, MismatchedUnitsError
from .units.base import *


class LogarithmicUnit(AbstractUnit):
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
        add_to_namespace: bool = True,
        canon_symbol: bool = False,
    ):
        self._suffix = suffix
        self._log_base = "e" if log_base == "e" or log_base is None else float(log_base) if isinstance(log_base, (str, dec, float)) else log_base
        self._prefactor = dec("1") if prefactor is None else prefactor if isinstance(prefactor, dec) else prefactor
        self._reference = reference
        super().__init__(
            symbol=symbol,
            name=name,
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
            return PrefixedLogarithmicUnit(
                prefix=other,
                root=self,
                add_to_namespace=False,
                canon_symbol=False,
            )
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
    
    # Unlike other units, log units do not compare equal to 1 of themselves
    def __hash__(self):
        return hash((self.log_base, self.prefactor, self.reference))
    
    # Just compare hashes for now
    def __eq__(self, other):
        return hash(self) == hash(other)

    def __gt__(self, other):
        return hash(self) > hash(other)
    
    def __ge__(self, other):
        return hash(self) >= hash(other)

    def is_dimensionless(self) -> bool:
        # Technically the answer is always True but this should only return True if
        # self.base() is dimensionless
        return self.reference.is_dimensionless()
    
    def from_absolute(self, other: Quantity):
        """Convert an absolute `Quantity` to a relative `LogarithmicQuantity` with this unit.

        When `Quantity.on_scale()` is called on a quantity and the target unit is an instance of
        `LogarithmicUnit`, this method of the target unit will be called.

        If the `LogarithmicUnit` passed is unreferenced, the reference value will be
        set to be 1 of the unit of the absolute quantity.
        """
        if self._reference is None and not other.is_dimensionless():
            new_reference = Quantity(1, other.unit)
            return LogarithmicQuantity.from_absolute(self.with_reference(new_reference), other)
        else:
            return LogarithmicQuantity.from_absolute(self, other)

    def with_reference(
            self,
            reference: Quantity,
            suffix: str | None,
            name: str | None = None,
            alt_names: list | None = None,
            add_to_namespace: bool = False,
        ):
        """Create an identical unit but with the provided reference.
        
        Optionally, new name(s) can be specified; otherwise, the name(s) of the origin
        unit is kept.
        """
        new_suffix = suffix if suffix is not None else self.suffix
        new_name = name if name is not None else self.name
        return LogarithmicUnit(
            symbol=self.symbol,
            suffix=new_suffix,
            name=new_name,
            log_base=self.log_base,
            prefactor=self.prefactor,
            reference=reference,
            alt_names=alt_names,
            add_to_namespace=add_to_namespace,
            canon_symbol=False,
            )
    

class PrefixedLogarithmicUnit(LogarithmicUnit):
    """A unit created through the combination of a `LogarithmicUnit` with a `Prefix`.

    Acts almost exactly like a `LogarithmicUnit` except that it cannot be prefixed.

    If `name` is not provided, both `name` and `alt_names` will be automatically
    generated by concatenation of `prefix.name` to `unit.name` and each name in
    `unit.alt_names`.
    If `name` is provided, no prefixation will be done at all, and `name` and
    `alt_names` will be used as is.
    If `name` is not provided but `alt_names` is, both the given `alt_names` and the
    automatically generated ones will be used.
    """

    __slots__ = ("_prefix", "_root")

    def __init__(
        self,
        prefix: Prefix,
        root: LogarithmicUnit,
        name: str | None = None,
        alt_names: list | None = None,
        add_to_namespace: bool = False,
        canon_symbol: bool = False,
    ):
        self._prefix = prefix
        self._root = root

        # Create prefixed symbol
        concat_symbol = prefix.symbol + root.symbol

        # Use provided names if given, otherwise generate prefixed ones
        if name is not None:
            concat_name = name
            concat_alt_names = alt_names
        else:
            if (prefix.name is not None) and (root.name is not None):
                concat_name = prefix.name + root.name
            else:
                concat_name = None
            # Automatically prefix any alternative names of the unit and add to list of alt names
            if (prefix.name is not None) and (root.alt_names is not None):
                concat_alt_names = [] if alt_names is None else alt_names
                for alt_name in root.alt_names:
                    concat_alt_name = prefix.name + alt_name
                    concat_alt_names.append(concat_alt_name)
            else:
                concat_alt_names = None if alt_names is None else alt_names
        # Determine new prefactor
        prefactor = root.prefactor / prefix.multiplier
        super().__init__(
            symbol=concat_symbol,
            suffix=root.suffix,
            name=concat_name,
            log_base=root.log_base,
            prefactor=prefactor,
            reference=root._reference,
            alt_names=concat_alt_names,
            add_to_namespace=add_to_namespace,
            canon_symbol=canon_symbol,
        )

    @property
    def prefix(self):
        return self._prefix

    @property
    def root(self):
        return self._root
    
    # Prevent prefixing again
    def __rmul__(self, other):
        if isinstance(other, Prefix):
            raise AlreadyPrefixedError
        else:
            super().__rmul__(other)
    
    def with_reference(
            self,
            reference: Quantity,
            suffix: str | None = None,
            name: str | None = None,
            alt_names: list | None = None,
            add_to_namespace: bool = False,
            ):
        new_suffix = suffix if suffix is not None else self.suffix
        return PrefixedLogarithmicUnit(
            prefix=self.prefix,
            root=self.root.with_reference(reference, new_suffix),
            name=name,
            alt_names=alt_names,
            add_to_namespace=add_to_namespace,
            canon_symbol=False,
            )


class LogarithmicQuantity(AbstractQuantity):
    """Represents quantities on a logarithmic scale relative to some reference quantity.
    
    `quanstants` does not support asymmetric uncertainties. If an uncertainty is
    provided, it must be an absolute quantity rather than a logarithmic one.

    If `number` is not passed, but `unit` is a referenced `LogarithmicUnit` and `value`
    is a `Quantity` with the same unit as that of `unit.reference`, the absolute value
    will be converted to a value on the scale.

    If both a `number` and a `value` are passed, `value` will be completely ignored.
    """

    __slots__ = ("_reference")

    def __init__(
        self,
        number: str | int | float | dec | None = None,
        unit: LogarithmicUnit | None = None,
        uncertainty: Quantity | None = None,
        value: Quantity | None = None,
        **kwargs,
    ):  
        # Ignore value if number was provided
        if (number is not None):
            value = None
        # If only unit and value were provided, defer to `from_absolute()`
        if (number is None) and (unit is not None) and (value is not None):
            converted = LogarithmicQuantity.from_absolute(unit, value)
            number, uncertainty = converted.number, converted.uncertainty
        super().__init__(
            number=number,
            unit=unit,
            uncertainty=None,
            **kwargs,
        )
        # Assuming no value was provided, overwrite now
        # The value of a logarithmic quantity should always be stored as an absolute
        # quantity in the units of the reference
        if value is None:
            self._value = self.to_absolute()
        else:
            self._value = value
        self._uncertainty = dec(0) if uncertainty is None or uncertainty == 0 else uncertainty

    @property
    def uncertainty(self) -> Quantity:
        return self._uncertainty

    @property
    def uncertainty(self):
        # Override because here we store uncertainty as a quantity, not just a number
        if not self._uncertainty:
            return Quantity(0, unitless)
        else:
            return self._uncertainty
    
    @property
    def value(self):
        return self._value
    
    @property
    def reference(self):
        return self.unit.reference
    
    def __repr__(self):
        result = super().__repr__()
        if "uncertainty" in result:
            result = result.replace("uncertainty=", "uncertainty=(") + ")"
        return result
    
    def __str__(self):
        if not self._uncertainty:
            return f"{self.number} {self.unit}"
        else:
            return f"{self.number} {self.unit} ± {self.uncertainty}"

    # For now, only allow arithmetic between quantities on the same scale 
    def __add__(self, other, correlation=0):
        if isinstance(other, LogarithmicQuantity) and self.unit == other.unit:
            return self.to_absolute().__add__(
                other.to_absolute(), correlation=correlation
            ).on_scale(self.unit)
        elif isinstance(other, Quantity):
            raise MismatchedUnitsError("Arithmetic is only possible between logarithmic quantities on the same scale!")
        else:
            return NotImplemented
        
    def __sub__(self, other, correlation=0):
        if isinstance(other, LogarithmicQuantity) and self.unit == other.unit:
            return self.to_absolute().__sub__(
                other.to_absolute(), correlation=correlation
            ).on_scale(self.unit)
        elif isinstance(other, Quantity):
            raise MismatchedUnitsError("Arithmetic is only possible between logarithmic quantities on the same scale!")
        else:
            return NotImplemented
    
    def __mul__(self, other, correlation=0):
        if isinstance(other, LogarithmicQuantity) and self.unit == other.unit:
            result = LogarithmicQuantity(self.number + other.number, self.unit)
            if not self._uncertainty and not other._uncertainty:
                return result
            else:
                new_uncertainty = get_uncertainty(
                    result.value.number,
                    "mul",
                    self.value,
                    other.value,
                    correlation=correlation,
                )
            return result.with_uncertainty(new_uncertainty)
        elif isinstance(other, Quantity):
            raise MismatchedUnitsError("Arithmetic is only possible between logarithmic quantities on the same scale!")
        else:
            return NotImplemented
    
    def __truediv__(self, other, correlation=0):
        if isinstance(other, LogarithmicQuantity) and self.unit == other.unit:
            result = LogarithmicQuantity(self.number - other.number, self.unit)
            if not self._uncertainty and not other._uncertainty:
                return result
            else:
                new_uncertainty = get_uncertainty(
                    result.value.number,
                    "div",
                    self.value,
                    other.value,
                    correlation=correlation,
                )
            return result.with_uncertainty(new_uncertainty)
        elif isinstance(other, Quantity):
            raise MismatchedUnitsError("Arithmetic is only possible between logarithmic quantities on the same scale!")
        else:
            return NotImplemented

    def __hash__(self):
        return hash(self.value)

    def __eq__(self, other):
        return self.value == other

    def __gt__(self, other):
        return self.value > other

    def __ge__(self, other):
        return self.value >= other

    @classmethod
    def from_absolute(cls, unit: LogarithmicUnit, quantity: Quantity):
        """Convert a `Quantity` to the logarithmic scale defined by the given unit."""
        ratio = quantity.base() / unit.reference.base()
        if not ratio.is_dimensionless():
            raise MismatchedUnitsError("Ratio of quantity to reference value is not dimensionless!")
        elif unit.log_base == 10:
            new_number = unit.prefactor * ((ratio).number).log10()
        elif unit.log_base == "e":
            new_number = unit.prefactor * ((ratio).number).ln()
        elif unit.log_base == 2:
            new_number = unit.prefactor * dec(math.log2((ratio).number))
        else:
            new_number = unit.prefactor * dec(math.log((ratio).number, unit.log_base))
        return cls(new_number, unit, quantity.uncertainty, value=quantity)

    def to_absolute(self):
        """Convert to an absolute `Quantity` expressed in the units of the reference."""
        if self.unit.log_base == "e":
            return (self.unit.reference * dec(math.e) ** (self.number / self.unit.prefactor)).with_uncertainty(self.uncertainty)
        else:
            return (self.unit.reference * self.unit.log_base ** (self.number / self.unit.prefactor)).with_uncertainty(self.uncertainty)

    # Disallow rounding to uncertainty since units don't match
    def round_to_uncertainty(self, ndigits=None, pad=None, mode=None):
        return MismatchedUnitsError(
            "A LogarithmicQuantity has an absolute uncertainty so the precisions are not comparable."
        )

    # TODO Come up with some better way
    def resolution(self):
        return LogarithmicQuantity(10 ** self.number.as_tuple().exponent, self._unit)

    def cancel(self):
        return self

    def fully_cancel(self):
        return self

    def canonical(self):
        return self

    def base(self):
        return self.value.base()
    
    def to(self, other):
        return self.value.to(other)

    def on_scale(self, other):
        return self.value.on_scale(other)