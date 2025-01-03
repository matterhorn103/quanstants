"""These tests test the examples used in the README to check they work as advertised."""

import decimal
from decimal import Decimal as dec
from fractions import Fraction as frac

import pytest

from quanstants import (
    units as qu,
    prefixes as qp,
    constants as qc,
    Quantity,
    quanfig,
)

class TestREADME:
    """Here we parse the README and test each example statement automatically.
    
    A single method here tests all examples in a subsection of "Usage", with multiple
    assert statements per test.
    The results are thus not very helpful in the event of a failure, so it is still
    necessary to add manual tests below for diagnostics.
    However, for the common case that an example is added and a corresponding test
    is forgotten, it will still be tested by this automatic test.
    In this way, this test class provides a guarantee 
    """
    with open("README.md", "r", encoding="utf-8") as f:
        readme = [line.rstrip() for line in f.readlines()]
    subsections = {}
    current_subsection = None
    parsing_usage = False
    for line in readme:
        if line[:3] == "## ":
            if line == "## Usage":
                parsing_usage = True
            else:
                parsing_usage = False
        elif line[:4] == "### ":
            current_subsection = line[4:]
            subsections[current_subsection] = list()
        elif parsing_usage and current_subsection is not None:
            subsections[current_subsection].append(line)
    
    def run_subsection(self, subsection):
        # Go through markdown, ignore lines outside of code blocks
        # Execute lines in code blocks if they are input statements
        # If a line is an output statement, check that it is the result of
        # executing the previous line
        in_code_block = False
        last_executed = ""
        for n, line in enumerate(subsection):
            if line == "```python":
                in_code_block = True
            elif line == "```":
                in_code_block = False
            elif in_code_block:
                if "Error" in line.split()[0]:
                    continue
                elif "Error" in subsection[n+1].split()[0]:
                    continue
                elif line[:4] == ">>> ":
                    print(f"Executing: {line}")
                    last_executed = line[4:]
                    exec(last_executed)
                else:
                    print(f"Supposed result: {line}")
                    if last_executed.startswith("print("):
                        print("(Swapped `print()` for `str()`)")
                        last_executed = last_executed.replace("print(", "str(")
                        assert eval(last_executed) == line
                    else:
                        assert repr(eval(last_executed)) == line
            else:
                continue
    
    def test_quantity_creation(self):
        self.run_subsection(self.subsections["Quantity creation"])
    
    def test_units_and_prefixes(self):
        self.run_subsection(self.subsections["Units and prefixes"])
    
    def test_uncertainties(self):
        self.run_subsection(self.subsections["Uncertainties"])
    
    def test_parsing(self):
        self.run_subsection(self.subsections["Parsing strings"])
    
    def test_arithmetic(self):
        self.run_subsection(self.subsections["Arithmetic"])
    
    def test_conversion(self):
        self.run_subsection(self.subsections["Conversion"])
    
    def test_equalities(self):
        self.run_subsection(self.subsections["Equalities"])
    
    def test_constants(self):
        self.run_subsection(self.subsections["Constants"])
    
    def test_rounding(self):
        self.run_subsection(self.subsections["Rounding"])
        # Reset decimal rounding behaviour
        decimal.getcontext().rounding = decimal.ROUND_HALF_EVEN
    
    def test_temperatures(self):
        self.run_subsection(self.subsections["Temperatures"])
    
    def test_logs(self):
        self.run_subsection(self.subsections["Logarithmic scales"])
    
    # Don't test the config section as the tomls make it complicated


class TestQuantityCreation:
    def test_multiplication(self):
        q = 4 * qu.metre
        assert str(q) == "4 m"

    def test_symbol(self):
        q = 4 * qu.m
        assert str(q) == "4 m"

    def test_symbol_equivalence(self):
        assert qu.m is qu.metre

    def test_alt_spelling(self):
        q = 4 * qu.meter
        assert str(q) == "4 m"

    def test_alt_spelling_equivalence(self):
        assert qu.meter is qu.metre

    def test_unit_squared(self):
        q = 4 * qu.metre**2
        assert str(q) == "4 m²"

    def test_unit_negative_exponent(self):
        q = 4 * qu.joule * qu.kilogram**-1
        assert str(q) == "4 J kg⁻¹"

    def test_division(self):
        q = 4 * qu.metre / qu.second
        assert str(q) == "4 m s⁻¹"

    def test_int(self):
        q = 4010 * qu.metre**3
        assert str(q) == "4010 m³"

    def test_float(self):
        q = 4.01 * qu.volt
        assert str(q) == "4.01 V"

    def test_float_with_power_ten(self):
        q = "4.01e3" * qu.coulomb
        assert str(q) == "4.01E+3 C"

    def test_decimal(self):
        q = dec("0.401") * qu.newton * qu.metre
        assert str(q) == "0.401 N m"

    def test_precision_retention_with_str(self):
        q = "741.60" * qu.g * qu.mol**-1
        assert str(q) == "741.60 g mol⁻¹"

    def test_quantity_instantiation(self):
        q = Quantity(0.997, qu.kg / qu.L)
        assert str(q) == "0.997 kg L⁻¹"

    def test_quantity_creation_method_equivalence(self):
        q1 = Quantity(0.997, qu.kg / qu.L)
        q2 = 0.997 * (qu.kg / qu.L)
        assert q1 == q2


class TestUnitsAndPrefixes:
    def test_prefixes(self):
        q = 50 * (qp.micro * qu.metre)
        assert str(q) == "50 μm"
    
    def test_prefix_symbols(self):
        q = 50 * (qp.μ * qu.m)
        assert str(q) == "50 μm"

    def test_binary_symbols(self):
        assert (256 * (qp.giga * qu.byte)) < (256 * (qp.gibi * qu.byte))
    
    def test_prefixed_units(self):
        # Just check that all those advertised are available
        q = 50 * qu.micrometre
        q = 50 * qu.micron
        q = "99.7" * qu.megahertz
    
    def test_non_si_metric_units(self):
        # Just check that all those advertised are available
        q = 45032.5 * qu.kilowatthour
        q = 1.27 * qu.carat

    def test_imperial_us(self):
        # Just check that all those advertised are available
        from quanstants.units import imperial, us
        q = 6 * qu.foot
        q = 20 * qu.us_fluid_ounce
        q = 32 * qu.nautical_mile
        q = 32 * (qp.nano * qu.mile)
        assert 32 * qu.nautical_mile != 32 * (qp.nano * qu.mile)
    
    def test_unit_shared_between_modules(self):
        from quanstants.units import imperial, us
        assert imperial.foot is us.foot
    

class TestUncertainties:
    def test_with_uncertainty(self):
        gravity = ("6.67430e-11" * qu.newton * qu.metre**2 * qu.kilogram**-2).with_uncertainty("0.00015e-11")
        assert repr(gravity) == "Quantity(6.67430E-11, N m² kg⁻², uncertainty=1.5E-15)"

    def test_plus_minus(self):
        assert repr(("4.2" * qu.m).plus_minus("0.2")) == "Quantity(4.2, m, uncertainty=0.2)"
    
    def test_uncertainty_as_quantity(self):
        assert repr(("4.2" * qu.metre).plus_minus("20" * qu.centimetre)) == "Quantity(4.2, m, uncertainty=0.20)"

    def test_str_parentheses(self):
        assert str(("6.67430e-11" * qu.newton * qu.metre**2 * qu.kilogram**-2).with_uncertainty("0.00015e-11")) == "6.67430(15)E-11 N m² kg⁻²"

    def test_str_plus_minus(self):
        assert str(("4.2" * qu.metre).plus_minus("20" * qu.centimetre)) == "4.2 ± 0.20 m"

    def test_set_uncertainty_style(self):
        quanfig.UNCERTAINTY_STYLE = "PLUSMINUS"
        assert str(("6.67430e-11" * qu.newton * qu.metre**2 * qu.kilogram**-2).with_uncertainty("0.00015e-11")) == "6.67430E-11 ± 1.5E-15 N m² kg⁻²"
        quanfig.UNCERTAINTY_STYLE = "PARENTHESES"

    def test_uncertainty_at_quantity_creation(self):
        density = Quantity(0.99704702, qu.kg/qu.L, uncertainty=0.00000083)
        assert str(density) == "0.99704702(83) kg L⁻¹"
    
    def test_get_uncertainty(self):
        density = Quantity(0.99704702, qu.kg/qu.L, uncertainty=0.00000083)
        assert repr(density.uncertainty) == "Quantity(8.3E-7, kg L⁻¹)"
    

class TestParsing:
    def test_unit_symbol(self):
        assert repr(Quantity("4 m")) == "Quantity(4, m)"
    
    def test_unit_name(self):
        assert repr(Quantity("4 metre")) == "Quantity(4, m)"
    
    def test_reciprocal(self):
        assert repr(Quantity("741.60 g mol-1")) == "Quantity(741.60, g mol⁻¹)"
    
    def test_slash(self):
        assert repr(Quantity("0.997e3 g/L")) == "Quantity(997, g L⁻¹)"

    def test_uncertainty_parentheses(self):
        assert repr(Quantity("6.67430(15)E-11 N m² kg⁻²")) == "Quantity(6.67430E-11, N m² kg⁻², uncertainty=1.5E-15)"
        
    def test_uncertainty_plus_minus(self):
        assert repr(Quantity("8.293 ± 0.010 V")) == "Quantity(8.293, V, uncertainty=0.010)"
    
    def test_uncertainty_plusslashminus(self):
        assert repr(Quantity("8.293 +/- 0.010 V")) == "Quantity(8.293, V, uncertainty=0.010)"


class TestArithmetic:
    def test_1(self):
        result = repr((4 * qu.metre * qu.second**-1) * (6 * qu.second))
        assert result == "Quantity(24, m)"

    def test_2(self):
        result = repr(("3.20" * qu.W) * "16.90")
        assert result == "Quantity(54.0800, W)"

    def test_3(self):
        result = repr((200 * qu.megajoule) / (70 * qu.kilogram))
        assert result == "Quantity(2.857142857142857142857142857, MJ kg⁻¹)"

    def test_4(self):
        result = repr((3 * qu.watt)**2)
        assert result == "Quantity(9, W²)"

    def test_5(self):
        result = repr((20 * qu.metre**2)**frac(1, 2))
        assert result == "Quantity(4.472135954999579392818347337, m)"

    def test_6(self):
        result = repr((4 * qu.metre) + (0.5 * qu.metre))
        assert result == "Quantity(4.5, m)"

    def test_7(self):
        result = repr((4 * qu.metre) - (50 * qu.centimetre))
        assert result == "Quantity(3.50, m)"

    def test_8(self):
        result = repr((4 * qu.metre) + (2 * qu.foot))
        assert result == "Quantity(4.6096, m)"

    def test_9(self):
        from quanstants.quantity import MismatchedUnitsError
        with pytest.raises(MismatchedUnitsError):
            result = repr((4 * qu.metre) + (3 * qu.kilogram))

    def test_10(self):
        assert (0.3 * qu.litre) > (150 * qu.millilitre)

    def test_11(self):
        assert (0.15 * qu.litre) >= (150 * qu.millilitre)

    def test_12(self):
        assert (0.3 * qu.litre) > (150 * qu.centimetre**3)

    def test_13(self):
        a = 100 * qu.m
        b = 25 * qu.m
        result = repr((a/b))
        assert result == "Quantity(4, (unitless))"

    def test_14(self):
        a = 100 * qu.m
        b = 25 * qu.m
        result = repr(2**(a / b))
        assert result == "Quantity(16, (unitless))"

    def test_15(self):
        a = 100 * qu.m
        b = 25 * qu.m
        result = repr((a / b).sqrt())
        assert result == "Quantity(2, (unitless))"

    def test_16(self):
        a = 100 * qu.m
        b = 25 * qu.m
        result = repr((a / b).exp())
        assert result == "Quantity(54.59815003314423907811026120, (unitless))"

    def test_17(self):
        a = 100 * qu.m
        b = 25 * qu.m
        result = repr((a / b).ln())
        assert result == "Quantity(1.386294361119890618834464243, (unitless))"

    def test_18(self):
        a = 100 * qu.m
        b = 25 * qu.m
        result = repr((a / b).log10())
        assert result == "Quantity(0.6020599913279623904274777894, (unitless))"

    def test_20(self):
        a = 100 * qu.m
        b = 25 * qu.m
        assert (a / b).is_dimensionless()

    def test_21(self):
        a = (3 * qu.m).plus_minus(0.1)
        b = (2 * qu.m).plus_minus(0.2)
        result = repr(a + b)
        assert result == "Quantity(5, m, uncertainty=0.2236067977499789696409173669)"

    def test_22(self):
        a = (3 * qu.m).plus_minus(0.1)
        b = (2 * qu.m).plus_minus(0.2)
        result = repr(a - b)
        assert result == "Quantity(1, m, uncertainty=0.2236067977499789696409173669)"

    def test_23(self):
        a = (3 * qu.m).plus_minus(0.1)
        b = (2 * qu.m).plus_minus(0.2)
        result = repr(a * b)
        assert result == "Quantity(6, m², uncertainty=0.6324555320336758663997787090)"

    def test_24(self):
        a = (3 * qu.m).plus_minus(0.1)
        b = (2 * qu.m).plus_minus(0.2)
        result = repr(a / b)
        assert result == "Quantity(1.5, (unitless), uncertainty=0.1581138830084189665999446772)"

    def test_25(self):
        a = (3 * qu.m).plus_minus(0.1)
        b = (2 * qu.m).plus_minus(0.2)
        result = repr(2**(a/b))
        assert result == "Quantity(2.828427124746190097603377448, (unitless), uncertainty=0.3099848428288716908396318060)"

    def test_26(self):
        a = (3 * qu.m).plus_minus(0.1)
        b = (2 * qu.m).plus_minus(0.2)
        result = repr(a.__add__(b, correlation=0.7))
        assert result == "Quantity(5, m, uncertainty=0.2792848008753788233976784908)"

    def test_27(self):
        a = (3 * qu.m).plus_minus(0.1)
        b = (2 * qu.m).plus_minus(0.2)
        result = repr(a.__sub__(b, correlation=1))
        assert result == "Quantity(1, m, uncertainty=0.1)"


class TestConversion:
    def test_to_millimeter(self):
        result = (2 * qu.metre).to(qu.millimetre)
        assert repr(result) == "Quantity(2E+3, mm)"

    def test_to_metre(self):
        from quanstants.units import imperial
        result = (6 * qu.foot).to(qu.metre)
        assert repr(result) == "Quantity(1.8288, m)"

    def test_to_second(self):
        result = (6 * qu.hour).to(qu.s)
        assert repr(result) == "Quantity(21600, s)"

    def test_to_joule(self):
        result = ((3 * (qp.kilo * qu.watt)) * (1 * qu.day)).to(qu.joule)
        assert repr(result) == "Quantity(2.59200E+8, J)"

    def test_base(self):
        result = (50 * qu.joule).base()
        assert repr(result) == "Quantity(50, m² kg s⁻²)"

    def test_cancel(self):
        result = (45 * (qu.m * qu.s * qu.s**-1)).cancel()
        assert repr(result) == "Quantity(45, m)"
    
    def test_fully_cancel(self):
        result = ((30 * qu.kilowatt * qu.s) / (200 * qu.s * qu.watt**-1)).fully_cancel()
        assert repr(result) == "Quantity(0.00015, kW²)"

    def test_fully_cancel_2(self):
        result = ((3000 * qu.metre**2) / (20 * qu.foot)).fully_cancel()
        assert repr(result) == "Quantity(492.1259842519685039370078740, m)"

    def test_not_canonical(self):
        mass = 20 * qu.kilogram
        acceleration = 3 * qu.metre * qu.second**-2
        assert repr(mass * acceleration) != repr(acceleration * mass)

    def test_canonical(self):
        mass = 20 * qu.kilogram
        acceleration = 3 * qu.metre * qu.second**-2
        assert repr((mass * acceleration).canonical()) == repr((acceleration * mass).canonical())


class TestEqualities:
    def test_mixed_units(self):
        assert 3 * qu.kilometre == 3000 * qu.metre

    def test_uncertainties(self):
        assert 3 * qu.kilometre == (3000 * qu.metre).plus_minus(20)

    def test_units(self):
        assert qu.watt == qu.joule / qu.second
    
    def test_unit_quantity_comparison(self):
        assert qu.watt == 1 * qu.joule * qu.s**-1

    def test_equal_to_zero(self):
        assert 0 * qu.m == 0
    
    def test_two_zeroes(self):
        assert 0 * qu.m == 0 * qu.s

    def test_unitless_quantity(self):
        assert (2 * qu.unitless) == 2
    
    def test_unitless_equal_to_unity(self):
        assert qu.unitless == 1


class TestConstants:
    def test_planck_constant(self):
        result = qc.Planck_constant
        assert repr(result) == "Constant(Planck_constant = 6.62607015E-34 J s)"
    
    def test_symbol(self):
        assert qc.h is qc.Planck_constant
    
    def test_abbrev_named_constant(self):
        assert qc.Planck is qc.Planck_constant

    def test_add_codata(self):
        from quanstants.constants import codata2018
        result = qc.vacuum_electric_permittivity
        assert repr(result) == "Constant(vacuum_electric_permittivity = 8.8541878128(13)E-12 F m⁻¹)"

    def test_as_quantity(self):
        E = qc.proton_mass * qc.speed_of_light**2
        assert repr(E) == "Quantity(1.503277615985125705245525892E-10, kg m² s⁻², uncertainty=4.583651411557769964000000001E-20)"
    
    def test_as_unit(self):
        result = qc.proton_mass.to((qp.M * qu.eV)/qc.c.as_unit()**2)
        assert repr(result) == "Quantity(938.2720881604903652873556334, MeV c⁻², uncertainty=2.860890187940270488303725942E-7)"


class TestRounding:
    a = (324.9 * qu.J) * (1.674 * qu.mol**-1)
    b = a.plus_minus(0.03)

    def test_round_to_places(self):
        result = self.a.round_to_places(3)
        assert repr(result) == "Quantity(543.883, J mol⁻¹)"

    def test_round_to_figures(self):
        result = self.a.round_to_figures(4)
        assert repr(result) == "Quantity(543.9, J mol⁻¹)"

    def test_round_to_sigfigs(self):
        result = self.a.round_to_sigfigs(2)
        assert repr(result) == "Quantity(5.4E+2, J mol⁻¹)"

    def test_round_to_places_default(self):
        result = self.a.round_to_places()
        assert repr(result) == "Quantity(543.88, J mol⁻¹)"

    def test_round_to_figures_default(self):
        result = self.a.round_to_figures()
        assert repr(result) == "Quantity(544, J mol⁻¹)"

    def test_round_pad(self):
        result = self.a.round_to_places(6)
        assert repr(result) == "Quantity(543.882600, J mol⁻¹)"
    
    def test_round_pad_off(self):
        result = self.a.round_to_places(6, pad=False)
        assert repr(result) == "Quantity(543.8826, J mol⁻¹)"

    def test_round_uncertainty(self):
        result = self.b.round_to_uncertainty()
        assert repr(result) == "Quantity(543.88, J mol⁻¹, uncertainty=0.03)"
    
    def test_round_uncertainty_no_uncertainty(self):
        result = self.a.round_to_uncertainty()
        assert repr(result) == "Quantity(543.8826, J mol⁻¹)"
    
    def test_round_mode_exact(self):
        result = self.a.round(method_if_uncertainty="PLACES", method_if_exact="FIGURES")
        assert repr(result) == "Quantity(544, J mol⁻¹)"
    
    def test_round_mode_uncert(self):
        result = self.b.round(method_if_uncertainty="PLACES", method_if_exact="FIGURES")
        assert repr(result) == "Quantity(543.88, J mol⁻¹, uncertainty=0.03)"

    def test_round_default(self):
        result = self.a.round()
        assert repr(result) == "Quantity(544, J mol⁻¹)"

    def test_round_default_uncert(self):
        result = self.b.round()
        assert repr(result) == "Quantity(543.88, J mol⁻¹, uncertainty=0.03)"

    def test_round_builtin(self):
        result = round(self.a, 5)
        assert repr(result) == "Quantity(543.88, J mol⁻¹)"

    def test_round_mode_different(self):
        assert round(dec("1.25"), 1) != ("1.25" * qu.m).round_to_places(1).number
    
    def test_set_rounding_mode(self):
        quanfig.ROUNDING_MODE = "ROUND_HALF_DOWN"
        assert repr(("1.25" * qu.m).round_to_places(1)) == "Quantity(1.2, m)"
        quanfig.ROUNDING_MODE = "ROUND_HALF_UP"
    
    def test_round_uncertainty(self):
        result = self.b.round_uncertainty(1)
        assert repr(result) == "Quantity(543.8826, J mol⁻¹, uncertainty=0.03)"


class TestTemperatures:
    def test_unit_equivalence(self):
        assert qu.degreeCelsius is qu.celsius is qu.degreeCentigrade

    def test_mult(self):
        assert 25 * qu.celsius == 25 * qu.kelvin

    def test_at(self):
        assert repr((25 @ qu.celsius).to(qu.kelvin)) == "Quantity(298.15, K)"

    def test_temp_arithmetic(self):
        T = 126.85 @ qu.celsius
        result = (1600 * qu.joule) / T
        assert repr(result) == "Quantity(4, J K⁻¹)"

    def test_temp_diff(self):
        result = (25 @ qu.celsius) - (-5 @ qu.celsius)
        assert repr(result) == "Quantity(30, °C)"

    def test_temp_inc(self):
        result = (25 @ qu.celsius) + (25 * qu.celsius)
        assert repr(result) == "Temperature(50, °C)"
    
    def test_temp_dec(self):
        result = (25 @ qu.celsius) - (25 * qu.celsius)
        assert repr(result) == "Temperature(0, °C)"

    def test_kelvin_to_celsius(self):
        result = ((273.15 * qu.kelvin) + (25 * qu.kelvin)).on_scale(qu.celsius)
        assert repr(result) == "Temperature(25.00, °C)"

    def test_celsius_to_fahrenheit(self):
        result = (0 @ qu.celsius).on_scale(qu.fahrenheit)
        assert repr(result) == "Temperature(32, °F)"