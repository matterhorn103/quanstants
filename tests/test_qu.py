from quanstants import qu, Unit, UnitId

class TestMainApi:
    def test_units(self):
        units = qu.units

    def test_metre(self):
        m = qu.metre
        assert isinstance(m, Unit)

    def test_brackets_lookup(self):
        m = qu.units["metre"]
        assert isinstance(m, Unit)

    def test_unit_equivalence(self):
        m1 = qu.units["metre"]
        m2 = qu.metre
        assert m1 == m2


class TestUnits:
    def test_id(self):
        m = qu.metre
        assert m.id == UnitId(0x110000)
