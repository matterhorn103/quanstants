from ..unit import CompoundUnit, DerivedUnit
from ..quantity import Quantity
from ..si import *
from ..units.prefixed import centimetre

# fmt: off

angstrom = DerivedUnit("Å", "angstrom", Quantity(1e-10, m), canon_symbol=True, alt_names=["ångström"])
reciprocal_centimetre = CompoundUnit(((centimetre, -1),), name="reciprocal_centimetre", add_to_namespace=True, alt_names=["reciprocal_centimeter", "inverse_centimetre", "inverse_centimeter", "wavenumber"])

# fmt: on
