from decimal import Decimal as dec

from quanstants import (
    units as qu,
    prefixes as qp,
    constants as qc,
    Quantity,
    quanfig,
)

a = 123.456789 * qu.m
b = (123.456789 * qu.m).plusminus(0.00054)
c = (123.456789 * qu.m).plusminus(0.000054)
d = dec("123.45")
e = 123.45 * qu.m
g = ("6.67430e-11" * qu.newton * qu.metre**2 * qu.kilogram**-2).with_uncertainty("0.00015e-11")

print(f"{a = }")
#print(f"{b = }")
#print(f"{c = }")
#print(f"{d = }")
#print(f"{e = }")
print(f"{g = }")
print(g)
print("---")
#print(f"{a.round() = }")
#print(f"{b.round() = }")
#print(f"{c.round() = }")
#print(f"{round(d, 1) = }")
#print(f"{e.round_to_places(1) = }")
#print(f"{b.round_to_figures(2) = }")
#print(f"{b.round_to_uncertainty(2) = }")
#print(f"{b.round_to_uncertainty(3) = }")
#print(f"{b.round_uncertainty(1, mode="PLACES") = }")
print(f"{g.round() = }")
print(f"{g.round_to_places(5) = }")
print(f"{g.round_to_figures(3) = }")
print(f"{g.round_to_uncertainty(0) = }")


quanfig.ROUND_BEFORE_PRINT = True
print(a)
