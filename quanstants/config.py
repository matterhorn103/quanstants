import __main__
from pathlib import Path
import tomllib

# Define available options, their default values, and the respective docstring
_config_dict = {
    "ROUNDING_MODE": {
        "default": "ROUND_HALF_UP",
        "doc": (
"""Set the rounding behaviour for internal Decimal objects."""
        ),
    },
    "ROUND_TO": {
        "default": "PLACES",
        "doc": (
"""Controls the default behaviour of round(Quantity) and Quantity.round().

Options: "PLACES", "FIGURES", "UNCERTAINTY".
"""
        ),
    },
    "ROUND_PAD": {
        "default": True,
        "doc": (
"""Like siunitx, determines if extra zeroes should be added to reach desired sigfigs."""
        ),
    },
    "CONVERT_FLOAT_AS_STR": {
        "default": True,
        "doc": (
"""Whether floats should be converted to Decimal via str or directly."""
        ),
    },
    "AUTO_CANCEL":{
        "default": True,
        "doc": (
"""Whether identical units should be cancelled and combined automatically after arithmetic."""
        ),
    },
    "LITRE_SYMBOL":{
        "default": "L",
        "doc": (
"""Whether to use a lowercase l or uppercase L as the symbol for the litre."""
        ),
    },
    "PRETTYPRINT": {
        "default": True,
        "doc": (
"""Whether to format printed strings in a nice way.

Changing this setting also changes various other settings to match.
"""
        ),
    },
    "UNCERTAINTY_STYLE": {
        "default": "PARENTHESES",
        "doc": (
"""How uncertainties of quantities should be formatted.

Options: "PARENTHESES", "PLUSMINUS".
"""
        ),
    },
    "UNIT_SEPARATOR": {
        "default": " ",
        "doc": (
"""Unicode character used to separate unit terms.

Default is a normal space.
"""
        ),
    },
    "INVERSE_UNIT": {
        "default": "NEGATIVE_SUPERSCRIPT",
        "doc": (
"""How units with negative exponents should be formatted.

Options: "NEGATIVE_SUPERSCRIPT", "SLASH".
"""
        ),
    },
    "UNICODE_SUPERSCRIPTS": {
        "default": True,
        "doc": (
"""Whether exponents of units should be formatted with Unicode superscript characters."""
        ),
    },
    "GROUP_DIGITS": {
        "default": False,
        "doc": (
"""Whether numbers should be printed with separators between groups of digits."""
        ),
    },
    "GROUP_DIGITS_STYLE": {
        "default": "all",
        "doc": (
"""Whether to group digits before (`"integer"`) the decimal point, after (`"decimal"`), or both (`"all"`)."""
        ),
    },
    "GROUP_SEPARATOR": {
        "default": " ",
        "doc": (
"""The Unicode character which should be put between groups of digits.

The default is the U+2009 " " "THIN SPACE", which matches `siunitx`.
Naturally this will appear as a normal-width space in a monospaced font.

Other typical options include a normal space " ", a comma ",", a full stop ".",
or an underscore "_".
"""
        ),
    },
}

class QuanstantsConfig:
    def __init__(self):
        # Initiate options dynamically
        for key, value in _config_dict.items():
            # Set default values
            setattr(self, f"_{key}", value["default"])
            # Make each variable a property with a simple getter
            # Handle setting of properties within __setattr__()
            setattr(type(self),
                key,
                property(
                    fget=lambda self, k=key: getattr(self, f'_{k}'),
                    doc=value["doc"],
                )
            )
    
    def __setattr__(self, name, value):
        if hasattr(self, f"_{name}"):
            if name == "PRETTYPRINT":
                self._UNICODE_SUPERSCRIPTS = value
                self._GROUP_DIGITS = value
                self._PRETTYPRINT = value
            else:
                super().__setattr__(f"_{name}", value)
        else:
            super().__setattr__(name, value)
    
    def read_config(self, file):
        """Read configuration from a TOML file.
        
        Options should be specified as key-value pairs at the top level (not in a table).
        For example:

        ```toml
        ROUND_TO = "FIGURES"
        CONVERT_FLOAT_AS_STR = false
        ```
        """
        with open(file, "b") as f:
            config = tomllib.load(f)
        for key, value in config.items():
            setattr(self, key, value)
    
    def find_config(self):
        """Look for a configuration file called `quanstants.toml` in a couple of set locations.
        
        Currently, the following locations are checked in the given order:

        1. The directory of the main script, from `__main__.__file__`
        2. The current working directory, from `pathlib.Path.cwd()`

        Any options specified in the first file found will override the defaults.
        Other files with lower priority in the above list will be completely ignored.
        """
        paths_to_check = []
        try:
            paths_to_check.append(Path(__main__.__file__).parent)
        except AttributeError:
            pass
        paths_to_check.append(Path.cwd())
        while (toml_path := None) is None:
            for path in paths_to_check:
                if (path / "quanstants.toml").exists():
                    toml_path = (path / "quanstants.toml")
                else:
                    continue
            break
        if toml_path:
            self.read_config(toml_path)
        

# Create instance of config class that will be passed around to other modules
# The user should interact with the instance (which is added to the main namespace in __init__.py),
# not with the class itself
quanfig = QuanstantsConfig()