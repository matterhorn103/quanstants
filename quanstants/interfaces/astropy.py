from astropy import units as u

from ..units import astro, base, common, imperial, si

# Key astropy classes, many of which to be accounted for:
"""
class astropy.units.quantity.Quantity(value: QuantityLike, unit=None, dtype=<class 'numpy.inexact'>, copy=True, order=None, subok=False, ndmin=0)

CompositeUnit = CompoundUnit
Decibel
DecibelUnit
Dex
DexUnit
Equivalency
FunctionQuantity
FunctionUnitBase
IrreducibleUnit = BaseUnit
LogQuantity
LogUnit
MagUnit
Magnitude
NamedUnit
PhysicalType
PrefixUnit = PrefixedUnit
Quantity
QuantityInfo
QuantityInfoBase
SpecificTypeQuantity
StructuredUnit
Unit
UnitBase ~ AbstractUnit
UnrecognizedUnit
"""

# Units that need to be convertible to quanstants:
"""
# astropy.units.__init__.py does:

from . import (
    astrophys,
    cgs,
    core,
    decorators,
    misc,
    photometric,
    physical,
    quantity,
    si,
    structured,
)
from .astrophys import *
from .cgs import *
from .core import *
from .core import set_enabled_units
from .decorators import *
from .misc import *
from .photometric import *
from .physical import *
from .quantity import *
from .si import *
from .structured import *

# Where each imported module contains the following:

# si
a 365.25 days = qu.julian_year
A
Angstrom
arcmin
arcsec
Bq
C
cd
Ci
cm
d
deg
deg_C
eV
F
fortnight
g
h
H
hourangle
Hz
J
K
kg
l
lm
lx
m
mas
micron
min
mol
N
Ohm
Pa
%
rad
s
S
sday sidereal day 86164.091 s
sr
t
T
uas micro arc second 10e-6 "
V
W
Wb
wk
yr 365.25 days

# cgs
abC
Ba
Bi
C
cd
cm
D
deg_C
dyn
erg
Fr
g
G
Gal
K
k
mol
Mx
Oe
P
rad
s
sr
St
statA

# astrophys
adu
AU
beam
bin
chan
ct
DN
earthMass
earthRad
electron
jupiterMass
jupiterRad
Jy
lsec
lyr
pc
ph
R
Ry
solLum
solMass
solRad
Sun

# misc
bar
barn
bit
byte
cycle
M_e
M_p
pix
spat
Torr
u
vox

#photometric
AB
Bol
bol
mgy
ST

#imperial (Uses US definitions!)
ac
BTU
cal
cup (US)
deg_F
deg_R
foz (US)
ft
fur
gallon (US)
hp
inch
kcal
kip
kn
lb
lbf
mi
mil
nmi
oz
pint (US)
psi
quart (US)
slug
st
tbsp (US)
ton
tsp (US)
yd

# cds
%
---
\h
A
a
a0
AA
al
alpha
arcmin
arcsec
atm
AU
bar
barn
bit
byte
C
c
cal
cd
Crab
ct
D
d
deg
dyn
e
eps0
erg
eV
F
G
g
gauss
geoMass
H
h
hr
Hz
inch
J
JD
jovMass
Jy
K
k
l
lm
Lsun
lx
lyr
m
mag
mas
me
min
MJD
mmHg
mol
mp
Msun
mu0
muB
N
Ohm
Pa
pc
ph
pi
pix
ppm
R
rad
Rgeo
Rjup
Rsun
Ry
S
s
sr
Sun
T
t
u
V
W
Wb
yr
µas

#logarithmic
# these are classes, need to think carefully how to convert:
LogUnit([physical_unit, function_unit])
MagUnit([physical_unit, function_unit])
DexUnit([physical_unit, function_unit])
DecibelUnit([physical_unit, function_unit])
LogQuantity(value[, unit, dtype, copy, ...])
Magnitude(value[, unit, dtype, copy, order, ...])
Decibel(value[, unit, dtype, copy, order, ...])
Dex(value[, unit, dtype, copy, order, ...])
"""
astropy_to_quanstants = {

}