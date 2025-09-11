from quanstants import qu, Exponent


class TestFrac:

    def test_positive_frac(self):
        f = Exponent(1, 2)

    def test_frac_equality(self):
        f1 = Exponent(1, 2)
        f2 = Exponent(1, 2)
        assert f1 == f2

    def test_negative_frac(self):
        f1 = Exponent(-1, 2)
        f2 = Exponent(1, -2)
        assert f1 == f2

    def test_improper_frac(self):
        f = Exponent(3, 2)

    def test_frac_str(self):
        f = Exponent(1, 2)
        print(f)
