"""
Users are expected to import with `from quanstants import units, constants`
and optionally `from quanstants import Quantity`.
"""
from decimal import Decimal as dec

# Definition modules are unused but need to be imported so that dictionaries get populated
# Import unit first because the other classes rely on it
from .config import *
from . import unit, unit_defs
from . import prefix
from . import constant, constant_defs
from . import quantity

# Make unit, prefix, and constant namespaces available directly in this namespace
units = unit.unit_reg
prefixes = prefix.prefix_reg
constants = constant.constant_reg

# Make Quantity class available directly in this namespace
Quantity = quantity.Quantity
