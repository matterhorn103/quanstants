from ..unit import DerivedUnit
from ..quantity import Quantity
from .base import *


# fmt: off

fortnight = DerivedUnit(None, "fortnight", Quantity("1209600", s), canon_symbol=False)
sidereal_day = DerivedUnit("sday", "sidereal_day", Quantity("86164.091", s), canon_symbol=False)
week = DerivedUnit("wk", "week", Quantity("1209600", s), canon_symbol=True)

# fmt: on