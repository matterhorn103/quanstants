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

    def test_frac_from_bits0(self):
        f = Frac(0, 1)
        assert f == Frac.from_bits(0x00)

    def test_frac_from_bits1(self):
        f = Frac(2, 1)
        assert f == Frac.from_bits(0x12)

    def test_frac_from_bits2(self):
        f = Frac(1, 2)
        assert f == Frac.from_bits(0x21)
    
    def test_frac_from_bits3(self):
        f = Frac(-1, 1)
        assert f == Frac.from_bits(0xF1)

    def test_frac_from_bits4(self):
        f = Frac(-2, 1)
        assert f == Frac.from_bits(0xF2)

    def test_frac_from_bits5(self):
        f = Frac(-1, 2)
        assert f == Frac.from_bits(0xE1)

    def test_frac_to_bits0(self):
        f = Frac(0, 1)
        assert f.to_bits() == 0x00

    def test_frac_to_bits1(self):
        f = Frac(2, 1)
        assert f.to_bits() == 0x12

    def test_frac_to_bits2(self):
        f = Frac(1, 2)
        assert f.to_bits() == 0x21
    
    def test_frac_to_bits3(self):
        f = Frac(-1, 1)
        assert f.to_bits() == 0xF1

    def test_frac_to_bits4(self):
        f = Frac(-2, 1)
        assert f.to_bits() == 0xF2

    def test_frac_to_bits5(self):
        f = Frac(-1, 2)
        assert f.to_bits() == 0xE1
