"""Typographic units."""

from ..unit import DerivedUnit
from ..quantity import Quantity
from .base import *
from .imperial import inch

# fmt: off

point = DerivedUnit("pt", "point", Quantity("3.5277777777777777777778e-4", m))
pica = DerivedUnit("P", "pica", Quantity("8.7333333333333333333333e-4", m))

# fmt: on
