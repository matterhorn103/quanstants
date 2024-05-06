import math
from decimal import Decimal as dec

from ..unit import Unitless, BaseUnit, CompoundUnit, DerivedUnit, Factor
from ..quantity import Quantity
from ..si import *
from ..prefix_defs import centimetre

# fmt: off

angstrom = DerivedUnit("Å", "angstrom", Quantity(1e-10, m), canon_symbol=True, alt_names=["ångström"])
reciprocal_centimetre = CompoundUnit((Factor(centimetre, -1),), name="reciprocal_centimetre", add_to_reg=True, alt_names=["reciprocal_centimeter", "inverse_centimetre", "inverse_centimeter", "wavenumber"])

# fmt: on
