from decimal import Decimal as dec

from .unit import BaseUnit


second = BaseUnit("s", "second", "T")
metre = BaseUnit("m", "metre", "L", ["meter"])
kilogram = BaseUnit("kg", "kilogram", "M", ["kilo"])
ampere = BaseUnit("A", "ampere", "I", ["amp"])
kelvin = BaseUnit("K", "kelvin", "Θ")
mole = BaseUnit("mol", "mole", "N")
candela = BaseUnit("cd", "candela", "J")