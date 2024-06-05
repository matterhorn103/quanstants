from abc import ABCMeta, abstractmethod
from decimal import Decimal as dec
from typing import Self

from .config import quanfig
from .exceptions import MismatchedUnitsError
from .format import format_number, format_quantity
from . import units
from . import rounding


class AbstractQuantity(metaclass=ABCMeta):
    """Parent class for all quantities of all flavours, both absolute and relative.

    The quantity is expressed as a numerical value `number` and a unit of measurement
    `unit`, with an optional associated `uncertainty`, which is also a numerical value.
    `number` is any type that can be converted to `Decimal`, including strings.
    `unit` is any subclass of `AbstractUnit`.
    `uncertainty` is also any type that can be converted to `Decimal`, including
    strings, however if `uncertainty` is not specified or given as `0` or `None`
    (default) or anything else "falsy" then the quantity`s uncertainty is set to `0`.
    `uncertainty` may also be a `Quantity` itself, which is useful if it was calculated
    separately.

    Normally, `number` and `unit` are both required. To create a unitless quantity, the
    special unitless unit (found in the main units namespace i.e. under
    `quanstants.units.unitless`) should be provided.

    Alternatively, all three may be left as `None` and `value` may be specified instead
    as either a string or another `Quantity`. Passing a string simply invokes
    `Quantity.parse(value)`.

    If only a single string is passed to `number`, and no unit is provided, the string
    will be parsed by `Quantity.parse()`, so if it contains both a number and a unit, an
    appropriate `Quantity` will be created.

    However, while both the above work, the preferred method for quantity creation from
    a single string is to call `Quantity.parse(string)`.
    """

    __slots__ = ("_number", "_unit", "_uncertainty", "_value", "_pending_cancel", "_is_normalized")

    def __init__(
        self,
        number: str | int | float | dec | None = None,
        unit=None,
        uncertainty: str | int | float | dec | Self | None = None,
        _pending_cancel: bool = False,
    ):

        # Did extensive timings for this, shown in comments in ns in format:
        # dec; str, int, float => sum(str, int, float)
        # Prioritize fast dec as most Quantity creation is internal with a dec
        # Also choose to prioritize float over int/str as performance-sensitive things
        # are likely using floats
        # Considered using function but adds around 40 ns overhead
        # One-liner isn't faster than multi-line
        # Timings tested with dec("3.2"); "3.2", 3, 3.2
        # For reference, timings for conversion of each type directly and via str:
        # _number = dec(number)  # 86; 703, 116, 708 => 1527
        # _number = dec.from_float(number)  # N/A; N/A, 116, 719
        # _number = dec(str(number))  # 516; 331, 185, 328 => 844
        # 844 ns is presumably the lower limit for the total of the non-dec timings
        # I have absolutely no idea why the latter should be 2x faster for str and float
        # Naturally "3.2" vs 3 is unfair, but it is more relevant here
        # For reference, timings for dec(number) with 3 in each type:  # 86; 143, 116, 318 => 577
        # And for dec(str(number)):  # 217; 151, 191, 250 => 592

        # Current method: # 18; 405, 244, 420 => 1069
        if isinstance(number, dec):
            self._number = number
        elif isinstance(number, float) and quanfig.CONVERT_FLOAT_AS_STR:
            self._number = dec(str(number))
        else:
            self._number = dec(number)

        # _number = number if isinstance(number, dec) else dec(str(number)) if (isinstance(number, float) and quanfig.CONVERT_FLOAT_AS_STR) else dec(number)  # 18; 430, 203, 422 => 1055
        # _number = number if isinstance(number, dec) else dec(number) if isinstance(number, (int, str)) else dec(str(number)) if quanfig.CONVERT_FLOAT_AS_STR else dec(str(number)) # 18; 518, 184, 498 => 1200
        # _number = number if isinstance(number, dec) else dec(number) if isinstance(number, (int, str)) or not quanfig.CONVERT_FLOAT_AS_STR else dec(str(number))  # 20; 483, 186, 493 => 1162
        # Old method, very slow for decimals:
        # _number = dec(str(number)) if quanfig.CONVERT_FLOAT_AS_STR else dec(number)  # 540; 368, 213, 367 => 948

        if isinstance(unit, str):
            self._unit = units.parse(unit)
        elif unit is None:
            self._unit = units.unitless
        else:
            self._unit = unit

        if not uncertainty:
            self._uncertainty = dec(0)
        elif isinstance(uncertainty, AbstractQuantity):
            if uncertainty._unit != self._unit:
                self._uncertainty = uncertainty.to(self._unit).number
            else:
                self._uncertainty = uncertainty.number
        elif isinstance(uncertainty, dec):
            self._uncertainty = uncertainty
        elif isinstance(uncertainty, float) and quanfig.CONVERT_FLOAT_AS_STR:
            self._uncertainty = dec(str(uncertainty))
        else:
            self._uncertainty = dec(uncertainty)

        # Variable that indicates unit needs cancelling but is initially uncancelled
        self._pending_cancel = _pending_cancel
        # Variable to indicate normalization should be skipped
        self._is_normalized = False


    # These properties should be overridden by subclasses, but unfortunately it is no
    # longer possible to create abstract properties

    @property
    def number(self) -> dec:
        if quanfig.AUTO_NORMALIZE and not self._is_normalized:
            if str(self._number)[-quanfig.AUTO_NORMALIZE:] == "0" * quanfig.AUTO_NORMALIZE:
                # We can just normalize in place in this case as it doesn't change the
                # value or the hash of either the number or the Quantity itself
                self._number = self._number.normalize()
            self._is_normalized = True
        return self._number

    @property
    def unit(self):
        return self._unit

    @property
    def uncertainty(self):
        return self._uncertainty

    @property
    def value(self):
        return self._value

    @abstractmethod
    def resolution(self) -> Self:
        """Return the resolution of the quantity, ignoring the uncertainty.

        The resolution is the size of a unit of the smallest significant figure.

        Termed "resolution" to avoid confusion between this and the computer science
        concept of "precision", which typically refers to the number of digits.
        """
        raise NotImplementedError

    @abstractmethod
    def cancel(self) -> Self:
        raise NotImplementedError

    @abstractmethod
    def fully_cancel(self) -> Self:
        raise NotImplementedError

    @abstractmethod
    def canonical(self) -> Self:
        raise NotImplementedError

    @abstractmethod
    def base(self) -> Self:
        raise NotImplementedError

    @abstractmethod
    def to(self, other) -> Self:
        raise NotImplementedError

    @abstractmethod
    def on_scale(self, other) -> Self:
        raise NotImplementedError

    def __repr__(self) -> str:
        # Don't truncate or group digits here, only apply printing options to printing
        num_string = format_number(
            self.number,
            truncate=0,
            group=0,
        )
        if not self._uncertainty:
            return f"{type(self).__name__}({num_string}, {self.unit})"
        else:
            uncert_string = format_number(
                self._uncertainty,
                truncate=0,
                group=0,
            )
            return f"{type(self).__name__}({num_string}, {self.unit}, uncertainty={uncert_string})"

    def __str__(self) -> str:
        if quanfig.ROUND_BEFORE_PRINT:
            # Note this doesn't actually replace the object, only the value of self in
            # this function's scope
            self = self.round()
        return format_quantity(
            self,
            truncate=quanfig.ELLIPSIS_LONG_DECIMAL,
            group=quanfig.GROUP_DIGITS,
            group_which=quanfig.GROUP_DIGITS_STYLE,
            group_sep=quanfig.GROUP_SEPARATOR,
            uncertainty_style=quanfig.UNCERTAINTY_STYLE,
        )

    def __round__(self, ndigits=None, method=None, pad=None) -> Self:
        """Alias for `Quantity.round()` to allow the use of the built-in `round()`."""
        return self.round(ndigits, method, pad)

    def round(
        self,
        ndigits=None,
        method=None,
        pad=None,
        mode=None,
        method_if_uncertainty=None,
        method_if_exact=None,
    ) -> Self:
        """Return the quantity with the numerical part rounded by the specified method.

        Calls one of `Quantity`'s other rounding methods depending on the value of
        `method`:
        `"PLACES"` ⇒ `Quantity.round_to_places()`
        `"FIGURES"` ⇒ `Quantity.round_to_figures()`
        `"UNCERTAINTY"` ⇒ `Quantity.round_to_uncertainty()`

        By specifying `method_if_uncertainty` and `method_if_exact` separately,
        quantities can be rounded differently depending on whether they have an
        uncertainty or not. Specifying these values overrides the value of `method` for
        those quantities.

        If no rounding methods are provided explicitly, the default rounding methods are
        specified by `quanstants.quanfig.ROUND_TO_IF_UNCERTAINTY` and
        `quanstants.quanfig.ROUND_TO_IF_EXACT`.

        Similarly, if no value is provided for `ndigits`, each rounding method will use
        the default value specified by `quanstants.quanfig.NDIGITS_<rounding method>`.

        The current defaults are such that exact quantities are rounded to 3 s.f., and
        quantities with uncertainties have their uncertainty rounded to 1 s.f. and the
        quantity is then rounded to the same precision.

        Note the distinction in `quanstants` between a rounding "method" (to decimal
        places or significant figures etc.) and rounding "mode" (how to round the final
        digit i.e. up or down).

        The mode used for rounding is set by the `quanstants.quanfig.ROUNDING_MODE`
        variable, which takes any of the `decimal` module's rounding modes. The default
        is `"ROUND_HALF_UP"`, i.e. to nearest with ties going away from zero.

        Like `siunitx`, and like `Decimal`, by default extra zeroes will be added to a
        short number to reach the desired precision. This behaviour can be forced on or
        off by passing `pad=True` or `pad=False`, or set globally by changing
        `quanstants.quanfig.ROUND_PAD`.

        Note that the rounding is done within a `decimal.localcontext()`, which means
        that the mode specified by `quanstants.quanfig.ROUNDING_MODE` does not override
        the current `decimal.Context()` and other `Decimal` instances will continue to
        round based on `decimal.getcontext().rounding`, which by default uses
        `"ROUND_HALF_EVEN"`.
        """
        if self._uncertainty:
            selected_method = (
                method_if_uncertainty if method_if_uncertainty
                else method if method
                else quanfig.ROUND_TO_IF_UNCERTAINTY
            )
        else:
            selected_method = (
                method_if_exact if method_if_exact
                else method if method
                else quanfig.ROUND_TO_IF_EXACT
            )

        if selected_method == "PLACES":
            return self.round_to_places(ndigits, pad, mode)
        elif selected_method == "FIGURES":
            return self.round_to_figures(ndigits, pad, mode)
        elif selected_method == "UNCERTAINTY":
            return self.round_to_uncertainty(ndigits, pad, mode)

    def round_to_places(self, ndigits=None, pad=None, mode=None) -> Self:
        """Return the quantity with the numerical part rounded to the specified number
        of decimal places.

        Defaults to `quanstants.quanfig.NDIGITS_PLACES` if `ndigits` is not provided.
        """
        # Default to choices in config variables if not passed
        ndigits = quanfig.NDIGITS_PLACES if ndigits is None else ndigits
        pad = quanfig.ROUND_PAD if pad is None else pad
        mode = quanfig.ROUNDING_MODE if mode is None else mode
        
        rounded = type(self)(
                number=rounding.to_places(self.number, ndigits, pad, mode),
                unit=self._unit,
                uncertainty=self._uncertainty,
                _pending_cancel=self._pending_cancel,
            )
        return rounded

    def round_to_figures(self, ndigits=None, pad=None, mode=None) -> Self:
        """Return the quantity with the numerical part rounded to the specified number
        of significant figures.

        Defaults to `quanstants.quanfig.NDIGITS_FIGURES` if `ndigits` is not provided.
        """
        # Default to choices in config variables if not passed
        ndigits = quanfig.NDIGITS_FIGURES if ndigits is None else ndigits
        pad = quanfig.ROUND_PAD if pad is None else pad
        mode = quanfig.ROUNDING_MODE if mode is None else mode
        
        return type(self)(
            number=rounding.to_figures(self.number, ndigits, pad, mode),
            unit=self._unit,
            uncertainty=self._uncertainty,
            _pending_cancel=self._pending_cancel,
        )

    def round_to_sigfigs(self, *args, **kwargs) -> Self:
        """Alias for `round_to_figures()`."""
        return self.round_to_figures(*args, **kwargs)

    def round_to_uncertainty(self, ndigits=None, pad=None, mode=None) -> Self:
        """Round the uncertainty to the specified number of significant figures, then
        return the quantity with the numerical part rounded to the same precision.

        Defaults to `quanstants.quanfig.NDIGITS_UNCERTAINTY` if `ndigits` is not
        provided.

        If `pad` is set to `True`, the number of the quantity will be padded if
        necessary to match the precision of the uncertainty. However, no padding is ever
        applied to the uncertainty itself, to ensure that the rounded quantity will
        never have a precision greater than the original uncertainty.
        """
        # Default to choices in config variables if not passed
        ndigits = quanfig.NDIGITS_UNCERTAINTY if ndigits is None else ndigits
        pad = quanfig.ROUND_PAD if pad is None else pad
        mode = quanfig.ROUNDING_MODE if mode is None else mode

        # Check that the quantity even has an uncertainty
        if not self._uncertainty:
            return self
        # Sanity check for requested number of sigfigs
        if ndigits < 1:
            return self
        # First round the uncertainty
        # Get the uncertainty as a Quantity, because then it can round itself
        # Never pad the uncertainty as that would be increasing its precision
        rounded_uncertainty = (
            self.uncertainty.round_to_figures(ndigits, pad=False, mode=mode)
        )
        # Now round the number to the same precision
        return (
            self.round_to_resolution_of(rounded_uncertainty, pad, mode)
            .with_uncertainty(rounded_uncertainty)
        )

    def round_to_resolution_of(self, other, pad=None, mode=None) -> Self:
        if self._unit != other._unit:
            raise MismatchedUnitsError
        places = other.number.as_tuple().exponent * -1
        return self.round_to_places(places, pad, mode)

    def round_uncertainty(self, ndigits=None, method=None, mode=None) -> Self:
        """Round the uncertainty without changing the number.

        Calls `.round()` on the uncertainty with the passed options, so will otherwise
        default to the behaviour of `round()` i.e. the rounding method will be
        `quanstants.quanfig.ROUND_TO_IF_EXACT` and the number of digits will be
        `quanstants.quanfig.NDIGITS_<rounding method>`.

        Unlike when rounding the number of a quantity, `quanstants.quanfig.ROUND_PAD`
        has no effect. The uncertainty is never padded when rounding as that would imply
        an increase in the precision of the uncertainty itself.
        """
        return self.with_uncertainty(
            self.uncertainty.round(ndigits, method, pad=False, mode=mode)
        )

    def with_uncertainty(self, uncertainty) -> Self:
        """Return a new quantity with the provided uncertainty."""
        return type(self)(
            number=self.number,
            unit=self._unit,
            uncertainty=uncertainty,
            _pending_cancel=self._pending_cancel,
        )

    def plus_minus(self, uncertainty) -> Self:
        """Alias for `with_uncertainty()`."""
        return self.with_uncertainty(uncertainty)

    def is_dimensionless(self) -> bool:
        """Check if unit is dimensionless."""
        return self._unit.is_dimensionless()
    
    def normalize(self, threshold: int = 0) -> Self:
        """Call `Decimal.normalize()` on the number and uncertainty.
        
        If a threshold is provided, only numbers with more trailing zeroes than the
        threshold will be normalized.
        """
        normalized = type(self)(
            rounding.normalize(self.number, threshold),
            self._unit,
            rounding.normalize(self._uncertainty, threshold),
            _pending_cancel=self._pending_cancel,
        )
        normalized._is_normalized = True
        return normalized
