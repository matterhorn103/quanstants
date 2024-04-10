from decimal import Decimal as dec

from quanstants import prefixes, units, constants, Quantity


assert constants.proton_mass.value.to(units.dalton) == Quantity(1.007276466621, units.Da, uncertainty=0.000000000053)