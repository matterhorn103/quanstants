from ..temperature import TemperatureUnit

# fmt: off

# TODO
from .base import kelvin
from .si import degreeCelsius
from .common import degreeFahrenheit

degreeReaumur = TemperatureUnit(
    "°Re",
    "degreeReaumur",
    "1.25",
    "273.15",
    canon_symbol=True,
    alt_names=["degree_Reaumur", "degreeRe", "reaumur"],
)

#rankine
#delisle
#newton
#romer

# fmt: on
