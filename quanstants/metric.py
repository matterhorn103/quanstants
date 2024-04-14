import math
from decimal import Decimal as dec

from .unit import Unitless, BaseUnit, DerivedUnit
from .quantity import Quantity
from .si import *

# fmt: off

# CODATA 2018
#Angstrom_star = Constant(None, "Angstrom_star", Quantity("1.00001495e-10", m, "0.00000090e-10"), canon_symbol=False)

# Other
watthour = DerivedUnit("Wh", "watthour", Quantity("3.6E6", joule), canon_symbol=True)

# Temperature
#celsius
#fahrenheit
#rankine
#delisle
#newton
#reaumur
#romer

# Standard states
# standard atm
# bar
# standard pressure

# carat
# point

# fmt: on