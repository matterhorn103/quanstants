# quanstants

A small package to make life easier for scientists who need physical quantities, units, and constants.

`quanstants` strives to make doing scientific calculations in Python as intuitive as doing them on paper.
It is strongly inspired by the LaTeX package `siunitx`, and of course also `astropy` and `pint`.

`quanstants` makes life easy by providing the following:

* The `Quantity` object, which describes a physical quantity as a product of a number and a unit, per the SI definition, with an associated uncertainty
* Arithmetic with quantities that is intelligent and unit-aware
  * Tracking units is an extremely useful sanity check to avoid mistakes
  * Impossible mathematical operations are flagged and prevented
  * Addition/subtraction of quantities with the same dimensions but different units works, via conversion
* A large library of units including SI, metric, imperial, and non-standard units
  * Easy creation of quantities through arithmetic between numbers and units
  * Easy expression of quantities in alternative units
* A large library of physical constants covering various fields, not just physics
* Default behaviours for numbers that align with non-programmers' expectations, rather than Python defaults
  * 5.2 * m gives exactly 5.2 m, not 5.2000000000000001776356... m
  * 0.1 m + 0.2 m is exactly equal to 0.3 m (whereas in Python, `0.1 + 0.2 == 0.3` returns `False`)
  * Significant figures are maintained: 1.30 m does not become 1.3 m, and 0.15 m - 0.05 m = 0.10 m, not 0.1 m
  * Rounding gives the same result as most scientists would write if rounding manually: 1.75 m ⇒ 1.8 m, 1.65 m ⇒ 1.7 m, and -1.65 m ⇒ -1.7 m ("round half away from zero", whereas the Python default is "round half to even")
  * Rounding to a certain number of significant figures is easy

## Basic usage

Quantity creation:
```
>>> from quanstants import units as u
>>> 4 * u.metre
Quantity(4, m)
>>> 4.2 * u.m
Quantity(4.2, m)
>>> 400 * u.metre**2
Quantity(400, m²)
>>> "4E3" * u.metre * u.second**-1
Quantity(4E+3, m s⁻¹)
>>> Decimal("0.410") * u.metre / u.second
Quantity(0.410, m s⁻¹)
```
Units and prefixes:
```
>>> from quanstants import units as u, prefixes as p
>>> 25 * u.watt
Quantity(25, W)
>>> 6 * u.foot
Quantity(6, ft)
>>> 50 * u.centimetre
Quantity(50, cm)
>>> 50 * (p.centi * u.metre)
Quantity(50, cm)
>>> 50 * (p.c * u.m)
Quantity(50, cm)
```
Basic arithmetic:
```
>>> (4 * u.metre) * (3 * u.second**-1)
Quantity(12, m s⁻¹)
>>> (4 * u.metre) + (50 * u.centimetre)
Quantity(4.5, m)
>>> (4 * u.metre) + (3 * u.kilogram)
quanstants.quantity.MismatchedUnitsError: Can't add quantity in Unit(m) to quantity in Unit(kg).
>>> (3 * u.watt)**2
Quantity(9, W²)
```
Conversion:
```
>>> (50 * u.joule).base()
Quantity(50, kg m² s⁻²)
>>> a = 1 * ((p.kilo * u.Watt) * u.hour)
Quantity(1, kW h)
>>> a.to(u.joule)
Quantity(3.600E+6, J)
>>> (6 * u.foot).to(u.metre)
Quantity(1.8288, m)
>>> speed = 3 * u.m * u.s**-1
>>> time = 15 * u.s
>>> distance = speed * time
Quantity(45, m s⁻¹ s)
>>> distance.cancel()
Quantity(45, m)
```
Constants:
```
>>> from quanstants import units as u, prefixes as p, constants as c
>>> c.Planck_constant
Quantity(6.62607015E-34, J s)
>>> c.Planck
Quantity(6.62607015E-34, J s)
>>> c.proton_mass
Quantity(1.67262192369E-27, kg, uncertainty=5.1E-37)
>>> E = c.proton_mass * c.speed_of_light**2
Quantity(1.503277615985125705245525892E-10, kg m² s⁻²)
>>> E.to((p.M * u.eV)/c.c**2)
Quantity(938.2720881604903652873556338, m² s⁻² MeV m⁻² s²)
>>> E.to((p.M * u.eV)/c.c**2).cancel()
Quantity(938.2720881604903652873556338, MeV)
```
Rounding:
```
>>> a = (324.9 * u.J) * (1.674 * u.mol**-1)
Quantity(543.8826, J mol⁻¹)
>>> a.round()
Quantity(544, J mol⁻¹)
>>> a.round(2)
Quantity(543.88, J mol⁻¹)
>>> a.sigfig(4)
Quantity(543.9, J mol⁻¹)
>>> a.sigfig(2)
Quantity(5.4E+2, J mol⁻¹)
```

## Notes

The primary source of authority is the SI brochure (9th Edition v2.01 in English), followed by ISO/IEC 80000.

Following the SI/ISO example means the names of units are primarily those defined there e.g. metre, litre.
As far as possible, however, units are also made available under alternative names, particular those common in American English.
For consistency, names of constants are written first and foremost in British/Irish/European English, but should also be available under the American English equivalents.

## Comparison to astropy and pint

Both `astropy` (via its modules `constants` and `units`) and `pint` are excellent packages, but both had limitations that inspired the creation of `quanstants`.

In comparison to `astropy`, the unit and constant selection is much broader than simply those useful for astronomers, and there is no reliance on `numpy`.

Neither package makes the same opinionated decisions about number handling and rounding as `quanstants`.
