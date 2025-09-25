from quanstants import Frac


class TestFrac:

    def test_positive_frac(self):
        f = Frac(1, 2)

    def test_frac_equality(self):
        f1 = Frac(1, 2)
        f2 = Frac(1, 2)
        assert f1 == f2

    def test_negative_frac(self):
        f1 = Frac(-1, 2)
        f2 = Frac(1, -2)
        assert f1 == f2

    def test_improper_frac(self):
        f = Frac(3, 2)

    def test_frac_str(self):
        f = Frac(1, 2)
        print(f)
        assert str(f) == "1⁄2"

    def test_frac_repr(self):
        f = Frac(3, 2)
        assert repr(f) == "Frac(3, 2)"

    def test_frac_from_bits(self):
        assert Frac(0, 1) == Frac.from_bits(0x00)
        assert Frac(2, 1) == Frac.from_bits(0x12)
        assert Frac(1, 2) == Frac.from_bits(0x21)
        assert Frac(-1, 1) == Frac.from_bits(0xF1)
        assert Frac(-2, 1) == Frac.from_bits(0xF2)
        assert Frac(-1, 2) == Frac.from_bits(0xE1)

    def test_frac_to_bits(self):
        assert Frac(0, 1).to_bits() == 0x00
        assert Frac(2, 1).to_bits() == 0x12
        assert Frac(1, 2).to_bits() == 0x21
        assert Frac(-1, 1).to_bits() == 0xF1
        assert Frac(-2, 1).to_bits() == 0xF2
        assert Frac(-1, 2).to_bits() == 0xE1
