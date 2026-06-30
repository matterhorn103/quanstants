from ..unit import DerivedUnit, unitless
from ..quantity import Quantity

# fmt: off

# Percent and other ratios
# Unfortunately will have preceding space, matching SI style but contrary to popular use
from .common import percent
permille = DerivedUnit("‰", "permille", Quantity("0.001", unitless), alt_names=["per_mille"], canon_symbol=True, _space_rule="PERCENTAGE_SPACE")
permyriad = DerivedUnit("‱", "permyriad", Quantity("0.0001", unitless), alt_names=["per_myriad"], canon_symbol=True, _space_rule="PERCENTAGE_SPACE")
percentmille = DerivedUnit(None, "percentmille", Quantity("0.0001", unitless), alt_names=["per_cent_mille"], canon_symbol=False, _space_rule="PERCENTAGE_SPACE")
parts_per_million = DerivedUnit("ppm", "parts_per_million", Quantity("1e-6", unitless), alt_names=["partspermillion"], canon_symbol=True)
# Define billion, trillion, quadrillion using the short scale
parts_per_billion = DerivedUnit("ppb", "parts_per_billion", Quantity("1e-9", unitless), alt_names=["partsperbillion"], canon_symbol=True)
parts_per_trillion = DerivedUnit("ppt", "parts_per_trillion", Quantity("1e-12", unitless), alt_names=["partspertrillion"], canon_symbol=True)
parts_per_quadrillion = DerivedUnit("ppq", "parts_per_quadrillion", Quantity("1e-15", unitless), alt_names=["partsperquadrillion"], canon_symbol=False)

# fmt: on