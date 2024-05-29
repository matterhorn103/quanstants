import __main__
from pathlib import Path
import tomllib

from platformdirs import user_config_dir

# Available options, their default values, and their respective docstrings, are
# contained in quanstants/config.toml

class QuanstantsConfig:
    def __init__(self):
        # Open default options from file
        with open(Path(__file__).with_name("config.toml"), "rb") as f:
            defaults = tomllib.load(f)
        super().__setattr__("options", {})
        self.config_table = defaults["config"]
        self.init_config(defaults["config"])
        self.toml_list = []

    def __getattribute__(self, name):
        if name in super().__getattribute__("options"):
            return super().__getattribute__("options")[name]["current"]
        else:
            return super().__getattribute__(name)

    def __setattr__(self, name, value):
        if name in self.options:
            # Make sure it's a valid option
            if "choices" in self.options[name] and value not in self.options[name]["choices"]:
                    raise TypeError(f"Value provided not amongst possible choices: {self.options[name]["choices"]}")
            else:
                # Some options need custom handling
                if name == "PRETTYPRINT":
                    self.UNICODE_SUPERSCRIPTS = value
                    self.GROUP_DIGITS = value
                # Update current value stored in options dict
                self.options[name]["current"] = value
        else:
            super().__setattr__(name, value)
    
    def print_options(self) -> None:
        for category in self.config_table.keys():
            print(f"[{category}]")
            print()
            for option, details in self.config_table[category].items():
                print(option)
                print("\t" + details["doc"].replace("\n", "\n\t"))
                print()
                if "choices" in details:
                    print(f"\tPossible values: {details["choices"]}")
                current = getattr(self, option)
                if isinstance(current, str):
                    print(f"\tCurrent value: "{current}"")
                    print(f"\tDefault value: "{details["default"]}"")
                else:
                    print(f"\tCurrent value: {current}")
                    print(f"\tDefault value: {details["default"]}")
                print()

    def find_toml(self, *dirs) -> Path | None:
        """Look for a file called `quanstants.toml` in a couple of locations.

        The path is appended to `self.toml_list` and is also returned.
        Currently, the following locations are checked in the given order:

        1. Any directory paths provided as arguments
        2. The current working directory, from `pathlib.Path.cwd()`
        3. All parent directories of the above
        4. The user's config directory at ~/.config/quanstants/quanstants.toml on macOS
        and Linux (or on Linux, $XDG_CONFIG_HOME/quanstants/quanstants.toml),
        or %USERPROFILE%/AppData/Roaming/quanstants/quanstants.toml on Windows.

        Any options specified in the first file found will override the defaults.
        Once a file has been found, any other files will be completely ignored.
        """
        paths_to_check = [*dirs]
        # Check in current working directory (in case different to above) and in parents
        paths_to_check.append(Path.cwd())
        paths_to_check.extend([Path(parent) for parent in Path.cwd().parents])
        # Check in user's config directory last
        paths_to_check.append(Path(user_config_dir("quanstants", roaming=True)))
        while (toml_path := None) is None:
            for path in paths_to_check:
                if (path / "quanstants.toml").exists():
                    toml_path = path / "quanstants.toml"
                else:
                    continue
            # Stop once all paths are checked but no quanstants.toml has been found
            break
        if toml_path:
            self.toml_list.append(toml_path)
        return toml_path
    
    def load_toml(self, sections: list[str] = None, toml_path: Path = None):
        """Load custom settings from a `quanstants.toml` file.
        
        If `sections` is not specified, all sections will be read by default.
        If `toml_path` is provided, it will also be appended to 
        If `toml_path` is not specified, the most recent path added to `self.toml_paths`
        (either by `find_toml()` or this function) is read.
        In the complete absence of an appropriate TOML file, simply returns `None`
        without loading anything.
        """
        if toml_path is None:
            try:
                toml_path = self.toml_list[-1]
            except IndexError:
                return
        else:
            self.toml_list.append(toml_path)
        with open(toml_path, "rb") as f:
            toml = tomllib.load(f)
        if sections is not None:
            valid_sections = set(sections) & set(toml.keys())
        else:
            valid_sections = toml.keys()
        if "config" in valid_sections:
            self.load_config(toml["config"])
        if "units" in valid_sections:
            self.load_units(toml["units"])
        if "constants" in valid_sections:
            self.load_constants(toml["constants"])
    
    def init_config(self, config_table: dict):
        # Initiate options dynamically
        # Tables within config are just for organization, ignore them
        # (Store them in option's table though)
        for category in config_table.keys():
            for option, details in config_table[category].items():
                # Add category to dict
                details["category"] = category
                # Make default value current value
                details["current"] = details["default"]
                # Put into options dict
                self.options[option] = details

    def load_config(self, config_table: dict):
        """Process configuration from a config table read from `quanstants.toml`.
        
        Options should be specified as key-value pairs in the respective tables, e.g.:

        ```toml
        [config.rounding]
        ROUNDING_MODE = "ROUND_HALF_DOWN"
        ROUND_PAD = false

        [config.arithmetic]
        AUTO_CANCEL = false

        [config.printing]
        INVERSE_UNIT = "SLASH"
        ```
        """
        # Tables within config are just for organization, ignore them
        for section in config_table.keys():
            for key, value in config_table[section].items():
                setattr(self, key, value)

    def load_units(self, units_table: dict):
        """Create units from a specification contained in a units table read from `quanstants.toml`.
        
        Units should be listed in tables according to their type.
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

        will create a unit using:

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
        
        for UnitClass in [BaseUnit, UnitlessUnit, DerivedUnit, CompoundUnit, TemperatureUnit]:
            section = UnitClass.__name__[:-4].lower()
            if section in units_table:
                for unit, kwargs in units_table[section].items():
                    if "name" not in kwargs:
                        kwargs["name"] = unit
                    if "value" in kwargs:
                        value_dict = kwargs["value"]
                        kwargs["value"] = Quantity(**value_dict)
                    UnitClass(**kwargs)
    
    def load_constants(self, constants_table: dict):
        """Create constants from a specification contained in a constants table read from `quanstants.toml`.
        
        Constants should be listed in a `[constants]` table.
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

        will create a constant using:

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


# Create instance of config class that will be passed around to other modules
# The user should interact with the instance (which is added to the main namespace in
# __init__.py), not with the class itself
quanfig = QuanstantsConfig()
