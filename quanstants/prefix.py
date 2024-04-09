from decimal import Decimal as dec

from .quantity import Quantity
from .unit import BaseUnit, DerivedUnit
from .unit_defs import kilogram


class PrefixAlreadyDefinedError(Exception):
    pass

# Namespace class to contain all the prefixes, making them useable with prefix.n notation
class PrefixReg:
    def add(self, name, prefix):
        if hasattr(self, name):
            raise PrefixAlreadyDefinedError
        setattr(self, name, prefix)

prefix_reg = PrefixReg()

# Simple list of prefix names
prefix_list = []

class AlreadyPrefixedError(Exception):
    pass

class Prefix:
    """An object representing a metric prefix.
    
    Combines with a BaseUnit or DerivedUnit to form a new DerivedUnit.
    """
    def __init__(
        self,
        symbol,
        name,
        multiplier: str | int | float | dec,
    ):
        self.symbol = symbol
        self.name = name
        self.multiplier = dec(str(multiplier))
        prefix_reg.add(self.symbol, self)
        prefix_reg.add(self.name, self)
        prefix_list.append(self.name)
    
    def __mul__(self, other):
        if isinstance(other, (BaseUnit, DerivedUnit)):
            # Special behaviour for kilo + gram
            if (self.name == "kilo") and (other.name == "gram"):
                return kilogram
            # Make sure the user is not trying to add a second prefix to a prefixed unit
            if isinstance(other, DerivedUnit):
                if other.name.startswith(tuple(prefix_list)):
                    raise AlreadyPrefixedError
            concat_symbol = self.symbol + other.symbol
            if (self.name is not None) and (other.name is not None):
                concat_name = self.name + other.name
            if (self.name is not None) and (other.alt_names is not None):
                concat_alt_names = []
                for alt_name in other.alt_names:
                    concat_alt_name = self.name + alt_name
                    concat_alt_names.append(concat_alt_name)
            else:
                concat_alt_names = None
            return DerivedUnit(
                symbol=concat_symbol,
                name=concat_name,
                value=Quantity(self.multiplier, other),
                add_to_reg=False,
                alt_names=concat_alt_names,
            )
        else:
            return NotImplemented


# Metric prefixes
quetta = Prefix("Q", "quetta", "1E+30")
ronna = Prefix("R", "ronna", "1E+27")
yotta = Prefix("Y", "yotta", "1E+24")
zetta = Prefix("Z", "zetta", "1E+21")
exa = Prefix("E", "exa", "1E+18")
peta = Prefix("P", "peta", "1E+15")
tera = Prefix("T", "tera", "1E+12")
giga = Prefix("G", "giga", "1E+9")
mega = Prefix("M", "mega", "1E+6")
kilo = Prefix("k", "kilo", "1E+3")
hecto = Prefix("h", "hecto", "1E+2")
deca = Prefix("da", "deca", "1E+1")
deci = Prefix("d", "deci", "1E-1")
centi = Prefix("c", "centi", "1E-2")
milli = Prefix("m", "milli", "1E-3")
micro = Prefix("μ", "micro", "1E-6")
nano = Prefix("n", "nano", "1E-9")
pico = Prefix("p", "pico", "1E-12")
femto = Prefix("f", "femto", "1E-15")
atto = Prefix("a", "atto", "1E-18")
zepto = Prefix("z", "zepto", "1E-21")
yocto = Prefix("y", "yocto", "1E-24")
ronto = Prefix("r", "ronto", "1E-27")
quecto = Prefix("q", "quecto", "1E-30")

# Binary prefixes
yobi = Prefix("Yi", "yobi", 1024**8)
zebi = Prefix("Zi", "zebi", 1024**7)
exbi = Prefix("Ei", "exbi", 1024**6)
pebi = Prefix("Pi", "pebi", 1024**5)
tebi = Prefix("Ti", "tebi", 1024**4)
gibi = Prefix("Gi", "gibi", 1024**3)
mebi = Prefix("Mi", "mebi", 1024**2)
kibi = Prefix("Ki", "kibi", 1024**1)