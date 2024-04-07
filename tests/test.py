from quanstants import prefixes as pr, units as un, constants as co

a = 30 * (pr.k * un.W) # 30 kW

b = 400 * un.J # 400 J

c = 5 * (pr.k * un.J) # 5 kJ

d = 6000 * un.J * un.mol**-1 # 6000 J/mol

e = 3 * c * un.mol**-1 # 3 kJ/mol

for qu in [a,b,c,d,e]:
    print(qu)
    print(qu.base())

f = pr.k * un.J
print(str(f))
print(repr(f))
print(f.name)
print(type(f))

print(b.base())
print(f.base())

g = b.base() / f.base()
print(g.__repr__())

h = g.cancel()
print(h)

print(h.number)
print(h.unit)
print(type(h.unit))

i = h.unit * f
print(i)

j = b.to(pr.k * un.J)
print(j.__repr__())

k = e.to(un.J * un.mol**-1)
print(k.__repr__())

l = pr.k * un.J
print(repr(l))

m = (1*l) / (1*f)
print(m.base().cancel())

n = 3 * un.m**2
print(n)
print(repr(n.to(un.m)))

o = 2 * un.m
p = 50 * (pr.centi * un.metre)
q = o + p
print(q)