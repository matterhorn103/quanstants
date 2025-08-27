from ..unit import BaseUnit

# fmt: off

# SI base units
second = BaseUnit("s", "second", dimensions="T")
metre = BaseUnit("m", "metre", dimensions="L", alt_names=["meter"])
kilogram = BaseUnit("kg", "kilogram", dimensions="M", alt_names=["kilo"])
ampere = BaseUnit("A", "ampere", dimensions="I", alt_names=["amp"])
from ..temperature import kelvin
mole = BaseUnit("mol", "mole", dimensions="N")
candela = BaseUnit("cd", "candela", dimensions="J")

# Create aliases for all of the above to make defining new units easier
s = second
m = metre
kg = kilogram
A = ampere
K = kelvin
mol = mole
cd = candela

# fmt: on
