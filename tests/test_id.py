from quanstants import Dimensions, Unit128

class TestUnit128:
    def test_new_second(self):
        u = Unit128(0, 0x1100)

    def test_metre_to_hex(self):
        u = Unit128(0, 0x110000)
        assert u.to_hex() == "110000"
    
    def test_kilogram_from_hex(self):
        u = Unit128.from_hex("11000000")
        assert u == Unit128(0, 0x11000000)

