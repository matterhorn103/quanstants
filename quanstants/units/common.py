"""
Common non-SI units.

Typically these belong more properly to other categories of unit, but are defined here
to reduce the overall import time of `quanstants`.
This way, a commonly used unit can be loaded individually, without loading the full
submodule to which it would more rightfully belong.
Generally the units below are also imported by the appropriate submodule, so that the
user is never told e.g. that `degreeFahrenheit` is not in `units.temperatures`.
"""

from decimal import Decimal as dec
import math

from ..unit import DerivedUnit, unitless
from ..temperature import TemperatureUnit
from ..prefix import PrefixedUnit
from ..prefixes.metric import kilo
from ..quantity import Quantity
from .base import *
from .si import joule, gram, day, rad, minute

# fmt: off

# Percent and other ratios
# Unfortunately will have preceding space, matching SI style but contrary to popular use
percent = DerivedUnit("%", "percent", Quantity("0.01", unitless), alt_names=["per_cent"], canon_symbol=True, _space_rule="PERCENTAGE_SPACE")

# Energy
watthour = DerivedUnit("Wh", "watthour", Quantity("3.6E6", joule), canon_symbol=True)
kilowatthour = PrefixedUnit(kilo, watthour, add_to_namespace=True)

# Time
# Year without qualifier means 365 days, intended for simple/quick/lay use
# Any scientific discipline will know exactly which definition they want to use
year = DerivedUnit("yr", "year", Quantity(365, day), canon_symbol=True)
julian_year = DerivedUnit("a", "julian_year", Quantity("365.25", day), canon_symbol=False)

# Temperature
degreeFahrenheit = TemperatureUnit("°F", "degreeFahrenheit", "0.5555555555555555555555555555555555555555555555555556", "459.67", alt_names=["fahrenheit", "degree_Fahrenheit", "degreeF"], add_to_namespace=True, canon_symbol=True)

# TODO
# Standard states
# standard atm
# bar
# standard pressure

carat = DerivedUnit("ct", "carat", Quantity("0.2", gram), canon_symbol=True)
point = DerivedUnit(None, "point", Quantity("0.002", gram), canon_symbol=False)

# Computing
bit = DerivedUnit("bit", "bit", Quantity(1, unitless), canon_symbol=False)
byte = DerivedUnit("B", "byte", Quantity(8, bit), canon_symbol=True)

# Miscellaneous
revolutions_per_minute = DerivedUnit("rpm", "revolutions_per_minute", Quantity(2 * dec(math.pi), rad * minute**-1), alt_names=["rev_per_min"], canon_symbol=True)

# fmt: on
