from .unit import UnitlessUnit, BaseUnit, DerivedUnit
from .quantity import Quantity


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

# Special unitless dimensionless unit
unitless = UnitlessUnit()

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

# Named SI derived units, all are canon symbols
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
# degreeC Don't define for now, need to work out how best to handle temp
lumen = DerivedUnit("lm", "lumen", Quantity(1, cd * sr), canon_symbol=True)
lux = DerivedUnit("lx", "lux", Quantity(1, cd * sr * m**-2), canon_symbol=True)
becquerel = DerivedUnit("Bq", "becquerel", Quantity(1, s**-1), canon_symbol=True)
gray = DerivedUnit("Gy", "gray", Quantity(1, m**2 * s**-2), canon_symbol=True)
sievert = DerivedUnit("Sv", "sievert", Quantity(1, m**2 * s**-2), canon_symbol=True)
katal = DerivedUnit("kat", "katal", Quantity(1, s**-1 * mol), canon_symbol=True)
