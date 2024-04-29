"""
Users are expected to import with `from quanstants import units, constants`
and optionally `from quanstants import Quantity`.
"""
from decimal import Decimal as dec

# Setup configuration first in case the user's preferences affect initial setup
from .config import quanfig

# Import unit first because the other classes rely on it
from . import unit
from . import prefix
from . import quantity
from . import constant

# Definition modules are "unused" imports but do need to be imported so that
# they are initiated and the unit/prefix/constant dictionaries get populated
# These are the basic minimum modules -- they are actually loaded already due to being
# used in other modules, but import here for clarity
from . import si, prefix_defs
# Then import whichever optional unit and constant modules should be provided by default
from .units import metric
from .constants import fundamental

# Make the unit, prefix, and constant namespaces available directly in this namespace
units = unit.unit_reg
prefixes = prefix.prefix_reg
constants = constant.constant_reg

# Make Quantity class available directly in this namespace
Quantity = quantity.Quantity
