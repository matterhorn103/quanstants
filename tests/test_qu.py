from quanstants import qu, Unit

class TestMainApi:
    def test_units(self):
        units = qu.units

    def test_metre(self):
        m = qu.metre
        assert isinstance(m, Unit)

    def test_brackets_lookup(self):
        m1 = qu.units["metre"]
        m2 = qu.metre
        assert m1 == m2
