from ..unit import DerivedUnit
from ..quantity import Quantity
from ..temperature import TemperatureUnit

# fmt: off

from .base import kelvin
degreeRankine = DerivedUnit(
    "°R",
    "degreeRankine",
    Quantity("0.5555555555555555555555555555555555555555555555555556", kelvin),
    alt_names=["rankine", "degree_Rankine", "degreeR"],
    add_to_namespace=True,
    canon_symbol=True
)
from .si import degreeCelsius
from .common import degreeFahrenheit
degreeReaumur = TemperatureUnit(
    "°Re",
    "degreeReaumur",
    "1.25",
    "273.15",
    alt_names=["reaumur", "degree_Reaumur", "degreeRe"],
    add_to_namespace=True,
    canon_symbol=True,
)

# TODO
#delisle
#newton
#romer

# fmt: on
