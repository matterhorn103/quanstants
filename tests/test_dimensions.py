from quanstants import Dimensions

class TestDim:
    def test_equality(self):
        dim1 = Dimensions(0, 1, 0, 2, 0, 0, 0)
        dim2 = Dimensions(0, 1, 0, 2, 0, 0, 0)
        assert dim1 == dim2
    
    def test_mul(self):
        dim1 = Dimensions(0, 1, 0, 2, 0, 0, 0)
        dim2 = Dimensions(2, 0, 0, 2, 0, 0, 0)
        assert dim1 * dim2 == Dimensions(2, 1, 0, 4, 0, 0, 0)

    def test_div(self):
        dim1 = Dimensions(0, 1, 0, 2, 0, 0, 0)
        dim2 = Dimensions(2, 0, 0, 2, 0, 0, 0)
        assert dim1 / dim2 == Dimensions(-2, 1, 0, 0, 0, 0, 0)

    def test_pow(self):
        dim1 = Dimensions(0, 1, 0, 2, 0, 0, 0)
        assert dim1.pow(2) == Dimensions(0, 2, 0, 4, 0, 0, 0)
