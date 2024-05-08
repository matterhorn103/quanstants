# quanstants

A small package to make life easier for scientists who need physical quantities, units, and constants.

`quanstants` strives to make doing scientific calculations in Python as intuitive as doing them on paper.
It is strongly inspired by the LaTeX package `siunitx`, and of course also `astropy` and `pint`.

What makes `quanstants` different?
An emphasis on the [principle of least astonishment](https://en.wikipedia.org/wiki/Principle_of_least_astonishment).
Importantly, applied from the perspective of a non-programmer in the natural sciences.

`quanstants` is particularly suited to situations where it is desirable to have the result match exactly that which you would produce yourself, and less suited to huge numbers of complex calculations.
For example, the implicit sanity checking that comes with tracking units helps to ensure calculations are being done correctly, and it works very well for programmatically producing nicely formatted text or LaTeX code for presentation of calculation results in e.g. a scientific publication.

`quanstants` makes life easy by providing the following:

* The `Quantity` object, which describes a physical quantity as a product of a number and a unit, per the SI definition, with an associated uncertainty
* Arithmetic with quantities that is intelligent and unit-aware
  * Tracking units is an extremely useful sanity check to avoid mistakes
  * Impossible mathematical operations are flagged and prevented
  * Addition/subtraction of quantities with the same dimensions but different units works, via conversion
* A large library of units including SI, metric, imperial, and non-standard units
  * Easy creation of quantities through arithmetic between numbers and units
  * Easy unit conversion/expression of quantities in alternative units
  * Robust parsing of string input for quantity creation
  * Only a small core selection is loaded by default, but further unit modules are very easy to load (see examples below)
  * Support for temperatures on non-absolute scales such as Celsius and Fahrenheit, and sensible, unsurprising behaviour of those temperatures in calculations
* A large library of constants covering various fields, not just physics
  * Easy definition of custom constants
* Default behaviours for numbers that align with non-programmers' expectations, rather than Python defaults
  * `5.2 * m` gives _exactly_ `5.2 m`, _not_ `5.2000000000000001776356... m`
  * `0.1 m + 0.2 m` is _exactly_ equal to `0.3 m` (whereas in Python, `0.1 + 0.2 == 0.3` returns `False`)
  * Significant figures are maintained: `1.30 m` does _not_ become `1.3 m`, and `0.15 m - 0.05 m` ⇒ `0.10 m`, _not_ `0.1 m`
  * Rounding gives the same result as most scientists would write if rounding manually: `1.75 m` ⇒ `1.8 m`, `1.65 m` ⇒ `1.7 m`, and `-1.65 m` ⇒ `-1.7 m` ("round half away from zero", whereas the Python default is "round half to even")
  * Rounding to a certain number of significant figures is easy
* Use of the `Decimal` type under the hood to avoid the [quirks of binary floating point](https://docs.python.org/3/tutorial/floatingpoint.html#tut-fp-issues), which as well as enabling the above, avoids imprecision arising from rounding errors
  * Most mathematical operations and functions are delegated to [Python's `Decimal`](https://docs.python.org/3/library/decimal.html), which in turn follows the General Decimal Arithmetic Specification and thus indirectly IEEE 754. `quanstants` simply tracks units and uncertainties on top, and aims to support all operations on `Quantity` that are supported by `Decimal`
* Customization of various options including the rounding behaviour, automatic cancellation of units, and many elements of printing formatting
  * Configuration via a `.toml` file -- autodetected in the current directory, but can also be manually loaded

## Usage

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
>>> qu.m is qu.metre  # Both refer to same object
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
>>> 8 * qu.J * qu.mol**-1 * qu.kg**-1  # Exponention takes priority over multiplication
Quantity(8, J mol⁻¹ kg⁻¹)
>>> 8 * qu.J / qu.mol * qu.kg  # Evaluated left to right as ((8 * J) / mol) * kg
Quantity(8, J kg mol⁻¹)
>>> 8 * qu.J / (qu.mol * qu.kg)  # Terms in parentheses evaluated first
Quantity(8, J mol⁻¹ kg⁻¹)
```

Numbers can be given as `int`, `float`, `str`, or `Decimal`, and the quantity will show the expected result and also retain the precision provided:
```python
>>> 4010 * qu.metre**3
Quantity(4010, m³)
>>> 4.01 * qu.volt
Quantity(4.01, V)
>>> "4010" * qu.coulomb  # Python's Decimal assumes all figures are significant
Quantity(4010, C)
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

Various common prefixed units are pre-generated and available within `quanstants.units`, but not under their symbol to avoid clashes within the namespace:
```python
>>> 50 * qu.micrometre
Quantity(50, µm)
>>> 50 * qu.micron
Quantity(50, µm)
>>> "99.7" * qu.megahertz  # qu.MHz raises AttributeError
Quantity(99.7, MHz)
```

A huge selection of units is available, including metric units not approved by the SI:
```python
>>> 45032.5 * qu.kilowatthour
Quantity(45032.5, kWh)
>>> 1.27 * qu.carat
Quantity(1.27, ct)
```

Other systems and categories of units are easily accessed by importing the appropriate module, which adds the units within to `quanstants.units`:
```python
>>> from quanstants.units import imperial
>>> 6 * qu.foot
Quantity(6, ft)
>>> from quanstants.units import us
>>> 20 * qu.us_fluid_ounce
Quantity(20, fl oz)
>>> 32 * qu.nautical_mile
Quantity(32, nmi)
>>> 32 * (qp.nano * qu.mile)  # Nothing to stop you doing this
Quantity(32, nmi)
>>> 32 * qu.nautical_mile == 32 * (qp.nano * qu.mile)
False
```

Units are often made available in multiple modules if shared between systems, so you only need to worry about importing the unit system of interest:
```python
>>> from quanstants import imperial, us
>>> imperial.foot is us.foot
True
>>> imperial.foot is us.us_survey_foot
False
```

Currently, `quanstants` comes by default with a set of >51 pre-defined units, accessible under >113 different names in the `quanstants.units` namespace class.
An additional 87 prefixed units are also pre-defined.
With all unit submodules imported, >117 (unprefixed) units are defined under >212 different names.

This compares to ~950 defined names in a default `pint` `UnitRegistry()`.
`astropy.units` contains >3900 defined names, but most are just prefixed units, so the total number of unique unprefixed units is more like 70 to 80.

### Uncertainties

The numerical value of a quantity can be most conveniently given an associated uncertainty using the provided method:
```python
>>> from quanstants import units as qu
>>> gravity = ("6.67430e-11" * qu.newton * qu.metre**2 * qu.kilogram**-2).with_uncertainty("0.00015e-11")
Quantity(6.67430E-11, N m² kg⁻², uncertainty=1.5E-15)
```
The uncertainty can be given as a number with the same units as the quantity, or as a `Quantity` itself:
```python
>>> ("4.2" * qu.m).plusminus("0.2")  # Alias for with_uncertainty()
Quantity(4.2, m, uncertainty=0.2)
>>> ("4.2" * qu.metre).plusminus("20" * qu.centimetre)
Quantity(4.2, m, uncertainty=0.20)
```
When printed, uncertainties are shown by parentheses if possible, or with `±` as a fallback:
```
>>> print(("6.67430e-11" * qu.newton * qu.metre**2 * qu.kilogram**-2).with_uncertainty("0.00015e-11"))
6.67430(15)E-11 N m² kg⁻²
>>> print(("4.2" * qu.metre).plusminus("20" * qu.centimetre))  # Precisions don't match
4.2 ± 0.20 m
```
The use of ± at all times can be forced by setting `UNCERTAINTY_STYLE` (see Config below).

The uncertainty can also be passed to `Quantity()` directly:
```python
>>> from quanstants import units as qu, Quantity
>>> density = Quantity(0.99704702, qu.kg/qu.L, uncertainty=0.00000083)
>>> print(density)
0.99704702(83) kg L⁻¹
```

A request for the uncertainty of a quantity returns it as another `Quantity`:
```python
>>> density.uncertainty
Quantity(8.3E-7, kg L⁻¹)
```

### Parsing strings

Though not generally the preferred option for quantity creation, quantities can also be created by passing a single string as an argument to the `Quantity` class with the number and unit separated by a space:

```python
>>> Quantity("4 m")
Quantity(4, m)
>>> Quantity("4 metre")
Quantity(4, m)
>>> Quantity("741.60 g mol-1")
Quantity(741.60, g mol⁻¹)
>>> Quantity("0.997e3 g/L")
Quantity(997, g L⁻¹)
```

Uncertainties can also be specified in the string, in either of two usual forms:
```python
>>> Quantity("6.67430(15)E-11 N m² kg⁻²")
Quantity(6.67430E-11, N m² kg⁻², uncertainty=1.5E-15)
>>> Quantity("8.293 ± 0.010 V")
Quantity(8.293, V, uncertainty=0.010)
>>> Quantity("8.293 +/- 0.010 V")  # Also acceptable
Quantity(8.293, V, uncertainty=0.010)
```

### Arithmetic

Mathematical operations are in general performed with the quantity considered to be the product of its number and unit.
The implementation of arithmetic on the numerical part(s) is typically that of the `Decimal` type.

Quantities can be multiplied and divided by other quantities or by numbers, and the results will possess the correct number to the correct precision, and the correct units:
```python
>>> (4 * qu.metre * qu.second**-1) * (6 * qu.second)
Quantity(24, m)
>>> ("3.20" * qu.W) * "16.90"
Quantity(54.0800, W)
>>> (200 * qu.megajoule) / (70 * qu.kilogram)
Quantity(2.857142857142857142857142857, MJ kg⁻¹)
```

Raising to the power of a number is also supported for integer and fractional powers:
```python
>>> (3 * qu.watt)**2
Quantity(9, W²)
>>> from fractions import Fraction
>>> (20 * qu.metre**2)**Fraction(1, 2)
Quantity(4.472135954999579392818347337, m)
```

Addition and subtraction is possible when it makes physical sense i.e. when the units of the two quantities are the same or have the same dimension:
```python
>>> (4 * qu.metre) + (0.5 * qu.metre)
Quantity(4.5, m)
>>> (4 * qu.metre) - (50 * qu.centimetre)
Quantity(3.50, m)
>>> from quanstants.units import us
>>> (4 * qu.metre) + (2 * qu.foot)
Quantity(4.6096, m)
```

If the units do not match, a `MismatchedUnitsError` will be raised, which serves as a useful sanity check:
```python
>>> (4 * qu.metre) + (3 * qu.kilogram)
quanstants.quantity.MismatchedUnitsError: Can't add quantity in Unit(m) to quantity in Unit(kg).
```

Similarly, (in)equalities are implemented between quantities of the same dimension:
```python
>> (0.3 * qu.litre) > (150 * qu.millilitre)
True
>>> (0.15 * qu.litre) >= (150 * qu.millilitre)
True
>>> (0.3 * qu.litre) > (150 * qu.centimetre**3)
True
```

Quantities can be used as exponents themselves, and have the same `sqrt()`, `exp()`, `ln()`, and `log10()` functions implemented as `Decimal` does:
```python
>>> a = 100 * qu.m
>>> b = 25 * qu.m
>>> a/b
Quantity(4, (unitless))
>>> 2**(a/b)
Quantity(16, (unitless))
>>> (a/b).sqrt()
Quantity(2.000000000000000000000000000, (unitless))
>>> (a/b).exp()
Quantity(54.59815003314423907811026120, (unitless))
>>> (a/b).ln()
Quantity(1.386294361119890618834464243, (unitless))
>>> (a/b).log10()
Quantity(0.6020599913279623904274777894, (unitless))
>>> a.log10()
quanstants.quantity.NotDimensionlessError: Cannot take the logarithm of a non-dimensionless quantity!
```

You can easily check whether a quantity is dimensionless:
```python
>>> (a/b).is_dimensionless()
True
```

Uncertainties are kept track of correctly across arithmetic operations using uncertainty propagation rules:
```python
>>> a = (3 * qu.m).plusminus(0.1)
>>> b = (2 * qu.m).plusminus(0.2)
>>> a + b
Quantity(5, m, uncertainty=0.22360679774997896)
>>> a - b
Quantity(1, m, uncertainty=0.22360679774997896)
>>> a * b
Quantity(6, m², uncertainty=0.6324555320336759)
>>> a / b
Quantity(1.5, (unitless), uncertainty=0.15811388300841897)
>>> 2**(a/b)
Quantity(2.828427124746190097603377448, (unitless), uncertainty=0.3099848428288717)
```

By default, the correlation between two quantities is assumed to be zero.
Uncertainties in the results of operations on two correlated quantities can also be obtained, however, by accessing the corresponding dunder method of the quantity directly and giving the correlation:
```python
>>> a.__add__(b, correlation=0.7)
Quantity(5, m, uncertainty=0.2792848008753788)
>>> a.__sub__(b, correlation=1)
Quantity(1, m, uncertainty=0.1)
```

### Conversion

Expressing a quantity in other units is done easily:
```python
>>> (2 * qu.metre).to(qu.millimetre)
Quantity(2E+3, mm)
>>> (6 * qu.foot).to(qu.metre)
Quantity(1.8288, m)
>>> (6 * qu.hour).to(qu.s)
Quantity(21600, s)
>>> from quanstants import prefixes as qp
>>> ((3 * qp.kilo * qu.watt) * (1 * qu.day)).to(qu.joule)
Quantity(2.59200E+8, J)
```

Use the `base()` function to express a quantity in terms of SI base units:
```python
>>> (50 * qu.joule).base()
Quantity(50, kg m² s⁻²)
```

Use the `cancel()` function to combine like terms in the unit after a calculation:
```python
>>> speed = 3 * qu.m * qu.s**-1
>>> time = 15 * qu.s
>>> distance = 45 * (qu.m * qu.s * qu.s**-1)
Quantity(45, m s s⁻¹)
>>> distance.cancel()
Quantity(45, m)
```
Note that by default `AUTO_CANCEL` is set to `True` (see Config below), in which case cancelling happens automatically after arithmetic operations:
```python
>>> speed * time
Quantity(45, m)
```

Use `fully_cancel()` to additionally combine units with the same dimensionality:
```python
>>> x = (30 * qu.kilowatt * qu.s) / (200 * qu.s * qu.watt**-1)
>>> x.cancel()
Quantity(0.15, kW W)
>>> x.fully_cancel()
Quantity(0.00015, kW²)
>>> length = (3000 * qu.metre**2) / (20 * qu.foot)
>>> length.cancel()
Quantity(150, m² ft⁻¹)
>>> length.fully_cancel()
Quantity(492.1259842519685039370078740, m)
```

The presentation of compound units is dependent on the order of mathematical operations.
While this doesn't affect (in)equalities, it may sometimes be useful to get a canonical, reproducible representation of a quantity and its units, for which the `canonical()` method is available:
```python
>>> mass = 20 * qu.kilogram
>>> acceleration = 3 * qu.metre * qu.second**-2
>>> mass * acceleration
Quantity(60, kg m s⁻²)
>>> acceleration * mass
Quantity(60, m kg s⁻²)
>>> (mass * acceleration).canonical()
Quantity(60, s⁻² m kg)
>>> (acceleration * mass).canonical()
Quantity(60, s⁻² m kg)
```

### Equalities

Both units and quantities are (to a large extent) immutable -- once created, their properties are fixed.
A quantity cannot be changed in-place, and operations on one will return a new `Quantity` object.
This means units, quantities, and their subclasses are hashable and can be used e.g. as keys in a `dict`.

Quantities with equivalent values will compare equal:
```python
>>> 3 * qu.kilometre == 3000 * qu.metre
True
```

This is the case even if the quantities have uncertainties, even if the uncertainties are different sizes:
```python
>>> 3 * qu.kilometre == (3000 * qu.metre).plusminus(20)
True
```

Units are compared based on their value, and are equal to quantities with the same value:
```python
>>> qu.watt == qu.joule / qu.second
True
>>> qu.watt == 1 * qu.J * qu.s**-1
True
```

A quantity with number 0 is still a valid quantity (and it might have an uncertainty!), but quantities with numbers equal to zero are considered themselves equal to 0 (and therefore equal to each other):
```python
>>> 0 * qu.m
Quantity(0, m)
>>> 0 * qu.m == 0
True
>>> 0 * qu.m == 0 * qu.s
True
```

Unitless quantities are considered to be equal to their numerical value, and the special unitless unit is considered equal to 1:
```python
>>> 2 * qu.unitless
Quantity(2, (unitless)
>>> 2 * qu.unitless == 2
True
>>> qu.unitless == 1
True
```

### Constants

In a similar fashion to units and prefixes, physical constants are available in the constants registry under various names and symbols:
```python
>>> from quanstants import units as qu, prefixes as qp, constants as qc
>>> qc.Planck_constant
Constant(Planck_constant = 6.62607015E-34 J s)
>>> qc.h
Constant(Planck_constant = 6.62607015E-34 J s)
>>> qc.Planck  # Anything ending with "constant" is available under the truncated form too
Constant(Planck_constant = 6.62607015E-34 J s)
```

Only a small selection is imported initially, but further sets of constants can be added to the constants registry in a way analogous to units:
```python
>>> from quanstants.constants import codata2018
>>> qc.vacuum_electric_permittivity
Constant(vacuum_electric_permittivity = 8.8541878128(13)E-12 F m⁻¹)
```

Note the case-sensitivity: constants named after a person or other proper noun are capitalized, whereas units named after people are not (as per SI style):
```python
>>> qc.Bohr_radius
Constant(Bohr_radius = 5.29177210903(80)E-11 m)
>>> 16 * qu.tesla  # Incidentally, the magnetic field required to levitate a frog
Quantity(16, T)
```

Mostly, constants behave like quantities:
```python
>>> qc.proton_mass
Constant(proton_mass = 1.67262192369(51)E-27 kg)
>>> E = qc.proton_mass * qc.speed_of_light**2
Quantity(1.503277615985125705245525892E-10, kg m² s⁻², uncertainty=4.58365141155777E-20)
```

Constants can be used as units by calling the `as_unit()` method, which creates a unit with the same value:
```python
>>> qc.proton_mass.to((qp.M * qu.eV)/qc.c.as_unit()**2)
Quantity(938.2720881604903652873556334, MeV c⁻², uncertainty=2.860890187940270488303725942E-7)
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

### Temperatures

Absolute temperatures in kelvin are normal quantities like any other:
```python
>>> T = 200 * qu.kelvin
Quantity(200, K)
>>> 2 * T
Quantity(400, K)
```

```python
>>> 0 @ qu.celsius
Temperature(0, °C)
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


Using `Quantity` objects in complex or extensive calculations, as opposed to binary floats or even decimal floats, will of course have a not-unsubstantial associated overhead.
`quanstants` is not intended for this usage; though, after all, neither is Python, and in many cases other advantages win out over pure speed and computational efficiency.

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

In comparison to `pint`, quantities are immutable by design -- any function or operation that would change a `Quantity` returns a new `Quantity`, and both units and quantities are hashable. `quanstants` also contains significantly more constant values.

## TODO

* Support for complex numbers
* Support for logarithmic units
* Adjustment of prefixes (either automatically or on request) so that e.g. 2000 kJ becomes 2 MJ
* Complete (>95%) test coverage
* Documentation beyond this readme
