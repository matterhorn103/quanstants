"""
Users are expected to import the desired subpackages with e.g.
`from quanstants import units, constants`.
"""

### SETUP ###
# Setup configuration first in case the user's preferences affect initial setup
from .config import quanfig

quanfig.find_toml()
quanfig.load_toml(["config"])
