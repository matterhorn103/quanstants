from ..unit import DerivedUnit
from ..quantity import Quantity
from .base import *

# fmt: off

from .common import bit
from .common import byte
nibble = DerivedUnit(None, "nibble", Quantity(4, bit), canon_symbol=False, alt_names=["nybble", "nyble", "nybl", "half_byte", "tetrade", "semioctet", "quadbit", "quartet"])
octet = DerivedUnit("o", "octet", Quantity(8, bit), canon_symbol=False)

# fmt: on