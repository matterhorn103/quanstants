"""Namespace to contain all the constants, making them useable with qc.c notation."""

from ..exceptions import AlreadyDefinedError

# Note there is no need to import constants from other modules as they are
# added to this namespace programmatically


def add(name: str, constant):
    """Add a `Constant` object to the module under the provided name.
    This method provides a safe way to add constants to the module.
    Names in the module's namespace cannot be overwritten in this way, and attempting to add a
    constant under a name that is already defined will raise an `AlreadyDefinedError`.
    If it is necessary to redefine a name, do it by setting the variable in the normal way i.e.
    `constants.already_assigned_name = new_value`.
    """
    if name in globals():
        raise AlreadyDefinedError
    globals()[name] = constant

def list_names():
    """Return a list of all constant names in the namespace, in human-readable format i.e. as strings.
    Essentially just return the value of `constants.globals().keys()` but with anything that isn't a constant
    filtered out.
    Note that a) the values returned are variable names as strings, not the `Constant` objects
    themselves and b) a constant is typically listed multiple times under different names, as well as
    under its symbol if it has `canon_symbol=True`.
    """
    filtered_names = {name for name in globals().keys() if name[0] != "_"}
    return list(filtered_names)

def list_constants():
    """Return a list of all `Constant` objects currently in the namespace.
    Unlike `list_names()`, the values are `Constant` objects not strings, and each constant is only
    listed once, regardless of how many names it is registered under in the namespace.
    """
    filtered_names = list_names()
    # Using the set approach doesn't work because Constants are currently unhashable - can this be fixed?
    # unique_constants = {self.__dict__[name] for name in filtered_names}
    # return list(unique_constants)
    unique_constants = []
    for name in filtered_names:
        if globals()[name] not in unique_constants:
            unique_constants.append(globals()[name])
    return unique_constants
