"""Namespace to contain all the prefixes, making them useable with qp.n notation"""

from ..exceptions import AlreadyDefinedError

# Note there is no need to import prefixes from other modules as they are
# added to this namespace programmatically

def add(name: str, prefix):
    if name in globals():
        raise AlreadyDefinedError
    globals()[name] = prefix
