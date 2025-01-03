from decimal import Decimal as dec

from quanstants import (
    units as qu,
    prefixes as qp,
    constants as qc,
    Quantity,
    quanfig,
)


# assert qc.proton_mass.value.to(qu.dalton) == Quantity(1.007276466621, qu.Da, uncertainty=0.000000000053)
