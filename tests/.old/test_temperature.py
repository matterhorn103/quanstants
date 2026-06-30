from decimal import Decimal as dec

from quanstants import (
    units as qu,
    prefixes as qp,
    constants as qc,
    temperature,
    Quantity,
    quanfig,
)
from quanstants.units import temperatures


class TestCelsius:
    def test_temp_creation(self):
        t = 50 @ qu.celsius
        assert repr(t) == "Temperature(50, °C)"

    def test_quant_creation(self):
        q = 50 * qu.celsius
        assert repr(q) == "Quantity(50, °C)"

    def test_temp_quant_inequivalence(self):
        assert (50 * qu.celsius == 50 @ qu.celsius) is False

    def test_temp_increase(self):
        t = 50 @ qu.celsius
        q = 50 * qu.celsius
        assert t + q == 100 @ qu.celsius

    def test_temp_decrease(self):
        t = 50 @ qu.celsius
        q = 50 * qu.celsius
        assert t - q == 0 @ qu.celsius

    def test_conversion_from_kelvin(self):
        q = 250 * qu.kelvin
        assert repr(q.on_scale(qu.celsius)) == "Temperature(-23.15, °C)"

    def test_temp_conversion_at_0(self):
        t = 0 @ qu.celsius
        assert t.base().number == dec("273.15")

    def test_temp_conversion_at_150(self):
        t = 150 @ qu.celsius
        assert t.base().number == dec("423.15")

    def test_temp_conversion_at_0K(self):
        t = -273.15 @ qu.celsius
        assert t.base().number == dec("0")

    def test_temp_difference(self):
        t1 = 75 @ qu.celsius
        t2 = -20 @ qu.celsius
        assert t1 - t2 == 95 * qu.celsius

    def test_temp_addition(self):
        t1 = 75 @ qu.celsius
        t2 = -20 @ qu.celsius
        assert t1 + t2 == "601.3" * qu.kelvin

    def test_celsius_fahrenheit_intersection(self):
        t = -40 @ qu.celsius
        assert t.on_scale(qu.fahrenheit).number == dec("-40")

    def test_conversion_to_fahrenheit_at_0(self):
        t = 0 @ qu.celsius
        assert t.on_scale(qu.fahrenheit) == 32 @ qu.fahrenheit

    def test_conversion_to_fahrenheit_scale(self):
        t = 232 @ qu.celsius
        assert t.on_scale(qu.fahrenheit) == "449.6" @ qu.fahrenheit

    def test_conversion_to_fahrenheit_unit(self):
        t = 232 @ qu.celsius
        assert t.to(qu.fahrenheit) == "909.27" * qu.fahrenheit

    def test_use_in_equation(self):
        H = -50 * qu.kilojoule * qu.mole**-1
        S = 20 * qu.joule * qu.kelvin**-1 * qu.mole**-1
        T = 75 @ qu.celsius
        G = H - T * S
        assert G == "-56.963" * qu.kilojoule * qu.mole**-1
    
    # TODO test rounding and other methods that haven't been overloaded
