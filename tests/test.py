from decimal import Decimal as dec

from quanstants import (
    units as qu,
    prefixes as qp,
    constants as qc,
    Quantity,
    QuanstantsConfig,
)

#QuanstantsConfig.INVERSE_UNIT = "SLASH"
a = 5.2 * qu.m * qu.s**-1

b = 3 * qu.s**-1 * qu.m
print(a)
print(a.__repr__())
print(a.number.__repr__())
print(b)
print(a.canonical())
print(b.canonical())
print(a.unit == b.unit)
print(a.canonical().unit.__repr__())
print(b.canonical().unit.__repr__())
print(a.canonical().unit == b.canonical().unit)
print(a == b)

print("----------------")
c = 2*qu.kg
d = 2*(qp.k * qu.g)
print(c.base())
print(d.base())
print(c.base().unit == d.base().unit)
print(c == d)
print(qu.kg.__repr__())

print("----------------")
e = dec("0.6096") * qu.m
f = 2 * qu.ft
print(e > f)
g = 3 * qu.ft
print(f < g)
h = e + g

i = 3 * qu.J**-2 * qu.m * qu.s**-1 * qu.kg
print(i)
print(i.canonical())
print("----------------")



