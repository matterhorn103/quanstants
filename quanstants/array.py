import pyarrow as pa

from .exceptions import MismatchedUnitsError
from .abstract_quantity import AbstractQuantity
from .abstract_unit import AbstractUnit


# Match precision of Python's Decimal for now
t_num = pa.decimal128(28)
t_unit = pa.string()
t_uncert = pa.decimal128(28)

dtype_quanstants = pa.struct(
    [
        ("number", t_num),
        ("unit", t_unit),
        ("uncertainty", t_uncert),
    ]
)


class QuanstantsArrowArray(pa.Array):

    def __init__(
        self,
        numbers: list,
        unit: AbstractUnit | None = None,
        uncertainties: list | None = None,
    ):
        if isinstance(numbers[0], AbstractQuantity):
            self._unit = str(numbers[0].unit)
            homogenous = True
            for q in numbers:
                homogenous &= (str(q.unit) == self._unit)
            if homogenous is False:
                raise MismatchedUnitsError
            super().__init__([(q.number, self._unit, q.uncertainty) for q in numbers], dtype=dtype_quanstants)
        else:
            self._unit = str(unit)
            super().__init__([(numbers[i], unit, uncertainties[i]) for i in range(len(numbers))], dtype=dtype_quanstants)

    
    @property
    def unit(self):
        return self._unit



class QuanstantsArray:

    def __init__(self, x, unit=None):
        if isinstance(x[0], AbstractQuantity):
            self.numbers = tuple(q.number for q in x)
            self.unit = x[0].unit
            homogenous = True
            for q in x:
                homogenous &= (q.unit == self.unit)
            if homogenous is False:
                raise MismatchedUnitsError
        else:
            self.numbers = x
            self.unit = unit

    def to(self, other):
        converted_unit = self.unit.value.to(other)
        new_numbers = [n * converted_unit.number for n in self.numbers]
        return QuanstantsArray(new_numbers, converted_unit.unit)