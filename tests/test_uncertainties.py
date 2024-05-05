from decimal import Decimal as dec

import pytest

from quanstants import (
    units as qu,
)

class TestUncertaintyArithmetic:
    q1 = (20 * qu.m).with_uncertainty(2)
    q2 = (30 * qu.m).with_uncertainty(5)

    def test_addition(self):
        assert round((self.q1 + self.q2).uncertainty, 5) == round("5.3851648071345" * qu.m, 5)
    
    def test_subtraction(self):
        assert round((self.q1 - self.q2).uncertainty, 5) == round("5.3851648071345" * qu.m, 5)

    def test_multiplication(self):
        assert round((self.q1 * self.q2).uncertainty, 5) == round("116.619037896906" * qu.m**2, 5)

    def test_division(self):
        assert round((self.q1 / self.q2).uncertainty, 5) == round("0.129576708774340" * qu.unitless, 5)

    def test_division_reversed(self):
        assert round((self.q2 / self.q1).uncertainty, 5) == round("0.2915475947422" * qu.unitless, 5)

    def test_exponention(self):
        assert (self.q1 ** 2).uncertainty == "80" * qu.m**2
    
    def test_exponention_reverse(self):
        assert round((3 ** (self.q1 / self.q2)).uncertainty, 5) == round("0.296109426930146" * qu.unitless, 5)
    
    def test_natural_log(self):
        assert round(((self.q1 / self.q2).ln()).uncertainty, 5) == round("0.194365063161" * qu.unitless, 5)

    def test_log_base10(self):
        assert round(((self.q1 / self.q2).log10()).uncertainty, 5) == round("0.08441167440582" * qu.unitless, 5)
    
    def test_exp(self):
        assert round(((self.q1 / self.q2).exp()).uncertainty, 5) == round("0.25238096660761" * qu.unitless, 5)