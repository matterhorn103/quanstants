from .unit import Unitless, BaseUnit, DerivedUnit
from .prefix import Prefix, PrefixedUnit
from .quantity import Quantity

from .si import *
from .metric import watthour


# fmt: off

# Metric prefixes
quecto = Prefix("q", "quecto", "1E-30")
ronto = Prefix("r", "ronto", "1E-27")
yocto = Prefix("y", "yocto", "1E-24")
zepto = Prefix("z", "zepto", "1E-21")
atto = Prefix("a", "atto", "1E-18")
femto = Prefix("f", "femto", "1E-15")
pico = Prefix("p", "pico", "1E-12")
nano = Prefix("n", "nano", "1E-9")
micro = Prefix("μ", "micro", "1E-6")
milli = Prefix("m", "milli", "1E-3")
centi = Prefix("c", "centi", "1E-2")
deci = Prefix("d", "deci", "1E-1")
deca = Prefix("da", "deca", "1E+1")
hecto = Prefix("h", "hecto", "1E+2")
kilo = Prefix("k", "kilo", "1E+3")
mega = Prefix("M", "mega", "1E+6")
giga = Prefix("G", "giga", "1E+9")
tera = Prefix("T", "tera", "1E+12")
peta = Prefix("P", "peta", "1E+15")
exa = Prefix("E", "exa", "1E+18")
zetta = Prefix("Z", "zetta", "1E+21")
yotta = Prefix("Y", "yotta", "1E+24")
ronna = Prefix("R", "ronna", "1E+27")
quetta = Prefix("Q", "quetta", "1E+30")

# Binary prefixes
kibi = Prefix("Ki", "kibi", 1024**1)
mebi = Prefix("Mi", "mebi", 1024**2)
gibi = Prefix("Gi", "gibi", 1024**3)
tebi = Prefix("Ti", "tebi", 1024**4)
pebi = Prefix("Pi", "pebi", 1024**5)
exbi = Prefix("Ei", "exbi", 1024**6)
zebi = Prefix("Zi", "zebi", 1024**7)
yobi = Prefix("Yi", "yobi", 1024**8)

# Pre-define the most common prefixed units such that they are available in the main namespace
# Base selection on that in siunitx
#                    0     1      2     3     4      5      6     7    8     9
common_prefixes = [atto, femto, pico, nano, micro, milli, kilo, mega, giga, tera]
for p in common_prefixes[1:6]:
    PrefixedUnit(p, gram, add_to_reg=True)

for p in common_prefixes[1:7]:
    if p.name != "micro":
        PrefixedUnit(p, metre, add_to_reg=True)
PrefixedUnit(micro, metre, add_to_reg=True, alt_names=["micron"])
PrefixedUnit(centi, metre, add_to_reg=True)
PrefixedUnit(deci, metre, add_to_reg=True)

for p in common_prefixes[:6]:
    PrefixedUnit(p, second, add_to_reg=True)

for p in common_prefixes[1:7]:
    PrefixedUnit(p, mole, add_to_reg=True)

for p in common_prefixes[2:7]:
    PrefixedUnit(p, ampere, add_to_reg=True)

for p in common_prefixes[4:5]:
    PrefixedUnit(p, litre, add_to_reg=True)
PrefixedUnit(hecto, litre, add_to_reg=True)

for p in common_prefixes[2:6]:
    PrefixedUnit(p, kelvin, add_to_reg=True)

for p in common_prefixes[5:]:
    PrefixedUnit(p, hertz, add_to_reg=True)

for p in common_prefixes[5:8]:
    PrefixedUnit(p, newton, add_to_reg=True)

for p in common_prefixes[6:9]:
    PrefixedUnit(p, pascal, add_to_reg=True)

PrefixedUnit(milli, ohm, add_to_reg=True)
PrefixedUnit(kilo, ohm, add_to_reg=True, alt_names=["kilohm"])
PrefixedUnit(mega, ohm, add_to_reg=True, alt_names=["megohm"])

for p in common_prefixes[2:7]:
    PrefixedUnit(p, volt, add_to_reg=True)

for p in common_prefixes[3:9]:
    PrefixedUnit(p, watt, add_to_reg=True)

PrefixedUnit(kilo, watthour, add_to_reg=True)

for p in common_prefixes[3:8]:
    PrefixedUnit(p, joule, add_to_reg=True)

for p in common_prefixes[5:]:
    PrefixedUnit(p, electronvolt, add_to_reg=True)

for p in common_prefixes[1:6]:
    PrefixedUnit(p, farad, add_to_reg=True)

for p in common_prefixes[1:6]:
    PrefixedUnit(p, henry, add_to_reg=True)

for p in common_prefixes[3:6]:
    PrefixedUnit(p, coulomb, add_to_reg=True)

for p in common_prefixes[4:6]:
    PrefixedUnit(p, tesla, add_to_reg=True)

# fmt: on