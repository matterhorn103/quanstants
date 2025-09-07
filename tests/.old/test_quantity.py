from decimal import Decimal as dec

from quanstants import (
    units as qu,
    prefixes as qp,
    constants as qc,
    Quantity,
    quanfig,
)


class TestRounding:
    def test_round_default(self):
        q = (123.456789 * qu.m).plus_minus(0.000543)


class TestParser:
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

    def test_parser_with_uncertainty(self):
        number = "1.234"
        unit = qu.m * qu.s**-2
        uncertainty = "0.056"
        string_to_parse = "1.234(56) m s-2"
        assert Quantity(number, unit, uncertainty) == Quantity(string_to_parse)

    def test_parser_with_uncertainty_plus_minus_unicode(self):
        number = "1.234"
        unit = qu.m * qu.s**-2
        uncertainty = "0.056"
        string_to_parse = "1.234 ± 0.056 m s-2"
        assert Quantity(number, unit, uncertainty) == Quantity(string_to_parse)

    def test_parser_with_uncertainty_plus_minus_unicode_nospaces(self):
        number = "1.234"
        unit = qu.m * qu.s**-2
        uncertainty = "0.056"
        string_to_parse = "1.234±0.056 m s-2"
        assert Quantity(number, unit, uncertainty) == Quantity(string_to_parse)

    def test_parser_with_uncertainty_plus_minus_ascii(self):
        number = "1.234"
        unit = qu.m * qu.s**-2
        uncertainty = "0.056"
        string_to_parse = "1.234 +/- 0.056 m/s2"
        assert Quantity(number, unit, uncertainty) == Quantity(string_to_parse)

    def test_parser_with_uncertainty_plus_minus_ascii_nospaces(self):
        number = "1.234"
        unit = qu.m * qu.s**-2
        uncertainty = "0.056"
        string_to_parse = "1.234+/-0.056 m/s2"
        assert Quantity(number, unit, uncertainty) == Quantity(string_to_parse)
