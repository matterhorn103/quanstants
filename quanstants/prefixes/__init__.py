"""Namespace to contain all the prefixes, making them useable with qp.n notation"""

class PrefixAlreadyDefinedError(Exception):
    pass

def add(name: str, prefix):
    if name in globals():
        raise PrefixAlreadyDefinedError
    globals()[name] = prefix
