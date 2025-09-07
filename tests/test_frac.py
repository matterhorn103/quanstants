from quanstants import qu, Frac16


class TestFrac:

    def test_positive_frac(self):
        f = Frac16(1, 2)

    def test_frac_equality(self):
        f1 = Frac16(1, 2)
        f2 = Frac16(1, 2)
        assert f1 == f2

    def test_negative_frac(self):
        f1 = Frac16(-1, 2)
        f2 = Frac16(1, -2)
        assert f1 == f2

    def test_improper_frac(self):
        f = Frac16(3, 2)

    def test_frac_str(self):
        f = Frac16(1, 2)
        print(f)
