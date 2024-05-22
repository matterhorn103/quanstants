from ..unit import DerivedUnit
from ..quantity import Quantity
from .base import *

angstrom_star = DerivedUnit("Å*", "angstrom_star", Quantity("1.00001495e-10", m, "0.00000090e-10"), canon_symbol=False, alt_names=["ångström_star"])
from .common import julian_year
light_year = DerivedUnit("ly", "light_year", Quantity("9460730472580800", m), canon_symbol=True)