from decimal import Decimal as dec
from fractions import Fraction as frac

import pytest

from quanstants import qu, Unit, UnitId, UnitModule, Prefix


class TestMainApi:
    def test_units(self):
        _units = qu.units

    def test_units_brackets_lookup(self):
        m = qu.units["metre"]
        assert isinstance(m, Unit)

    def test_direct_brackets_lookup(self):
        m = qu["metre"]
        assert isinstance(m, Unit)

    def test_attribute(self):
        m = qu.metre
        assert isinstance(m, Unit)

    def test_units_symbol(self):
        m = qu.m
        assert isinstance(m, Unit)

    def test_unit_equivalence(self):
        m1 = qu.units["metre"]
        m2 = qu["metre"]
        m3 = qu.metre
        m4 = qu.m
        assert m1 == m2 == m3 == m4


class TestUnits:
    def test_id(self):
        m = qu.metre
        assert m.id == UnitId(0x110000)

    @pytest.mark.skip()
    def test_non_si_metric_units(self):
        # Just check that all those advertised are available
        qu.units.load_module(UnitModule.Metric)
        q = 45032.5 * qu["kilowatthour"]
        q = 1.27 * qu["carat"]

    def test_imperial(self):
        # Just check that all those advertised are available
        qu.units.load_module(UnitModule.Imperial)
        q = 6 * qu["foot"]
        q = 32 * qu["nautical_mile"]
        q = 32 * (qu.nano * qu["mile"])
        assert 32 * qu["nautical_mile"] != 32 * (qu.nano * qu["mile"])

    @pytest.mark.skip()
    def test_us_customary(self):
        # Just check that all those advertised are available
        qu.units.load_module(UnitModule.USCustomary)
        q = 6 * qu["foot"]
        q = 20 * qu["us_fluid_ounce"]
        q = 32 * qu["mile"]

    @pytest.mark.skip()
    def test_unit_common_between_modules(self):
        qu.units.load_module(UnitModule.Imperial)
        u1 = qu["foot"]
        qu.units.load_module(UnitModule.USCustomary)
        u2 = qu["foot"]
        assert u1 == u2
        assert u1.id == u2.id


class TestPrefixes:
    def test_prefixes(self):
        q = 50 * (qu.micro * qu.metre)
        assert str(q) == "50 μm"

    @pytest.mark.skip()
    def test_binary_symbols(self):
        assert (256 * (qu.giga * qu["byte"])) < (256 * (qu.gibi * qu["byte"]))

    def test_prefixed_units(self):
        # Just check that all those advertised are available
        q = 50 * qu.micrometre
        q = 27.2 * qu.kilowatt
        q = "99.7" * qu.megahertz


class TestQuantityCreation:
    def test_multiplication(self):
        q = 4 * qu.metre
        assert str(q) == "4 m"

    def test_symbol(self):
        q = 4 * qu.m
        assert str(q) == "4 m"

    def test_symbol_equivalence(self):
        assert qu.m == qu.metre

    def test_alt_spelling(self):
        q = 4 * qu.meter
        assert str(q) == "4 m"

    def test_alt_spelling_equivalence(self):
        assert qu.meter == qu.metre

    def test_unit_squared(self):
        q = 4 * qu.m**2
        assert str(q) == "4 m²"

    def test_unit_negative_exponent(self):
        q = 4 * qu.joule * qu.kg**-1
        assert str(q) == "4 J kg⁻¹"

    def test_division(self):
        q = 4 * qu.m / qu.s
        assert str(q) == "4 m s⁻¹"

    def test_int(self):
        q = 4010 * qu.m**3
        assert str(q) == "4010 m³"

    def test_float(self):
        q = 4.01 * qu.volt
        assert str(q) == "4.01 V"

    @pytest.mark.skip()
    def test_float_with_power_ten(self):
        q = "4.01e3" * qu.coulomb
        assert str(q) == "4.01E+3 C"

    def test_decimal(self):
        q = dec("0.401") * qu.newton * qu.m
        assert str(q) == "0.401 N m"

    def test_precision_retention_with_str(self):
        q = "741.60" * qu.g * qu.mol**-1
        assert str(q) == "741.60 g mol⁻¹"

    def test_quantity_instantiation(self):
        q = qu.quantity(0.997, qu.kg / qu.litre)
        assert str(q) == "0.997 kg L⁻¹"

    def test_quantity_creation_method_equivalence(self):
        q1 = qu.quantity(0.997, qu.kg / qu.litre)
        q2 = 0.997 * (qu.kg / qu.litre)
        assert q1 == q2


class TestParsing:
    def test_unit_symbol(self):
        assert repr(qu("4 m")) == "Quantity(4, m)"

    def test_unit_name(self):
        assert repr(qu("4 metre")) == "Quantity(4, m)"

    def test_reciprocal(self):
        assert repr(qu("741.60 g mol-1")) == "Quantity(741.60, g mol⁻¹)"

    def test_slash(self):
        assert repr(qu("0.997e3 g/L")) == "Quantity(997, g L⁻¹)"

    @pytest.mark.skip()
    def test_uncertainty_parentheses(self):
        assert (
            repr(qu("6.67430(15)E-11 N m² kg⁻²"))
            == "Quantity(6.67430e-11, N m² kg⁻², uncertainty=1.5e-15)"
        )

    @pytest.mark.skip()
    def test_uncertainty_plus_minus(self):
        assert repr(qu("8.293 ± 0.010 V")) == "Quantity(8.293, V, uncertainty=0.010)"

    @pytest.mark.skip()
    def test_uncertainty_plusslashminus(self):
        assert repr(qu("8.293 +/- 0.010 V")) == "Quantity(8.293, V, uncertainty=0.010)"


class TestUncertainties:
    def test_with_uncertainty(self):
        gravity = ("6.67430e-11" * qu.newton * qu.metre**2 * qu.kilogram**-2).with_uncertainty(
            "0.00015e-11"
        )
        assert repr(gravity) == "Quantity(6.67430e-11, N m² kg⁻², uncertainty=1.5e-15)"

    def test_plus_minus(self):
        assert repr(("4.2" * qu.m).plus_minus("0.2")) == "Quantity(4.2, m, uncertainty=0.2)"

    # Quantity as argument for with_uncertainty() no longer supported (at least not for now)
    #def test_uncertainty_as_quantity(self):
    #    assert (
    #        repr(("4.2" * qu.metre).plus_minus("20" * qu.centimetre))
    #        == "Quantity(4.2, m, uncertainty=0.20)"
    #    )

    @pytest.mark.skip()
    def test_str_parentheses(self):
        assert (
            str(
                ("6.67430e-11" * qu.newton * qu.metre**2 * qu.kilogram**-2).with_uncertainty(
                    "0.00015e-11"
                )
            )
            == "6.67430(15)e-11 N m² kg⁻²"
        )

    @pytest.mark.skip()
    def test_str_plus_minus(self):
        assert str(("4.2" * qu.metre).plus_minus("20" * qu.centimetre)) == "4.2 ± 0.20 m"

    @pytest.mark.skip()
    def test_set_uncertainty_style(self):
        qu.quanfig.uncertainty_style = "PLUSMINUS"
        assert (
            str(
                ("6.67430e-11" * qu.newton * qu.metre**2 * qu.kilogram**-2).with_uncertainty(
                    "0.00015e-11"
                )
            )
            == "6.67430e-11 ± 1.5e-15 N m² kg⁻²"
        )
        qu.quanfig.uncertainty_style = "PARENTHESES"

    @pytest.mark.skip()
    def test_uncertainty_at_quantity_creation(self):
        density = qu.quantity(0.99704702, qu.kg / qu.L, uncertainty=0.00000083)
        assert str(density) == "0.99704702(83) kg L⁻¹"

    def test_get_uncertainty(self):
        density = qu.quantity(0.99704702, qu.kg / qu.L, uncertainty=0.00000083)
        assert repr(density.uncertainty) == "Quantity(8.3e-7, kg L⁻¹)"


class TestArithmetic:
    def test_mul_quantity(self):
        result = repr((4 * qu.metre * qu.second**-1) * (6 * qu.second))
        assert result == "Quantity(24, m)"

    def test_mul_number(self):
        result = repr(("3.20" * qu.W) * "16.90")
        assert result == "Quantity(54.0800, W)"

    def test_div_quantity(self):
        result = repr((200 * qu.megajoule) / (70 * qu.kilogram))
        assert result == "Quantity(2.8571428571428571428571428571, MJ kg⁻¹)"

    def test_pow_int(self):
        result = repr((3 * qu.watt) ** 2)
        assert result == "Quantity(9, W²)"

    def test_pow_frac(self):
        result = repr((20 * qu.metre**2) ** frac(1, 2))
        assert result == "Quantity(4.472135954999579392818347337, m)"

    def test_add(self):
        result = repr((4 * qu.metre) + (0.5 * qu.metre))
        assert result == "Quantity(4.5, m)"

    def test_sub_mixed_prefixes(self):
        result = repr((4 * qu.metre) - (50 * qu.centimetre))
        assert result == "Quantity(3.50, m)"

    def test_add_mixed_compatible(self):
        qu.units.load_module(UnitModule.Imperial)
        result = repr((4 * qu.metre) + (2 * qu["foot"]))
        assert result == "Quantity(4.6096, m)"

    def test_add_mixed_mismatched(self):
        from quanstants import MismatchedUnitsError

        with pytest.raises(MismatchedUnitsError):
            result = repr((4 * qu.metre) + (3 * qu.kilogram))

    def test_greater_than_mixed_prefixes(self):
        assert (0.3 * qu.litre) > (150 * qu.millilitre)

    def test_greater_than_or_equal_to_mixed_prefixes(self):
        assert (0.15 * qu.litre) >= (150 * qu.millilitre)

    def test_greater_than_mixed_compatible(self):
        assert (0.3 * qu.litre) > (150 * qu.centimetre**3)

    def test_div_gives_dimensionless_result(self):
        a = 100 * qu.m
        b = 25 * qu.m
        assert (a / b).is_dimensionless()

    def test_div_gives_unitless_result(self):
        a = 100 * qu.m
        b = 25 * qu.m
        result = repr((a / b))
        assert result == "Quantity(4, (unitless))"

    @pytest.mark.skip()
    def test_rpow(self):
        a = 100 * qu.m
        b = 25 * qu.m
        result = repr(2 ** (a / b))
        assert result == "Quantity(16, (unitless))"

    @pytest.mark.skip()
    def test_sqrt(self):
        a = 100 * qu.m
        b = 25 * qu.m
        result = repr((a / b).sqrt())
        assert result == "Quantity(2, (unitless))"

    @pytest.mark.skip()
    def test_exp(self):
        a = 100 * qu.m
        b = 25 * qu.m
        result = repr((a / b).exp())
        assert result == "Quantity(54.59815003314423907811026120, (unitless))"

    @pytest.mark.skip()
    def test_ln(self):
        a = 100 * qu.m
        b = 25 * qu.m
        result = repr((a / b).ln())
        assert result == "Quantity(1.386294361119890618834464243, (unitless))"

    @pytest.mark.skip()
    def test_log10(self):
        a = 100 * qu.m
        b = 25 * qu.m
        result = repr((a / b).log10())
        assert result == "Quantity(0.6020599913279623904274777894, (unitless))"

    def test_add_with_uncertainty(self):
        a = (3 * qu.m).plus_minus(0.1)
        b = (2 * qu.m).plus_minus(0.2)
        result = repr(a + b)
        assert result == "Quantity(5, m, uncertainty=0.2236067977499789696409173669)"

    def test_sub_with_uncertainty(self):
        a = (3 * qu.m).plus_minus(0.1)
        b = (2 * qu.m).plus_minus(0.2)
        result = repr(a - b)
        assert result == "Quantity(1, m, uncertainty=0.2236067977499789696409173669)"

    def test_mul_with_uncertainty(self):
        a = (3 * qu.m).plus_minus(0.1)
        b = (2 * qu.m).plus_minus(0.2)
        result = repr(a * b)
        assert result == "Quantity(6, m², uncertainty=0.6324555320336758663997787090)"

    def test_div_with_uncertainty(self):
        a = (3 * qu.m).plus_minus(0.1)
        b = (2 * qu.m).plus_minus(0.2)
        result = repr(a / b)
        assert result == "Quantity(1.5, (unitless), uncertainty=0.1581138830084189665999446772)"

    @pytest.mark.skip()
    def test_rpow_with_uncertainty(self):
        a = (3 * qu.m).plus_minus(0.1)
        b = (2 * qu.m).plus_minus(0.2)
        result = repr(2 ** (a / b))
        assert (
            result
            == "Quantity(2.828427124746190097603377448, (unitless), uncertainty=0.3099848428288716908396318060)"
        )

    def test_add_with_correlated_uncertainty(self):
        a = (3 * qu.m).plus_minus(0.1)
        b = (2 * qu.m).plus_minus(0.2)
        result = repr(a.add_with_correlation(b, correlation=0.7))
        assert result == "Quantity(5, m, uncertainty=0.2792848008753788233976784908)"

    def test_sub_with_correlated_uncertainty(self):
        a = (3 * qu.m).plus_minus(0.1)
        b = (2 * qu.m).plus_minus(0.2)
        result = repr(a.sub_with_correlation(b, correlation=1))
        assert result == "Quantity(1, m, uncertainty=0.1)"


class TestConversion:
    def test_to_millimeter(self):
        result = (2 * qu.metre).in_unit(qu.millimetre)
        assert repr(result) == "Quantity(2E+3, mm)"

    def test_to_metre(self):
        qu.units.load_module(UnitModule.Imperial)
        result = (6 * qu["foot"]).in_unit(qu.metre)
        assert repr(result) == "Quantity(1.8288, m)"

    def test_to_second(self):
        result = (6 * qu.hour).in_unit(qu.s)
        assert repr(result) == "Quantity(21600, s)"

    def test_to_joule(self):
        result = ((3 * (qu.kilo * qu.watt)) * (1 * qu.day)).in_unit(qu.joule)
        assert repr(result) == "Quantity(2.59200E+8, J)"

    def test_base(self):
        result = (50 * qu.joule).in_base()
        assert repr(result) == "Quantity(50, kg m² s⁻²)"

    def test_cancelled_by_unit(self):
        result = (45 * (qu.m * qu.s * qu.s**-1)).cancelled_by_unit()
        assert repr(result) == "Quantity(45, m)"

    def test_cancelled_by_dimension_mixed_prefixes(self):
        result = ((30 * qu.kilowatt * qu.s) / (200 * qu.s * qu.watt**-1)).cancelled_by_dimension()
        assert repr(result) == "Quantity(0.00015, kW²)"

    def test_cancelled_by_dimension_mixed_compatible(self):
        qu.units.load_module(UnitModule.Imperial)
        result = ((3000 * qu.metre**2) / (20 * qu["foot"])).cancelled_by_dimension()
        assert repr(result) == "Quantity(492.1259842519685039370078740, m)"
    
    @pytest.mark.skip()
    def test_canonicized(self):
        mass = 20 * qu.kilogram
        acceleration = 3 * qu.metre * qu.second**-2
        ab = mass * acceleration
        ba = acceleration * mass
        assert repr(ab) != repr(ba)
        assert repr(ab.canonicized()) == repr(ba.canonicized())


class TestEqualities:
    def test_mixed_prefixes(self):
        assert 3 * qu.kilometre == 3000 * qu.metre

    def test_uncertainty_noneffect(self):
        assert 3 * qu.kilometre == (3000 * qu.metre).plus_minus(20)

    def test_units(self):
        assert qu.watt == qu.joule / qu.second

    def test_unit_quantity_comparison(self):
        assert qu.watt == 1 * qu.joule * qu.s**-1

    def test_zero_quantity_equal_to_zero(self):
        assert 0 * qu.m == 0

    def test_two_zero_quantities(self):
        assert 0 * qu.m == 0 * qu.s

    def test_unitless_quantity_equal_to_numeric(self):
        assert (2 * qu.one) == 2

    def test_one_unit_equal_to_one_int(self):
        assert qu.one == 1

@pytest.mark.skip()
class TestConstants:
    def test_planck_constant(self):
        result = qu.constants["Planck constant"]
        assert repr(result) == "Constant(Planck constant = 6.62607015E-34 J s)"

    def test_symbol(self):
        assert qu.constants.get_by_symbol("h") == qu.constants["Planck constant"]

    def test_abbrev_named_constant(self):
        assert qu.constants["Planck"] == qu.constants["Planck constant"]

    # def test_add_codata(self):
    #    qu.constants.load_module(ConstantsModule.Codata2018)
    #
    #    result = qu.constants["vacuum_electric_permittivity"]
    #    assert repr(result) == "Constant(vacuum_electric_permittivity = 8.8541878128(13)E-12 F m⁻¹)"

    def test_as_quantity(self):
        E = qu.constants["proton mass"] * qu.constants["speed of light"] ** 2
        assert (
            repr(E)
            == "Quantity(1.503277615985125705245525892E-10, kg m² s⁻², uncertainty=4.583651411557769964000000001E-20)"
        )

    def test_as_unit(self):
        result = qu.constants["proton mass"].in_unit(
            (qu.mega * qu.eV) / qu.constants["speed of light"].as_unit() ** 2
        )
        assert (
            repr(result)
            == "Quantity(938.2720881604903652873556334, MeV c⁻², uncertainty=2.860890187940270488303725942E-7)"
        )

@pytest.mark.skip()
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
        qu.quanfig.rounding_mode = "ROUND_HALF_DOWN"
        assert repr(("1.25" * qu.m).round_to_places(1)) == "Quantity(1.2, m)"
        qu.quanfig.rounding_mode = "ROUND_HALF_UP"
