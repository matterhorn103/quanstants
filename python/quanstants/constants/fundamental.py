"""The fundamental constants that in turn define the SI base units."""

from ..constant import Constant
from .. import units as qu

# fmt: off

# SI base fundamental constants
caesium_hyperfine_transition = Constant("Δν_Cs", "caesium_hyperfine_transition", "9192631770", qu.Hz, canon_symbol=True)
speed_of_light = Constant("c", "speed_of_light", "299792458", qu.m * qu.s**-1, canon_symbol=True)
Planck_constant = Constant("h", "Planck_constant", "6.62607015E-34", qu.J * qu.s, canon_symbol=True)
elementary_charge = Constant("e", "elementary_charge", "1.602176634E-19", qu.C, canon_symbol=True)
Boltzmann_constant = Constant("k_B", "Boltzmann_constant", "1.380649E-23", qu.J * qu.K**-1, canon_symbol=True)
Avogadro_constant = Constant("N_A", "Avogadro_constant", "6.02214076E23", qu.mol**-1, canon_symbol=True)
luminous_efficacy_540_THz = Constant("K_cd", "luminous_efficacy_540_THz", "683", qu.lm * qu.W**-1, canon_symbol=True)

# fmt: on
