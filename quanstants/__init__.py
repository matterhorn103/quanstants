"""
Users are expected to import with `from quanstants import units, constants`
and optionally `from quanstants import Quantity`.
"""

from decimal import Decimal as dec

### SETUP ###
# Setup configuration first in case the user's preferences affect initial setup
from .config import quanfig

quanfig.find_toml()
quanfig.load_toml(["config"])

# Initialize namespace modules
from . import units
from . import prefixes
from . import constants

# Import key classes in order of dependency
from . import quantity
from . import unit
from . import prefix
from . import constant

# Also make Quantity class available directly in this namespace
from .quantity import Quantity

### NAMESPACE POPULATION ###
# Definition modules are "unused" imports but do need to be imported so that
# they are initiated and the unit/prefix/constant dictionaries get populated

# The SI module is actually loaded already due to being
# used in other modules, but import here for clarity
from . import si

# Make sure prefixes are loaded as later units rely on them
from .prefixes import default

# Then import whichever optional unit and constant modules should be provided by default
from .units import default, prefixed
from .constants import default

# Now load any custom units and constants defined by the user in their toml
quanfig.load_toml(["units", "constants"])
