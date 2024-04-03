"""
Users are expected to import with `from quanstants import units, constants`
and optionally `from quanstants import Quantity`.
"""
# Expose Quantity class directly but not Constant or Unit
# Definition modules are unused but need to be imported so that dictionaries get populated
from . import constant, constant_defs, unit, unit_defs
from .quantity import Quantity

# Make unit and constant dictionaries available directly in this namespace
units = unit.unit_reg
constants = constant.constant_reg
