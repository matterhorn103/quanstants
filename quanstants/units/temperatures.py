from ..temperature import TemperatureUnit

# fmt: off

# TODO
from ..si import kelvin
from ..si import degreeCelsius
from .default import degreeFahrenheit

degreeReaumur = TemperatureUnit(
    "°Re",
    "degreeReaumur",
    "1.25",
    "273.15",
    add_to_reg=True,
    canon_symbol=True,
    alt_names=["degree_Reaumur", "degreeRe", "reaumur"],
)

#rankine
#delisle
#newton
#romer

# fmt: on
