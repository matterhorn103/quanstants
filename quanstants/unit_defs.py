from .unit import BaseUnit, DerivedUnit
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
pascal
joule
watt
coulomb
volt
farad
ohm
siemens
weber 
tesla
henry
degreeC
lumen
lux
becquerel
gray
sievert
katal
