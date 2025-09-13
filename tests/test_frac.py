from quanstants import qu, Frac


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
        assert str(f) == "1/2"

    def test_frac_repr(self):
        f = Frac(3, 2)
        assert repr(f) == "Frac(3, 2)"

    def test_frac_from_byte0(self):
        f = Frac(0, 1)
        assert f == Frac.from_byte(0x00)

    def test_frac_from_byte1(self):
        f = Frac(2, 1)
        assert f == Frac.from_byte(0x12)

    def test_frac_from_byte2(self):
        f = Frac(1, 2)
        assert f == Frac.from_byte(0x21)
    
    def test_frac_from_byte3(self):
        f = Frac(-1, 1)
        assert f == Frac.from_byte(0xF1)

    def test_frac_from_byte4(self):
        f = Frac(-2, 1)
        assert f == Frac.from_byte(0xF2)

    def test_frac_from_byte5(self):
        f = Frac(-1, 2)
        assert f == Frac.from_byte(0xE1)

    def test_frac_to_byte0(self):
        f = Frac(0, 1)
        assert f.to_byte() == 0x00

    def test_frac_to_byte1(self):
        f = Frac(2, 1)
        assert f.to_byte() == 0x12

    def test_frac_to_byte2(self):
        f = Frac(1, 2)
        assert f.to_byte() == 0x21
    
    def test_frac_to_byte3(self):
        f = Frac(-1, 1)
        assert f.to_byte() == 0xF1

    def test_frac_to_byte4(self):
        f = Frac(-2, 1)
        assert f.to_byte() == 0xF2

    def test_frac_to_byte5(self):
        f = Frac(-1, 2)
        assert f.to_byte() == 0xE1

    def test_frac_from_hex0(self):
        f = Frac(0, 1)
        assert f == Frac.from_hex("00")

    def test_frac_from_hex1(self):
        f = Frac(2, 1)
        assert f == Frac.from_hex("12")

    def test_frac_from_hex2(self):
        f = Frac(1, 2)
        assert f == Frac.from_hex("21")
    
    def test_frac_from_hex3(self):
        f = Frac(-1, 1)
        assert f == Frac.from_hex("F1")

    def test_frac_from_hex4(self):
        f = Frac(-2, 1)
        assert f == Frac.from_hex("F2")

    def test_frac_from_hex5(self):
        f = Frac(-1, 2)
        assert f == Frac.from_hex("E1")

    def test_frac_to_hex0(self):
        f = Frac(0, 1)
        assert f.to_hex() == "00"

    def test_frac_to_hex1(self):
        f = Frac(2, 1)
        assert f.to_hex() == "12"

    def test_frac_to_hex2(self):
        f = Frac(1, 2)
        assert f.to_hex() == "21"
    
    def test_frac_to_hex3(self):
        f = Frac(-1, 1)
        assert f.to_hex() == "F1"

    def test_frac_to_hex4(self):
        f = Frac(-2, 1)
        assert f.to_hex() == "F2"

    def test_frac_to_hex5(self):
        f = Frac(-1, 2)
        assert f.to_hex() == "E1"
