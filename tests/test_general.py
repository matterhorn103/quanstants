import decimal
from decimal import Decimal as dec
from fractions import Fraction as frac

from quanstants import qu, Unit, UnitId

class TestMainApi:
    def test_units(self):
        units = qu.units

    def test_metre(self):
        m = qu.metre
        assert isinstance(m, Unit)

    def test_brackets_lookup(self):
        m = qu.units["metre"]
        assert isinstance(m, Unit)

    def test_unit_equivalence(self):
        m1 = qu.units["metre"]
        m2 = qu.metre
        assert m1 == m2


class TestUnits:
    def test_id(self):
        m = qu.metre
        assert m.id == UnitId(0x110000)


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
        q = 4 * qu["joule"] * qu.kg**-1
        assert str(q) == "4 J kg⁻¹"

    def test_division(self):
        q = 4 * qu.m / qu.s
        assert str(q) == "4 m s⁻¹"

    def test_int(self):
        q = 4010 * qu.m**3
        assert str(q) == "4010 m³"

    def test_float(self):
        q = 4.01 * qu["volt"]
        assert str(q) == "4.01 V"

    def test_float_with_power_ten(self):
        q = "4.01e3" * qu["coulomb"]
        assert str(q) == "4.01E+3 C"

    def test_decimal(self):
        q = dec("0.401") * qu["newton"] * qu.m
        assert str(q) == "0.401 N m"

    def test_precision_retention_with_str(self):
        q = "741.60" * qu.g * qu.mol**-1
        assert str(q) == "741.60 g mol⁻¹"

    def test_quantity_instantiation(self):
        q = qu(0.997, qu.kg / qu["litre"])
        assert str(q) == "0.997 kg L⁻¹"

    def test_quantity_creation_method_equivalence(self):
        q1 = qu(0.997, qu.kg / qu["litre"])
        q2 = 0.997 * (qu.kg / qu["litre"])
        assert q1 == q2
