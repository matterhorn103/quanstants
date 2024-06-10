from astropy import units as u
import astropy.units.cgs
import astropy.units.astrophys
import astropy.units.misc
import astropy.units.photometric
import astropy.units.imperial
import astropy.units.cds

from ..quantity import Quantity
from .. import units as qu
from ..units import astro, base, chemistry, common, imperial, prefixed, si
from ..prefixes.metric import milli, micro

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
LogQuantity = LogarithmicQuantity
LogUnit = LogarithmicUnit
MagUnit
Magnitude
NamedUnit
PhysicalType
PrefixUnit = PrefixedUnit
Quantity = Quantity
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
"""

astropy_to_quanstants = {
# si
u.a: astro.julian_year,
u.A: base.ampere,
u.Angstrom: chemistry.angstrom,
u.arcmin: si.arcminute,
u.arcsec: si.arcsecond,
u.Bq: si.becquerel,
u.C: si.coulomb,
u.cd: base.candela,
#u.Ci: TODO
u.cm: prefixed.centimetre,
u.d: si.day,
u.deg: si.degree,
u.deg_C: si.degreeCelsius,
u.eV: si.electronvolt,
u.F: si.farad,
#u.fortnight: TODO,
u.g: si.gram,
u.h: si.hour,
u.H: si.henry,
#u.hourangle: TODO,
u.Hz: si.hertz,
u.J: si.joule,
u.K: base.kelvin,
u.kg: base.kilogram,
u.l: si.litre,
u.lm: si.lumen,
u.lx: si.lux,
u.m: base.metre,
u.mas: astro.milliarcsecond,
u.micron: prefixed.micrometre,
u.min: si.minute,
u.mol: base.mole,
u.N: si.newton,
u.Ohm: si.ohm,
u.Pa: si.pascal,
u.pct: common.percent,
u.rad: si.radian,
u.s: base.second,
u.S: si.siemens,
u.sday: astro.sidereal_day,
u.sr: si.steradian,
u.t: si.tonne,
u.T: si.tesla,
u.uas: astro.microarcsecond,
u.V: si.volt,
u.W: si.watt,
u.Wb: si.weber,
#u.wk: TODO,
u.yr: astro.julian_year,
# cgs,
#u.cgs.abC: TODO,
#u.cgs.Ba: TODO,
#u.cgs.Bi: TODO,
#u.cgs.C: TODO,
#u.cgs.cd: TODO,
#u.cgs.cm: TODO,
#u.cgs.D: TODO,
#u.cgs.deg_C: TODO,
#u.cgs.dyn: TODO,
#u.cgs.erg: TODO,
#u.cgs.Fr: TODO,
#u.cgs.g: TODO,
#u.cgs.G: TODO,
#u.cgs.Gal: TODO,
#u.cgs.K: TODO,
#u.cgs.k: TODO,
#u.cgs.mol: TODO,
#u.cgs.Mx: TODO,
#u.cgs.Oe: TODO,
#u.cgs.P: TODO,
#u.cgs.rad: TODO,
#u.cgs.s: TODO,
#u.cgs.sr: TODO,
#u.cgs.St: TODO,
#u.cgs.statA: TODO,
# astrophys
#u.astrophys.adu: TODO,
u.astrophys.AU: si.astronomical_unit,
#u.astrophys.beam: TODO all unitless?,
#u.astrophys.bin: TODO all unitless?,
#u.astrophys.chan: TODO all unitless?,
#u.astrophys.ct: TODO all unitless?,
#u.astrophys.DN: TODO,
#u.astrophys.earthMass: TODO,
#u.astrophys.earthRad: TODO,
#u.astrophys.electron: TODO,
#u.astrophys.jupiterMass: TODO,
#u.astrophys.jupiterRad: TODO,
#u.astrophys.Jy: TODO,
#u.astrophys.lsec: TODO,
#u.astrophys.lyr: TODO,
#u.astrophys.pc: TODO,
#u.astrophys.ph: TODO,
#u.astrophys.R: TODO,
#u.astrophys.Ry: TODO,
#u.astrophys.solLum: TODO,
#u.astrophys.solMass: TODO,
#u.astrophys.solRad: TODO,
#u.astrophys.Sun: TODO,
# misc
#u.misc.bar: common.bar TODO,
#u.misc.barn: TODO,
u.misc.bit: common.bit,
u.misc.byte: common.byte,
#u.misc.cycle: TODO,
#u.misc.M_e: TODO,
#u.misc.M_p: TODO,
#u.misc.pix: TODO,
#u.misc.spat: TODO,
#u.misc.Torr: TODO,
#u.misc.u: TODO,
#u.misc.vox: TODO,
#photometric
#u.photometric.AB: TODO,
#u.photometric.Bol: TODO,
#u.photometric.bol: TODO,
#u.photometric.mgy: TODO,
#u.photometric.ST: TODO,
#imperial (Uses US definitions!)
#u.ac,
#u.BTU,
#u.cal,
#u.cup (US),
#u.deg_F,
#u.deg_R,
#u.foz (US),
#u.ft,
#u.fur,
#u.gallon (US),
#u.hp,
#u.inch,
#u.kcal,
#u.kip,
#u.kn,
#u.lb,
#u.lbf,
#u.mi,
#u.mil,
#u.nmi,
#u.oz,
#u.pint (US),
#u.psi,
#u.quart (US),
#u.slug,
#u.st,
#u.tbsp (US),
#u.ton,
#u.tsp (US),
#u.yd,
## cds
#u.%,
#u.---,
#u.\h,
#u.A,
#u.a,
#u.a0,
#u.AA,
#u.al,
#u.alpha,
#u.arcmin,
#u.arcsec,
#u.atm,
#u.AU,
#u.bar,
#u.barn,
#u.bit,
#u.byte,
#u.C,
#u.c,
#u.cal,
#u.cd,
#u.Crab,
#u.ct,
#u.D,
#u.d,
#u.deg,
#u.dyn,
#u.e,
#u.eps0,
#u.erg,
#u.eV,
#u.F,
#u.G,
#u.g,
#u.gauss,
#u.geoMass,
#u.H,
#u.h,
#u.hr,
#u.Hz,
#u.inch,
#u.J,
#u.JD,
#u.jovMass,
#u.Jy,
#u.K,
#u.k,
#u.l,
#u.lm,
#u.Lsun,
#u.lx,
#u.lyr,
#u.m,
#u.mag,
#u.mas,
#u.me,
#u.min,
#u.MJD,
#u.mmHg,
#u.mol,
#u.mp,
#u.Msun,
#u.mu0,
#u.muB,
#u.N,
#u.Ohm,
#u.Pa,
#u.pc,
#u.ph,
#u.pi,
#u.pix,
#u.ppm,
#u.R,
#u.rad,
#u.Rgeo,
#u.Rjup,
#u.Rsun,
#u.Ry,
#u.S,
#u.s,
#u.sr,
#u.Sun,
#u.T,
#u.t,
#u.u,
#u.V,
#u.W,
#u.Wb,
#u.yr,
#u.µas,
}
"""
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

# TODO cope with astropy units that have a scale
def from_astropy(obj):
    if isinstance(obj, astropy.units.UnitBase):
        try:
            return astropy_to_quanstants[obj]
        except KeyError:
            for name in obj.names:
                try:
                    return qu.parse(name)
                except AttributeError:
                    continue 
    elif isinstance(obj, astropy.units.Quantity):
        return Quantity(obj.value, from_astropy(obj.unit))