from ..unit import DerivedUnit
from ..quantity import Quantity
from .base import *
from .si import litre, hour

# fmt: off

# Imperial units get the right to use the simple name, whereas US customary units are prefixed
# with "us" e.g. "us_foot"
# However, since US customary units are probably more likely to be used these days, US customary
# units are given canon symbols and imperial units aren't (unless there is no US cus. equivalent)
# Units identical in both systems are defined here and imported by `us.py`.

# Imperial (from the international yard and pound agreement and the Weights and Measures Act 1985)
# Length
twip = DerivedUnit(None, "twip", Quantity("0.0000176389", m))
thou = DerivedUnit("th", "thou", Quantity("0.0000254", m))
barleycorn = DerivedUnit(None, "barleycorn", Quantity("0.0084667", m))
inch = DerivedUnit("in", "inch", Quantity("0.0254", m)) # Can't add `in` to namespace as it is Python keyword
hand = DerivedUnit("hh", "hand", Quantity("0.1016", m))
foot = DerivedUnit("ft", "foot", Quantity("0.3048", m))
yard = DerivedUnit("yd", "yard", Quantity("0.9144", m))
chain = DerivedUnit("ch", "chain", Quantity("20.1168", m))
furlong = DerivedUnit("fur", "furlong", Quantity("201.168", m))
mile = DerivedUnit("mi", "mile", Quantity("1609.344", m))
league = DerivedUnit("lea", "league", Quantity("4828.032", m))
#link?
#rod?

# Area
#perch?
rood = DerivedUnit(None, "rood", Quantity("1011.714106", m**2))
acre = DerivedUnit("ac", "acre", Quantity("4046.8564224", m**2))

# Capacity
minim = DerivedUnit("min", "minim", Quantity("0.000059193880208333333333", litre)) # to 20 sf
fluid_scruple = DerivedUnit("fl s", "fluid_scruple", Quantity("0.0011838776041666666667", litre)) # to 20 sf
fluid_drachm = DerivedUnit("fl dr", "fluid_drachm", Quantity("0.0035516328125", litre), alt_names=["fluid_dram"])
fluid_ounce = DerivedUnit("fl oz", "fluid_ounce", Quantity("0.0284130625", litre))
gill = DerivedUnit("gi", "gill", Quantity("0.1420653125", litre))
pint = DerivedUnit("pt", "pint", Quantity("0.56826125", litre))
quart = DerivedUnit("qt", "quart", Quantity("1.1365225", litre))
gallon = DerivedUnit("gal", "gallon", Quantity("4.54609", litre))
peck = DerivedUnit(None, "peck", Quantity("9.09218", litre))
bushel = DerivedUnit(None, "bushel", Quantity("36.36872", litre))

# Mass (all three systems, only grain is common to all)
grain = DerivedUnit("gr", "grain", Quantity("64.79891E-6", kg))

## Avoirdupois system
dram = DerivedUnit("dr", "dram", Quantity("0.0017718451953125", kg))
ounce = DerivedUnit("oz", "ounce", Quantity("0.028349523125", kg))
pound = DerivedUnit("lb", "pound", Quantity("0.45359237", kg))
stone = DerivedUnit("st", "stone", Quantity("6.35029318", kg))
quarter = DerivedUnit("qr", "quarter", Quantity("12.70058636", kg))
cental = DerivedUnit(None, "cental", Quantity("45.359237", kg))
hundredweight = DerivedUnit("cwt", "hundredweight", Quantity("50.80234544", kg))
ton = DerivedUnit(None, "ton", Quantity("1016.0469088", kg), alt_names=["tun"])

## Troy system
pennyweight = DerivedUnit("dwt", "pennyweight", Quantity("0.00155517384", kg))
troy_ounce = DerivedUnit("oz t", "troy_ounce", Quantity("0.03110347680", kg))
troy_pound = DerivedUnit("lb t", "troy_pound", Quantity("0.37324172", kg))

## Apothecaries system
scruple = DerivedUnit("℈", "scruple", Quantity("0.001295962", kg))
drachm = DerivedUnit("ʒ", "drachm", Quantity("0.003887886", kg))
ounce_apothecaries = DerivedUnit("℥", "ounce_apothecaries", Quantity("0.031103088", kg))

# Nautical units
fathom_practical = DerivedUnit(None, "fathom_practical", Quantity("1.8288", m)) # According to Wiki, the Admiralty used 1 ftm = 6 ft in practice
british_fathom = DerivedUnit("br ftm", "british_fathom", Quantity("1.853184", m)) # 1/1000 nmi
fathom = DerivedUnit("ftm", "fathom", Quantity("1.852", m))
british_cable = DerivedUnit(None, "british_cable", Quantity("185.3184", m))
cable = DerivedUnit(None, "cable", Quantity("185.2", m))
british_nautical_mile = DerivedUnit("br nmi", "british_nautical_mile", Quantity("1853.184", m)) # Tradtional value of 6080 ft
nautical_mile = DerivedUnit("nmi", "nautical_mile", Quantity("1852", m)) # International definition
british_knot = DerivedUnit("br kn", "british_knot", Quantity("1853.184", m * hour**-1))
knot = DerivedUnit("kn", "knot", Quantity("1852", m * hour**-1))

# Temperature
from .temperatures import degreeFahrenheit

# fmt: on
