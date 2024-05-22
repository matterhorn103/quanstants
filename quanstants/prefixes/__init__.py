"""Namespace to contain all the prefixes, making them useable with qp.n notation"""

# Note there is no need to import prefixes from other modules as they are
# added to this namespace programmatically

class PrefixAlreadyDefinedError(Exception):
    pass

def add(name: str, prefix):
    if name in globals():
        raise PrefixAlreadyDefinedError
    globals()[name] = prefix
