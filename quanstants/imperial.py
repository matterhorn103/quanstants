import math
from decimal import Decimal as dec

from .unit import Unitless, BaseUnit, DerivedUnit
from .quantity import Quantity
from .si import *

# Imperial units get the right to use the simple name, whereas US customary units are prefixed
# with "us" e.g. "us_foot"
# However, since US customary units are probably more likely to be used these days, US customary
# units are given canon symbols and imperial units aren't (unless there is no US cus. equivalent)

# Imperial (from the international yard and pound agreement and the Weights and Measures Act 1985)
# Length
twip = DerivedUnit(None, "twip", Quantity("0.0000176389", m))
thou = DerivedUnit("th", "thou", Quantity("0.0000254", m))
barleycorn = DerivedUnit(None, "barleycorn", Quantity("0.0084667", m))
inch = DerivedUnit("in", "inch", Quantity("0.0254", m)) # Can't add in as it is Python keyword
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
#rood?
acre = DerivedUnit(None, "acre", Quantity("4046.8564224", m**2))

# Capacity
minim = DerivedUnit("min", "minim", Quantity("0.000059193880208333333333", litre)) # to 20 sf
fluid_scruple = DerivedUnit("fl s", "fluid_scruple", Quantity("0.0011838776041666666667", litre)) # to 20 sf
fluid_drachm = DerivedUnit("fl dr", "fluid_drachm", Quantity("0.0035516328125", litre), alt_names=["fluid_dram"])
fluid_ounce = DerivedUnit("fl oz", "fluid_ounce", Quantity("0.0284130625", litre))
gill = DerivedUnit("gi", "gill", Quantity("0.1420653125", litre))
pint = DerivedUnit("pt", "pint", Quantity("0.56826125", litre))
quart = DerivedUnit("qt", "quart", Quantity("1.1365225", litre))
gallon = DerivedUnit("gal", "gallon", Quantity("4.54609", litre))
#Bushel	=	8 gallons.
#Peck	=	2 gallons

# Mass
#Ton	=	2240 pounds.
#Hundredweight	=	112 pounds.
#Cental	=	100 pounds.
#Quarter	=	28 pounds.
#Stone	=	14 pounds.
pound = DerivedUnit("lb", "pound", Quantity("0.45359237", kg))
#Ounce] 1/16 pound
#Dram	=	1/16 ounce
#Grain	=	1/7000 pound
#Pennyweight	=	24 grains
#Ounce apothecaries	=	480 grains
#Drachm	=	1/8 ounce apothecaries
#Scruple	=	1/3 drachm

# Nautical
#fathom
#cable
#nautical mile
#knot

#Troy weight
#avoirdupois weight
#apothecaries' weight
#carat and point

