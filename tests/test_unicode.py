from decimal import Decimal as dec
from fractions import Fraction as frac

import pytest

from quanstants import (
    unicode,
    QuanstantsConfig,
)

class TestGenerateSuperscript:
    def test_unity(self):
        exponent = 1
        assert unicode.generate_superscript(exponent) == ""

    def test_positive_integer(self):
        exponent = 2
        assert unicode.generate_superscript(exponent) == "²"

    def test_negative_integer(self):
        exponent = -2
        assert unicode.generate_superscript(exponent) == "⁻²"

    def test_integer_zero(self):
        exponent = 0
        assert unicode.generate_superscript(exponent) == "⁰"

    def test_large_integer(self):
        exponent = 12
        assert unicode.generate_superscript(exponent) == "¹²"

    def test_large_negative_integer(self):
        exponent = -34
        assert unicode.generate_superscript(exponent) == "⁻³⁴"

    def test_positive_fraction(self):
        exponent = frac(1, 2)
        assert unicode.generate_superscript(exponent) == "¹⁄₂"

    def test_improper_fraction(self):
        exponent = frac(3, 2)
        assert unicode.generate_superscript(exponent) == "³⁄₂"
    
    def test_negative_fraction(self):
        exponent = frac(-1, 2)
        assert unicode.generate_superscript(exponent) == "⁻¹⁄₂"
    
    def test_fraction_integer(self):
        exponent = frac(4, 2)
        assert unicode.generate_superscript(exponent) == "²"

    def test_fraction_unity(self):
        exponent = frac(3, 3)
        assert unicode.generate_superscript(exponent) == ""
    
    def test_fraction_zero(self):
        exponent = frac(0, 2)
        assert unicode.generate_superscript(exponent) == "⁰"
    
    def test_float(self):
        exponent = 2.3
        with pytest.raises(Exception):
            unicode.generate_superscript(exponent)

    def test_float_integer(self):
        exponent = 2.0
        assert unicode.generate_superscript(exponent) == "²"

    def test_decimal(self):
        exponent = dec("2.3")
        with pytest.raises(Exception):
            unicode.generate_superscript(exponent)
    
    def test_decimal_integer(self):
        exponent = dec("2.0")
        # Currently also doesn't work as there is no Decimal.is_integer()
        with pytest.raises(Exception):
            unicode.generate_superscript(exponent)

    def test_unicode_superscripts_off(self):
        QuanstantsConfig.UNICODE_SUPERSCRIPTS = False
        # List of all exponents tested above, all should just return their string representation
        exponents = [1, 2, -2, 0, 12, -34, frac(1, 2), frac(3, 2), frac(-1, 2), frac(4, 2), frac(3, 3), frac(0, 2), 2.3, 2.0, dec("2.3"), dec("2.0")]
        for exponent in exponents:
            assert unicode.generate_superscript(exponent) == str(exponent)

