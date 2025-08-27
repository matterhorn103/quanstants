from pathlib import Path
import tomllib


def load_units_file(units_file: Path, module = None):
    """Create units from a specification contained in a table read from a TOML file.
    
    Units should be listed in tables according to their type.
    Each unit will be created as an instance of that type.
    If `module` is provided, the units will be assigned to variables within that module
    with the same names as the keys of each unit's table.

    Keyword arguments for the unit type's constructor should be supplied as
    key/value pairs.
    If `name` is not given, the key of the unit's table will be passed as the unit's
    name.
    Arguments that should be `Quantity` objects should be given as a table with
    key/value pairs for the arguments of the `Quantity` constructor.

    For example:

    ```toml
    [units.derived]
    thaum.symbol = "thm"
    thaum.value.number = "3.595e16"
    thaum.value.unit = "J"
    thaum.canon_symbol = true
    ```

    will create a single unit using:

    ```python
    DerivedUnit(
        symbol="thm",
        name="thaum",
        value=Quantity("3.595e16", "J"),
        canon_symbol=True,
    )
    ```
    """
    with open(units_file, "rb") as f:
        contents = tomllib.load(f)

    if "units" in contents:
        create_units(contents["units"], module=module)


def create_units(units_table: dict, module = None):
    """Create units from a table/dict.
    
    Units are contained in tables according to their type.
    Each unit will be created as an instance of that type.
    If `module` is provided, the unit will be assigned to a variable within that module
    with the same name as the key of the unit's table.

    Keyword arguments for the unit type's constructor should be supplied as key/value
    pairs.
    If `name` is not given, the key of the unit's table will be passed as the unit's
    name.
    Arguments that should be `Quantity` objects should be given as a table with
    key/value pairs for the arguments of the `Quantity` constructor.

    For example:

    ```python
    create_units({
        "derived": {
            "thaum": {
                "symbol": "thm",
                "value": {"number": "3.595e16", "unit": "J"},
                "canon_symbol": True,
            },
        },
    })
    ```

    will create a single unit using:

    ```python
    DerivedUnit(
        symbol="thm",
        name="thaum",
        value=Quantity("3.595e16", "J"),
        canon_symbol=True,
    )
    ```
    """
    from .quantity import Quantity
    from .unit import BaseUnit, UnitlessUnit, DerivedUnit, CompoundUnit
    from .temperature import TemperatureUnit
    from . import units
    
    for UnitClass in [BaseUnit, UnitlessUnit, DerivedUnit, CompoundUnit, TemperatureUnit]:
        section = UnitClass.__name__[:-4].lower()
        if section in units_table:
            for unit, kwargs in units_table[section].items():
                if "symbol" not in kwargs and section != "compound":
                    kwargs["symbol"] = None
                if "name" not in kwargs:
                    kwargs["name"] = unit
                if "value" in kwargs:
                    value_dict = kwargs["value"]
                    kwargs["value"] = Quantity(**value_dict)
                if "components" in kwargs:
                    components_list = kwargs["components"]
                    components = [[units.__dict__[c[0]], c[1]] for c in components_list]
                    kwargs["components"] = tuple(components)
                if module:
                    setattr(module, unit, UnitClass(**kwargs))
                else:
                    UnitClass(**kwargs)


def load_constants_file(constants_file: Path, module = None):
    """Create constants from a specification contained in a table read from a TOML file.
    
    Keyword arguments for `Constant()` should be supplied as key/value pairs.
    If `name` is not given, the key of the constants's table will be passed as the
    constant's name.
    Arguments that should be `Quantity` objects should be given as a table with
    key/value pairs for the arguments of the `Quantity` constructor.

    For example:

    ```toml
    [constants]
    swallow_airspeed_velocity.symbol = "swv"
    swallow_airspeed_velocity.value.number = "9"
    swallow_airspeed_velocity.value.unit = "m s-1"
    swallow_airspeed_velocity.alt_names = [ "european_swallow_airspeed_velocity", "unladen_swallow_airspeed_velocity" ]
    ```

    will create a single constant using:

    ```python
    Constant(
        symbol="swv",
        name="swallow_airspeed_velocity",
        value=Quantity("9", "m s-1"),
        alt_names=[
            "european_swallow_airspeed_velocity",
            "unladen_swallow_airspeed_velocity",
        ]
    )
    ```
    """
    with open(constants_file, "rb") as f:
        contents = tomllib.load(f)

    create_constants(contents, module=module)


def create_constants(constants_table: dict, module = None):
    """Create constants from a table/dict.
    
    Keyword arguments for `Constant()` should be supplied as key/value pairs.
    If `name` is not given, the key of the constants's table will be passed as the
    constant's name.
    Arguments that should be `Quantity` objects should be given as a table with
    key/value pairs for the arguments of the `Quantity` constructor.

    For example:

    ```python
    create_constants({
        "swallow_airspeed_velocity": {
            "symbol": "swv",
            "value": {"number": "9", "unit": "m s-1"},
            "alt_names": [
                "european_swallow_airspeed_velocity",
                "unladen_swallow_airspeed_velocity",
            ],
        },
    })
    ```

    will create a single constant using:

    ```python
    Constant(
        symbol="swv",
        name="swallow_airspeed_velocity",
        value=Quantity("9", "m s-1"),
        alt_names=[
            "european_swallow_airspeed_velocity",
            "unladen_swallow_airspeed_velocity",
        ]
    )
    ```
    """
    from .quantity import Quantity
    from .constant import Constant

    for constant, kwargs in constants_table.items():
        if "name" not in kwargs:
            kwargs["name"] = constant
        if "value" in kwargs:
            value_dict = kwargs["value"]
            kwargs["value"] = Quantity(**value_dict)
        Constant(**kwargs)
