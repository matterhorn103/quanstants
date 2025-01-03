import math
from decimal import Decimal as dec

from ..config import quanfig
from ..unit import DerivedUnit, UnitlessUnit, unitless
from ..prefixes.metric import deci
from ..quantity import Quantity
from .. import temperature
from ..temperature import TemperatureUnit
from ..log import LogarithmicUnit, PrefixedLogarithmicUnit
from .base import *

# fmt: off

# SI derived units
# Most importantly, define the gram, but simply as a DerivedUnit
gram = DerivedUnit("g", "gram", Quantity("0.001", kg), canon_symbol=True)

# Define radians and steradians as instances of UnitlessUnit
# Previously defined them as BaseUnits, but making them UnitlessUnits means they are
# equal to 1
# Could define them as DerivedUnits (like percent), but then they would get cancelled
radian = UnitlessUnit("rad", "radian", drop=False)
steradian = UnitlessUnit("sr", "steradian", drop=False)

# And giev them aliases
rad = radian
sr = steradian

# Named SI coherent derived units, all are canon symbols
hertz = DerivedUnit("Hz", "hertz", Quantity(1, s**-1), canon_symbol=True)
newton = DerivedUnit("N", "newton", Quantity(1, kg * m * s**-2), canon_symbol=True)
pascal = DerivedUnit("Pa", "pascal", Quantity(1, kg * m**-1 * s**-2), canon_symbol=True)
joule = DerivedUnit("J", "joule", Quantity(1, kg * m**2 * s**-2), canon_symbol=True)
watt = DerivedUnit("W", "watt", Quantity(1, kg * m**2 * s**-3), canon_symbol=True)
coulomb = DerivedUnit("C", "coulomb", Quantity(1, s * A), canon_symbol=True)
volt = DerivedUnit("V", "volt", Quantity(1, kg * m**2 * s**-3 * A**-1), canon_symbol=True)
farad = DerivedUnit("F", "farad", Quantity(1, kg**-1 * m**-2 * s**4 * A**2), canon_symbol=True)
ohm = DerivedUnit("Ω", "ohm", Quantity(1, kg * m**2 * s**-3 * A**-2), canon_symbol=True)
siemens = DerivedUnit("S", "siemens", Quantity(1, kg**-1 * m**-2 * s**3 * A**2), canon_symbol=True)
weber = DerivedUnit("Wb", "weber", Quantity(1, kg * m**2 * s**-2 * A**-1), canon_symbol=True)
tesla = DerivedUnit("T", "tesla", Quantity(1, kg * s**-2 * A**-1), canon_symbol=True)
henry = DerivedUnit("H", "henry", Quantity(1, kg * m**2 * s**-2 * A**-2), canon_symbol=True)
degreeCelsius = TemperatureUnit("°C", "degreeCelsius", "1", "273.15", alt_names=["degree_Celsius", "degreeC", "celsius", "degreeCentigrade", "degree_Centigrade", "centigrade"], add_to_namespace=True, canon_symbol=True)
lumen = DerivedUnit("lm", "lumen", Quantity(1, cd * sr), canon_symbol=True)
lux = DerivedUnit("lx", "lux", Quantity(1, cd * sr * m**-2), canon_symbol=True)
becquerel = DerivedUnit("Bq", "becquerel", Quantity(1, s**-1), canon_symbol=True)
gray = DerivedUnit("Gy", "gray", Quantity(1, m**2 * s**-2), canon_symbol=True)
sievert = DerivedUnit("Sv", "sievert", Quantity(1, m**2 * s**-2), canon_symbol=True)
katal = DerivedUnit("kat", "katal", Quantity(1, s**-1 * mol), canon_symbol=True)

# Non-SI units officially accepted for use with the SI
arcminute = DerivedUnit("′", "arcminute", Quantity(dec(math.pi)/10800, rad), canon_symbol=True, preceding_space=False)
arcsecond = DerivedUnit("″", "arcsecond", Quantity(dec(math.pi)/648000, rad), canon_symbol=True, preceding_space=False)
astronomical_unit = DerivedUnit("au", "astronomical_unit", Quantity(149597870700, m), canon_symbol=True)
bel = LogarithmicUnit("B", None, "bel", 10, canon_symbol=False) # let B be used by byte
dalton = DerivedUnit("Da", "dalton", Quantity("1.66053906660e-27", kg, "0.00000000050e-27"), alt_names=["atomic_mass_unit", "unified_atomic_mass_unit"], canon_symbol=True)
day = DerivedUnit("d", "day", Quantity(86400, s), canon_symbol=True)
decibel = PrefixedLogarithmicUnit(deci, bel, add_to_namespace=True, canon_symbol=True)
degree = DerivedUnit("°", "degree", Quantity(dec(math.pi)/180, rad), canon_symbol=True, preceding_space=False)
electronvolt = DerivedUnit("eV", "electronvolt", Quantity("1.602176634e-19", joule), canon_symbol=True)
hectare = DerivedUnit("ha", "hectare", Quantity("1e4", m**2), canon_symbol=True)
hour = DerivedUnit("h", "hour", Quantity(3600, s), canon_symbol=True)
litre = DerivedUnit(quanfig.LITRE_SYMBOL, "litre", Quantity("1e-3", m**3), alt_names=["liter"], canon_symbol=True)
minute = DerivedUnit("min", "minute", Quantity(60, s), canon_symbol=True)
neper = LogarithmicUnit("Np", None, "neper", "e", canon_symbol=True)
tonne = DerivedUnit("t", "tonne", Quantity(1000, kg), canon_symbol=True)

# TODO
# bel, decibel, neper for power vs root-power quantities
# (Though consider not including defining them here)

# fmt: on

