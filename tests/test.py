from decimal import Decimal as dec

from quanstants import prefixes as pr, units as un, constants as co

a = 3 * un.m * un.s**-1

b = 3 * un.s**-1 * un.m

print(a)
print(b)
print(a.canonical())
print(b.canonical())
print(a.unit == b.unit)
print(a.canonical().unit.__repr__())
print(b.canonical().unit.__repr__())
print(a.canonical().unit == b.canonical().unit)
print(a == b)

print("----------------")
c = 2*un.kg
d = 2*(pr.k * un.g)
print(c.base())
print(d.base())
print(c.base().unit == d.base().unit)
print(c == d)

print("----------------")
e = dec("0.6096") * un.m
f = 2 * un.ft
print(e > f)
g = 3 * un.ft
print(f < g)

