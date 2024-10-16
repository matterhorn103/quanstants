"""The British imperial system."""

import sys
from pathlib import Path

from ..unit import DerivedUnit
from ..quantity import Quantity
from .base import *
from .si import litre, hour
from .temperatures import degreeFahrenheit
from ..loader import load_units_file

load_units_file(Path(__file__).with_suffix(".toml"), module=sys.modules[__name__])
