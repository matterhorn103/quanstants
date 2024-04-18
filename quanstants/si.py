import math
from decimal import Decimal as dec

from .unit import Unitless, BaseUnit, DerivedUnit
from .quantity import Quantity


# fmt: off

# SI base units
second = BaseUnit("s", "second", dimension="T")
metre = BaseUnit("m", "metre", dimension="L", alt_names=["meter"])
kilogram = BaseUnit("kg", "kilogram", dimension="M", alt_names=["kilo"])
ampere = BaseUnit("A", "ampere", dimension="I", alt_names=["amp"])
kelvin = BaseUnit("K", "kelvin", dimension="Θ")
mole = BaseUnit("mol", "mole", dimension="N")
candela = BaseUnit("cd", "candela", dimension="J")

# Also define radians and steradians as dimensionless base units
radian = BaseUnit("rad", "radian", dimension="X")
steradian = BaseUnit("sr", "steradian", dimension="X")

# Create aliases for all of the above to make defining new units easier
s = second
m = metre
kg = kilogram
A = ampere
K = kelvin
mol = mole
cd = candela
rad = radian
sr = steradian

# Special unitless dimensionless unit
unitless = Unitless(add_to_reg=True)


# Derived units
# Most importantly, define the gram, but simply as a DerivedUnit
gram = DerivedUnit("g", "gram", Quantity("0.001", kg), canon_symbol=True)

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
#degreeC TODO Don't define for now, need to work out how best to handle temp
lumen = DerivedUnit("lm", "lumen", Quantity(1, cd * sr), canon_symbol=True)
lux = DerivedUnit("lx", "lux", Quantity(1, cd * sr * m**-2), canon_symbol=True)
becquerel = DerivedUnit("Bq", "becquerel", Quantity(1, s**-1), canon_symbol=True)
gray = DerivedUnit("Gy", "gray", Quantity(1, m**2 * s**-2), canon_symbol=True)
sievert = DerivedUnit("Sv", "sievert", Quantity(1, m**2 * s**-2), canon_symbol=True)
katal = DerivedUnit("kat", "katal", Quantity(1, s**-1 * mol), canon_symbol=True)

# Non-SI units officially accepted for use with the SI
arcminute = DerivedUnit("′", "arcminute", Quantity(dec(math.pi)/10800, rad), canon_symbol=True)
arcsecond = DerivedUnit("″", "arcsecond", Quantity(dec(math.pi)/648000, rad), canon_symbol=True)
astronomical_unit = DerivedUnit("au", "astronomical_unit", Quantity(149597870700, m), canon_symbol=True)
#bel TODO
dalton = DerivedUnit("Da", "dalton", Quantity("1.66053906660e-27", kg, "0.00000000050e-27"), canon_symbol=True, alt_names=["atomic_mass_unit", "unified_atomic_mass_unit"])
day = DerivedUnit("d", "day", Quantity(86400, s), canon_symbol=True)
#decibel TODO
degree = DerivedUnit("°", "degree", Quantity(dec(math.pi)/180, rad), canon_symbol=True)
electronvolt = DerivedUnit("eV", "electronvolt", Quantity("1.602176634e-19", joule), canon_symbol=True)
hectare = DerivedUnit("ha", "hectare", Quantity("1E4", m**2), canon_symbol=True)
hour = DerivedUnit("h", "hour", Quantity(3600, s), canon_symbol=True)
litre = DerivedUnit("L", "litre", Quantity("1E-3", m**3), canon_symbol=True, alt_names=["liter"])
minute = DerivedUnit("min", "minute", Quantity(60, s), canon_symbol=True)
#neper TODO
tonne = DerivedUnit("t", "tonne", Quantity(1000, kg), canon_symbol=True)

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

# fmt: on