# quanstants

A small package to make life easier for scientists who need physical quantities, units, and constants.

`quanstants` strives to make doing scientific calculations in Python as intuitive as doing them on paper.
It is strongly inspired by the LaTeX package `siunitx`, and of course also `astropy` and `pint`.

What makes `quanstants` different?
An emphasis on the [principle of least astonishment](https://en.wikipedia.org/wiki/Principle_of_least_astonishment).
Importantly, applied from the perspective of a non-programmer in the natural sciences.

`quanstants` is particularly suited to situations where it is desirable to have the result match exactly that which you would produce yourself, and less suited to large numbers of complex calculations.
For example, it works very well to replace an electronic calculator, or to produce nicely formatted text or LaTeX code.

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
* Use of the `Decimal` type under the hood to avoid the [quirks of binary floating point](https://docs.python.org/3/tutorial/floatingpoint.html#tut-fp-issues), which as well as enabling the above, avoids imprecision arising from rounding errors
  * Most mathematical operations and functions are delegated to [Python's `Decimal`](https://docs.python.org/3/library/decimal.html), which in turn follows the General Decimal Arithmetic Specification and thus indirectly IEEE 754. `quanstants` simply tracks units and uncertainties on top, and aims to support all operations on `Quantity` that are supported by `Decimal`.

## Basic usage

### Quantity creation

Create quantities easily by multiplying numbers and units:
```python
>>> from quanstants import units as qu
>>> 4 * qu.metre
Quantity(4, m)
>>> print(4 * qu.metre)
4 m
```

Units are also available under their symbol:
```python
>>> 4 * qu.m
Quantity(4, m)
>>> qu.m is qu.metre                    # Both refer to same object
True
```

And under alternative names too where appropriate:
```python
>>> from quanstants import units as qu
>>> 4 * qu.meter
Quantity(4, m)
```

Use the usual exponent syntax with units:
```python
>>> 4 * qu.metre**2
Quantity(4, m²)
>>> 4 * qu.joule * qu.kilogram**-1
Quantity(4, J kg⁻¹)
```

Alternatively, use division:
```python
>>> 4 * qu.metre / qu.second
Quantity(4, m s⁻¹)
```

But watch out for Python's order of operations, which still applies as normal:
```python
>>> 8 * qu.J * qu.mol**-1 * qu.kg**-1   # Exponention takes priority over multiplication
Quantity(8, J mol⁻¹ kg⁻¹)
>>> 8 * qu.J / qu.mol * qu.kg           # Evaluated left to right as ((8 * J) / mol) * kg
Quantity(8, J kg mol⁻¹)
>>> 8 * qu.J / (qu.mol * qu.kg)         # Terms in parentheses evaluated first
Quantity(8, J mol⁻¹ kg⁻¹)
```

Numbers can be given as `int`, `float`, `str`, or `Decimal`, and the quantity will show the expected result and also retain the precision provided:
```python
>>> 4010 * qu.metre**3
Quantity(4010, m³)
>>> 4.01 * qu.volt
Quantity(4.01, V)
>>> "4.01e3" * qu.coulomb
Quantity(4.01E+3, C)
>>> from decimal import Decimal
>>> Decimal("0.401") * qu.newton * qu.metre
Quantity(0.401, N m)
>>> Decimal("40.1") * qu.newton * qu.metre
Quantity(40.1, N m)
```

Contrast the `float` result with the default behaviour of `Decimal`:
```python
>>> Decimal(4.01)
Decimal('4.0099999999999997868371792719699442386627197265625')
```

Though note that, unavoidably, trailing zeroes in a `float` will have been dropped before being passed to `Quantity`:
```python
>>> 741.60 * qu.g * qu.mol**-1
Quantity(741.6, g mol⁻¹)
>>> "741.60" * qu.g * qu.mol**-1
Quantity(741.60, g mol⁻¹)
```

If necessary, quantities can also be created using the `Quantity` class:
```python
>>> from quanstants import units as qu, Quantity
>>> density = Quantity(0.997, qu.kg/qu.L)
>>> print(density)
0.997 kg L⁻¹
```

### Units and prefixes

Multiply a prefix with a unit to create a prefixed unit:
```python
>>> from quanstants import units as qu, prefixes as qp
>>> 50 * (qp.micro * qu.metre)
Quantity(50, µm)
>>> 50 * (qp.µ * qu.m)
Quantity(50, µm)
```

Binary prefixes are also defined:
```python
>>> 256 * (qp.gibi * qu.byte)
Quantity(256, GiB)
>>> (256 * (qp.giga * qu.byte)) < (256 * (qp.gibi * qu.byte))
True
```

Various common prefixed units are pre-generated and available within `units`, but not under their symbol to avoid clashes within the namespace:
```python
>>> 50 * qu.micrometre
Quantity(50, µm)
>>> 50 * qu.micron
Quantity(50, µm)
>>> "99.7" * qu.megahertz               # qu.MHz raises AttributeError
Quantity(99.7, MHz)
```

A huge selection of units is available, not just SI or metric:
```python
>>> 45032.5 * qu.kilowatthour
Quantity(45032.5, kWh)
>>> 6 * qu.foot
Quantity(6, ft)
>>> 1.27 * qu.carat
Quantity(1.27, ct)
>>> 32 * qu.nautical_mile
Quantity(32, nmi)
>>> 32 * (qp.nano * qu.mile)            # Nothing to stop you doing this
Quantity(32, nmi)
>>> 32 * qu.nautical_mile == 32 * (qp.nano * qu.mile)
False
```

Currently, `quanstants` comes with a set of >31 pre-defined units, plus an additional 105 pre-defined prefixed units, accessible under >209 different names in the `quanstants.units` namespace.
This compares to ~950 defined names in a default `pint` `UnitRegistry()`.
`astropy.units` contains >3900 defined names, but most are just prefixed units, so the total number of unique unprefixed units is more like 70 to 80.

### Basic arithmetic
```python
>>> (4 * qu.metre) * (3 * qu.second**-1)
Quantity(12, m s⁻¹)
>>> (4 * qu.metre) + (50 * qu.centimetre)
Quantity(4.5, m)
>>> (4 * qu.metre) + (3 * qu.kilogram)
quanstants.quantity.MismatchedUnitsError: Can't add quantity in Unit(m) to quantity in Unit(kg).
>>> (3 * qu.watt)**2
Quantity(9, W²)
```
### Conversion
```python
>>> (50 * qu.joule).base()
Quantity(50, kg m² s⁻²)
>>> a = 1 * ((qp.kilo * qu.Watt) * qu.hour)
Quantity(1, kW h)
>>> a.to(qu.joule)
Quantity(3.600E+6, J)
>>> (6 * qu.foot).to(qu.metre)
Quantity(1.8288, m)
>>> speed = 3 * qu.m * qu.s**-1
>>> time = 15 * qu.s
>>> distance = speed * time
Quantity(45, m s⁻¹ s)
>>> distance.cancel()
Quantity(45, m)
```
### Constants
```python
>>> from quanstants import units as u, prefixes as p, constants as c
>>> qc.Planck_constant
Quantity(6.62607015E-34, J s)
>>> qc.h
Quantity(6.62607015E-34, J s)
>>> qc.Planck
Quantity(6.62607015E-34, J s)
```

Note the case-sensitivity: constants named after a person or other proper noun are capitalized, whereas units named after people are not (as per SI style):
```python
>>> qc.Bohr_radius
Quantity(5.29177210903E-11, m, uncertainty=8.0E-21)
>>> 16 * qu.tesla                       # Incidentally, the magnetic field required to levitate a frog
Quantity(16, T)
```

```python
>>> qc.proton_mass
Quantity(1.67262192369E-27, kg, uncertainty=5.1E-37)
>>> E = qc.proton_mass * qc.speed_of_light**2
Quantity(1.503277615985125705245525892E-10, kg m² s⁻²)
>>> E.to((qp.M * qu.eV)/qc.c**2)
Quantity(938.2720881604903652873556338, m² s⁻² MeV m⁻² s²)
>>> E.to((qp.M * qu.eV)/qc.c**2).cancel()
Quantity(938.2720881604903652873556338, MeV)
```
### Rounding
```python
>>> a = (324.9 * qu.J) * (1.674 * qu.mol**-1)
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

## Why Decimal?

For the general arguments, see the [documentation for `Decimal`](https://docs.python.org/3/library/decimal.html).

The usual argument *against* the use of `Decimal` is that the precision of `float` is high enough for most applications.
For calculations, this is certainly true.

For other purposes, however, it does not always suffice, such as in the preparation of documents or for checking more trivial calculations.

By way of example, the original author was motivated originally by their experiences using Python to prepare data for publication in a scientific manuscript.
In one situation, quantities in two columns were all measured to a precision of 3 significant figures, but this precision would not be carried through.
Measured values such as `1.260` would be inserted as "1.26".
Moreover, a third column containing the difference between the first two columns needed to be rounded to two decimal places to reflect the lower precision -- a trivial sum to do by hand, but time-consuming for thousands of data points.


First and foremost, it easily allows significance and precision to be preserved.

Second, the lack of exactness of floats, i.e. that `0.1 + 0.1 + 0.1 - 0.3 != 0` and `0.1 + 0.2 != 0.3`.

Third, decimal numbers are what the user believes them to be.
An inexperienced user thinks that the float `8.7` is 8.7, because that is what is printed by Python, when in reality it is stored as `8.699999999999999289457264239899814128875732421875`.

Fourth, the ease of implementation of the desired rounding behaviours.
Significant figures are a challenge without the numerical type having a concept of significance.

Fifth, the rounding behaviour is *correct*.
Even if rounding is configured so that half rounds up, `round(0.35, 1)` will still always give `0.3`, because it is *actually* `0.34999999999999997779553950749686919152736663818359375`, this is just not obvious to the user.

Experienced programmers are likely aware of the pitfalls of working with binary mathematics and can identify when binary floating point suffices and when a switch to decimal is necessary for precision.
For inexperienced programmers, the behaviour of binary floating point is often perplexing.
This package aims to alleviate this confusion.



Using `Quantity` objects in complex or extensive calculations, as opposed to binary floats or even decimal floats, will of course have an associated cost.
`quanstants` is not intended for this usage.

## Standards and naming

The primary source of authority is the SI brochure (9th Edition v2.01 in English), followed by ISO/IEC 80000.

Following the SI/ISO example means the names of units are primarily those defined there e.g. metre, litre.
As far as possible, however, units are also made available under alternative names, particular those common in American English.
For consistency, names of constants are written first and foremost in British/Irish/European English, but should also be available under the American English equivalents.

## Comparison to astropy and pint

Both `astropy` (via its modules `constants` and `units`) and `pint` are excellent packages, but both had limitations that inspired the creation of `quanstants`.
Moreover, while they and various other packages each do some things very well and have some advanced functionality, all met only a subset of my (fairly basic) needs, so `quanstants` aims to take the best of each.

Most crucially, neither package makes the same opinionated decisions about number handling and rounding as `quanstants`: they do not store numbers as `Decimal` by default, or implement the more traditional rounding mode, or allow rounding to significant figures.

In comparison to `astropy`, the unit and constant selection is far broader than simply those useful for astronomers, and there is no reliance on `numpy`.

In comparison to `pint`, quantities are immutable by design -- any function or operation that would change a `Quantity` returns a new `Quantity`, and likewise for `Unit`. `quanstants` also contains significantly more constant values.

## TODO

* Complex quantities
* Uncertainty calculations
* Discard uncertainties on rounding
