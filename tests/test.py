from decimal import Decimal as dec

from quanstants import prefixes as pr, units as un, constants as co, unit_defs

a = 5.2 * un.m * un.s**-1

b = 3 * un.s**-1 * un.m

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
c = 2*un.kg
d = 2*(pr.k * un.g)
print(c.base())
print(d.base())
print(c.base().unit == d.base().unit)
print(c == d)
print(un.kg.__repr__())
print(unit_defs.kilogram.__repr__())

print("----------------")
e = dec("0.6096") * un.m
f = 2 * un.ft
print(e > f)
g = 3 * un.ft
print(f < g)
h = e + g
i = a + e
print("----------------")

class TestClass:
    def add(self, name):
        setattr(self, name, 12)

test = TestClass()
print(test)
print(type(test))
test2 = TestClass()
print(test2)
print(type(test2))

setattr(test, "ex1", 10)
print(test.ex1)
test.add("ex2")
print(test.ex2)

