from ..unit import DerivedUnit, unitless
from ..prefix import PrefixedUnit
from ..prefixes.default import kilo
from ..quantity import Quantity
from ..si import *

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

# Other
watthour = DerivedUnit("Wh", "watthour", Quantity("3.6E6", joule), canon_symbol=True)
kilowatthour = PrefixedUnit(kilo, watthour, add_to_namespace=True)

# Temperature
degreeFahrenheit = TemperatureUnit("°F", "degreeFahrenheit", "0.5555555555555555555555555555555555555555555555555556", "459.67", add_to_namespace=True, canon_symbol=True, alt_names=["degree_Fahrenheit", "degreeF", "fahrenheit"])

# Standard states
# standard atm
# bar
# standard pressure

carat = DerivedUnit("ct", "carat", Quantity("0.2", gram), canon_symbol=True)
point = DerivedUnit(None, "point", Quantity("0.002", gram), canon_symbol=False)

bit = DerivedUnit("bit", "bit", Quantity(1, unitless), canon_symbol=False)
byte = DerivedUnit("B", "byte", Quantity(8, bit), canon_symbol=True)

# fmt: on
