"""
Users are expected to import with `from quanstants import units, constants`
and optionally `from quanstants import Quantity`.
"""

from decimal import Decimal as dec
from importlib import import_module

### SETUP ###
# Setup configuration first in case the user's preferences affect initial setup
from .config import quanfig

quanfig.find_toml()
quanfig.load_toml(["config"])

# Initialize namespace modules
# No Unit, Prefix or Constant objects are created or loaded through this
from . import units
from . import prefixes
from . import constants

# Import key modules in order of dependency
# While doing this make the most useful classes available directly in this namespace
from .quantity import Quantity
from .unit import BaseUnit, DerivedUnit
from .prefix import Prefix
from .constant import Constant
from .temperature import TemperatureUnit, Temperature
from .log import LogarithmicUnit, LogarithmicQuantity

### NAMESPACE POPULATION ###
# Definition modules are "unused" imports but do need to be imported so that
# they are initiated and the unit/prefix/constant dictionaries get populated

# The SI base units module is actually loaded already due to being
# used in other modules, but import here for clarity
from .units import base

# Dynamically import whichever prefix, unit, and constant submodules should be loaded at
# import time
# The defaults are specified in `quanstants/config.toml`
# User can specify in `quanstants.toml` which submodules should be loaded
for namespace in ["prefixes", "units", "constants"]:
    for definition_set in getattr(quanfig, namespace.upper()):
        import_module(f".{definition_set}", f"quanstants.{namespace}")
# Each submodule imports whatever it needs directly rather than going via the namespaces
# so it should be possible to import any module individually without relying on having
# first imported any others

# Now load any custom units and constants defined by the user in their toml
quanfig.load_toml(["units", "constants"])
