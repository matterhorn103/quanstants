from quanstants import Dimensions, UnitId

class TestUnitId:
    def test_new_second(self):
        u = UnitId(0x1100)

    def test_metre_to_repr(self):
        u = UnitId(0x110000)
        assert repr(u) == "UnitId(0x110000)"

    def test_metre_to_str(self):
        u = UnitId(0x110000)
        assert str(u) == "0x110000"
