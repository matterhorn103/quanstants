"""Namespace to contain all the prefixes, making them useable with qp.n notation"""

from importlib import import_module

from ..config import quanfig
from ..exceptions import AlreadyDefinedError


### HELPER FUNCTIONS ###
# TODO Find a way to put these in a module without breaking the use of globals()

def add(name: str, prefix):
    if name in globals():
        raise AlreadyDefinedError
    globals()[name] = prefix


### NAMESPACE POPULATION ###

# Dynamically import whichever unit submodules should be loaded at import time
# The defaults are specified in `quanstants/config.toml`
# User can specify in `quanstants.toml` which submodules should be loaded
for definition_set in getattr(quanfig, "PREFIXES"):
    import_module(f".{definition_set}", f"quanstants.prefixes")

# Now load any custom units defined by the user in their toml
quanfig.load_toml(["prefixes"])


### KEY CLASSES ###

# Just make them available where the user might expect them

from ..prefix import Prefix