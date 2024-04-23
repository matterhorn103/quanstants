from decimal import Decimal as dec

from quanstants import (
    units as qu,
    prefixes as qp,
    constants as qc,
    Quantity,
    quanfig,
)

class TestQuantity:
    def test_parser(self):
        number = "3.25"
        unit = qu.m * qu.s**-2
        unit_string = "m s-2"
        string_to_parse = number + " " + unit_string
        assert Quantity(number, unit) == Quantity.parse(string_to_parse)
    
    def test_parser_recursive(self):
        q = 3.25 * qu.m * qu.s**-2
        assert Quantity.parse(str(q)) == q

    def test_parser_init(self):
        number = "3.25"
        unit = qu.m * qu.s**-2
        unit_string = "m s-2"
        string_to_parse = number + " " + unit_string
        assert Quantity(number, unit) == Quantity(string_to_parse)