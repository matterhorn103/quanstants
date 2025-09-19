from quanstants import Dimensions, UnitId

class TestUnitId:
    def test_new_second(self):
        u = UnitId(0, 0x1100)

    def test_metre_to_hex(self):
        u = UnitId(0, 0x110000)
        assert u.to_hex() == "110000"
    
    def test_kilogram_from_hex(self):
        u = UnitId.from_hex("11000000")
        assert u == UnitId(0, 0x11000000)

