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


class TestParser:
    def test_single_unit(self):
        assert qu.parse("m") == qu.metre

    def test_compound_unit(self):
        assert qu.parse("kg s") == qu.kilogram * qu.second
