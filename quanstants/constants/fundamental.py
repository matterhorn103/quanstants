from ..constant import Constant
from ..quantity import Quantity
from ..unit import unit_reg as u
from ..prefix import prefix_reg as p

# fmt: off

# SI base fundamental constants
caesium_hyperfine_transition = Constant("Δν_Cs", "caesium_hyperfine_transition", "9192631770", u.Hz, canon_symbol=True)
speed_of_light = Constant("c", "speed_of_light", "299792458", u.m * u.s**-1, canon_symbol=True)
Planck_constant = Constant("h", "Planck_constant", "6.62607015E-34", u.J * u.s, canon_symbol=True)
elementary_charge = Constant("e", "elementary_charge", "1.602176634E-19", u.C, canon_symbol=True)
Boltzmann_constant = Constant("k_B", "Boltzmann_constant", "1.380649E-23", u.J * u.K**-1, canon_symbol=True)
Avogadro_constant = Constant("N_A", "Avogadro_constant", "6.02214076E23", u.mol**-1, canon_symbol=True)
luminous_efficacy_540_THz = Constant("K_cd", "luminous_efficacy_540_THz", "683", u.lm * u.W**-1, canon_symbol=True)

# fmt: on
