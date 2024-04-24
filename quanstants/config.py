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
        "default": False,
        "doc": (
"""Whether identical units should be cancelled and combined automatically after arithmetic."""
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
    "PRETTYPRINT": {
        "default": True,
        "doc": (
"""Whether to format printed strings in a nice way.

Changing this setting also changes `UNICODE_SUPERSCRIPTS` to match.
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
                self._PRETTYPRINT = value
            else:
                super().__setattr__(f"_{name}", value)
        else:
            super().__setattr__(name, value)
    
    def read_config(self, file):
        """Read configuration from a TOML file.
        
        Options should be specified as key-value pairs at the top level (i.e. not in a table).
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

# Create instance of config class that will be passed around to other modules
# The user should interact with the instance (which is added to the main namespace in __init__.py),
# not with the class itself
quanfig = QuanstantsConfig()