from ..prefix import PrefixedUnit
from ..prefixes.metric import *
from .base import *
from .si import *

# fmt: off

# Pre-define the most common prefixed units such that they are available in the main namespace
# Base selection on that in siunitx
#                    0     1      2     3     4      5      6     7    8     9
common_prefixes = [atto, femto, pico, nano, micro, milli, kilo, mega, giga, tera]
for p in common_prefixes[1:6]:
    PrefixedUnit(p, gram, add_to_namespace=True)

for p in common_prefixes[1:7]:
    if p.name != "micro":
        PrefixedUnit(p, metre, add_to_namespace=True)
PrefixedUnit(micro, metre, alt_names=["micron"], add_to_namespace=True)
# Have a variable pointing to the centimetre for easier import by other modules
centimetre = PrefixedUnit(centi, metre, add_to_namespace=True)
PrefixedUnit(deci, metre, add_to_namespace=True)

for p in common_prefixes[:6]:
    PrefixedUnit(p, second, add_to_namespace=True)

for p in common_prefixes[1:7]:
    PrefixedUnit(p, mole, add_to_namespace=True)

for p in common_prefixes[2:7]:
    PrefixedUnit(p, ampere, add_to_namespace=True)

for p in common_prefixes[3:6]:
    PrefixedUnit(p, litre, add_to_namespace=True)
PrefixedUnit(centi, litre, add_to_namespace=True)
PrefixedUnit(hecto, litre, add_to_namespace=True)

for p in common_prefixes[2:6]:
    PrefixedUnit(p, kelvin, add_to_namespace=True)

for p in common_prefixes[5:]:
    PrefixedUnit(p, hertz, add_to_namespace=True)

for p in common_prefixes[5:8]:
    PrefixedUnit(p, newton, add_to_namespace=True)

for p in common_prefixes[6:9]:
    PrefixedUnit(p, pascal, add_to_namespace=True)

PrefixedUnit(milli, ohm, add_to_namespace=True)
PrefixedUnit(kilo, ohm, alt_names=["kilohm"], add_to_namespace=True)
PrefixedUnit(mega, ohm, alt_names=["megohm"], add_to_namespace=True)

for p in common_prefixes[2:7]:
    PrefixedUnit(p, volt, add_to_namespace=True)

for p in common_prefixes[3:9]:
    PrefixedUnit(p, watt, add_to_namespace=True)

for p in common_prefixes[3:8]:
    PrefixedUnit(p, joule, add_to_namespace=True)

for p in common_prefixes[5:]:
    PrefixedUnit(p, electronvolt, add_to_namespace=True)

for p in common_prefixes[1:6]:
    PrefixedUnit(p, farad, add_to_namespace=True)

for p in common_prefixes[1:6]:
    PrefixedUnit(p, henry, add_to_namespace=True)

for p in common_prefixes[3:6]:
    PrefixedUnit(p, coulomb, add_to_namespace=True)

for p in common_prefixes[4:6]:
    PrefixedUnit(p, tesla, add_to_namespace=True)

# fmt: on