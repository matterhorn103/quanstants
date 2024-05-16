from ..prefix import Prefix, PrefixedUnit

from ..si import *


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

# fmt: on
