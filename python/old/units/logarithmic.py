from ..quantity import Quantity
from ..log import LogarithmicUnit, PrefixedLogarithmicUnit

from .si import watt
from ..prefixes.metric import milli, deci

# fmt: off

from .si import bel
from .si import decibel
from .si import neper

# Common suffixed versions of the decibel
dBW = decibel.with_reference(Quantity(1, watt), suffix="W", name="dBW", add_to_namespace=True)
dBm = decibel.with_reference(Quantity(1, milli*watt), suffix="m", name="dBm", add_to_namespace=True)
# TODO Add more

# fmt: on