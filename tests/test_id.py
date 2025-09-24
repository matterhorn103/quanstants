from quanstants import Dimensions, UnitId

class TestUnitId:
    def test_new_second(self):
        u = UnitId(0, 0x1100)

    def test_metre_to_str(self):
        u = UnitId(0, 0x110000)
        assert str(u) == "UnitId(110000)"
