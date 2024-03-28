from decimal import Decimal as dec

from . import constants
from . import units
from .quantities import Quantity

# Make unit and constant dictionaries available directly in this namespace
unit = units.unit_reg
constant = constants.constant_reg
