from ..unit import CompoundUnit, DerivedUnit
from ..quantity import Quantity
from .base import *
from ..units.prefixed import centimetre

# fmt: off

angstrom = DerivedUnit("Å", "angstrom", Quantity(1e-10, m), alt_names=["ångström"], canon_symbol=True)
reciprocal_centimetre = CompoundUnit(((centimetre, -1),), name="reciprocal_centimetre", alt_names=["reciprocal_centimeter", "inverse_centimetre", "inverse_centimeter", "wavenumber"], add_to_namespace=True)

# fmt: on
