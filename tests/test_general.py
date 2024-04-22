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
        q = Quantity(0.997, qu.kg/qu.L)
        assert str(q) == "0.997 kg L⁻¹"
    
    def test_quantity_creation_method_equivalence(self):
        q1 = Quantity(0.997, qu.kg/qu.L)
        q2 = 0.997 * (qu.kg/qu.L)
        assert q1 == q2


class TestUnitsAndPrefixes:
    def test_imperial(self):
        from quanstants import imperial
        q = 6 * qu.foot
        assert str(q) == "6 ft"
        
    def test_us(self):
        from quanstants import us
        q = 20 * qu.us_fluid_ounce
        assert str(q) == "20 fl oz"