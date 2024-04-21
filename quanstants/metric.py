from .unit import DerivedUnit
from .quantity import Quantity
from .si import *

# fmt: off

# Percent and other ratios - unfortunately will have preceding space, matching SI style but contrary to popular use
percent = DerivedUnit("%", "percent", Quantity("0.01", unitless), canon_symbol=True, alt_names=["per_cent"])
permille = DerivedUnit("‰", "permille", Quantity("0.001", unitless), canon_symbol=True, alt_names=["per_mille"])
permyriad = DerivedUnit("‱", "permyriad", Quantity("0.0001", unitless), canon_symbol=True, alt_names=["per_myriad"])
percentmille = DerivedUnit(None, "percentmille", Quantity("0.0001", unitless), canon_symbol=False, alt_names=["per_cent_mille"])
parts_per_million = DerivedUnit("ppm", "parts_per_million", Quantity("1e-6", unitless), canon_symbol=True, alt_names=["partspermillion"])
# Define billion, trillion, quadrillion using the short scale
parts_per_billion = DerivedUnit("ppb", "parts_per_billion", Quantity("1e-9", unitless), canon_symbol=True, alt_names=["partsperbillion"])
parts_per_trillion = DerivedUnit("ppt", "parts_per_trillion", Quantity("1e-12", unitless), canon_symbol=True, alt_names=["partspertrillion"])
parts_per_quadrillion = DerivedUnit("ppq", "parts_per_quadrillion", Quantity("1e-15", unitless), canon_symbol=False, alt_names=["partsperquadrillion"])

# CODATA 2018
#Angstrom_star = Constant(None, "Angstrom_star", Quantity("1.00001495e-10", m, "0.00000090e-10"), canon_symbol=False)

# Other
watthour = DerivedUnit("Wh", "watthour", Quantity("3.6E6", joule), canon_symbol=True)

# Temperature
#celsius
#fahrenheit
#rankine
#delisle
#newton
#reaumur
#romer

# Standard states
# standard atm
# bar
# standard pressure

# carat
# point

# fmt: on