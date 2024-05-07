import __main__
from pathlib import Path
import tomllib

# Available options, their default values, and their respective docstrings, are contained in config.toml

class QuanstantsConfig:
    def __init__(self):
        # Open default options from file
        with open(Path(__file__).with_name("config.toml"), "rb") as f:
            defaults = tomllib.load(f)
        # Initiate options dynamically
        # Tables within config are just for organization, ignore them
        for section in defaults.keys():
            for key, value in defaults[section].items():
                # Set default values
                setattr(self, f"_{key}", value["default"])
                # Make each variable a property with a simple getter
                # Handle setting of properties within __setattr__()
                setattr(
                    type(self),
                    key,
                    property(
                        fget=lambda self, k=key: getattr(self, f"_{k}"),
                        doc=value["doc"],
                    ),
                )

    def __setattr__(self, name, value):
        if hasattr(self, f"_{name}"):
            # Setting some options needs to trigger changes to other options too
            if name == "PRETTYPRINT":
                self._UNICODE_SUPERSCRIPTS = value
                self._GROUP_DIGITS = value
                self._PRETTYPRINT = value
            else:
                super().__setattr__(f"_{name}", value)
        else:
            super().__setattr__(name, value)

    def read_config(self, file):
        """Read configuration from a quanstants TOML file.

        Options should be specified as key-value pairs in the respective tables:

        ```toml
        [config.rounding]
        ROUND_TO = "FIGURES"

        [config.conversion]
        CONVERT_FLOAT_AS_STR = false
        ```
        """
        with open(file, "rb") as f:
            toml = tomllib.load(f)
        # Just use config table for now, other tables will be for e.g. custom units
        config = toml["config"]
        # Tables within config are just for organization, ignore them
        for section in config.keys():
            for key, value in config[section].items():
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
                    toml_path = path / "quanstants.toml"
                else:
                    continue
            break
        if toml_path:
            self.read_config(toml_path)


# Create instance of config class that will be passed around to other modules
# The user should interact with the instance (which is added to the main namespace in __init__.py),
# not with the class itself
quanfig = QuanstantsConfig()
