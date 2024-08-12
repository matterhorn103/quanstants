"""Units related to thermodynamics, particularly units of energy and power."""

from ..unit import DerivedUnit
from ..prefix import PrefixedUnit
from ..prefixes.metric import kilo
from ..quantity import Quantity
from .base import *

# fmt: off

# Units of energy
#british_thermal_unit
#calorie
#kilocalorie


# Units of power
# Varieties of horsepower
imperial_horsepower = DerivedUnit("hp", "imperial_horsepower", Quantity())



# fmt: on