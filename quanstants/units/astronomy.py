from ..unit import DerivedUnit
from ..quantity import Quantity
from ..log import LogarithmicUnit
from .base import *
from .si import watt, hertz


# fmt: off

angstrom_star = DerivedUnit("Å*", "angstrom_star", Quantity("1.00001495e-10", m, "0.00000090e-10"), alt_names=["ångström_star"], canon_symbol=False)
from .common import julian_year
light_year = DerivedUnit("ly", "light_year", Quantity("9460730472580800", m), canon_symbol=True)
jansky = DerivedUnit("Jy", "jansky", Quantity("1e-26", watt * metre**-2 * hertz*-1), canon_symbol=False)

# Apparent magnitudes based on https://en.wikipedia.org/wiki/Apparent_magnitude for now
# Article cites Britannica as saying that apparent magnitude without qualification
# indicates V band, so give that "apparent_magnitude" as alias
apparent_magnitude_U = LogarithmicUnit("m", None, "apparent_magnitude_U", 10, "-2.5", Quantity(1810, jansky), canon_symbol=False)
apparent_magnitude_B = LogarithmicUnit("m", None, "apparent_magnitude_B", 10, "-2.5", Quantity(4260, jansky), canon_symbol=False)
apparent_magnitude_V = LogarithmicUnit("m", None, "apparent_magnitude_V", 10, "-2.5", Quantity(3640, jansky), alt_names=["apparent_magnitude"], canon_symbol=False)
apparent_magnitude_R = LogarithmicUnit("m", None, "apparent_magnitude_R", 10, "-2.5", Quantity(3080, jansky), canon_symbol=False)
apparent_magnitude_I = LogarithmicUnit("m", None, "apparent_magnitude_I", 10, "-2.5", Quantity(2550, jansky), canon_symbol=False)
apparent_magnitude_J = LogarithmicUnit("m", None, "apparent_magnitude_J", 10, "-2.5", Quantity(1600, jansky), canon_symbol=False)
apparent_magnitude_H = LogarithmicUnit("m", None, "apparent_magnitude_H", 10, "-2.5", Quantity(1080, jansky), canon_symbol=False)
apparent_magnitude_K = LogarithmicUnit("m", None, "apparent_magnitude_K", 10, "-2.5", Quantity(670, jansky), canon_symbol=False)
apparent_magnitude_g = LogarithmicUnit("m", None, "apparent_magnitude_g", 10, "-2.5", Quantity(3730, jansky), canon_symbol=False)
apparent_magnitude_r = LogarithmicUnit("m", None, "apparent_magnitude_r", 10, "-2.5", Quantity(4490, jansky), canon_symbol=False)
apparent_magnitude_i = LogarithmicUnit("m", None, "apparent_magnitude_i", 10, "-2.5", Quantity(4760, jansky), canon_symbol=False)
apparent_magnitude_z = LogarithmicUnit("m", None, "apparent_magnitude_z", 10, "-2.5", Quantity(4810, jansky), canon_symbol=False)

# fmt: on