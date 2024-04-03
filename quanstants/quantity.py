from decimal import Decimal as dec

class MismatchedUnitsError(Exception):
    pass

class Quantity:
    def __init__(self, number: int | float | dec, unit):
        self.number = number
        self.unit = unit
    
    def __str__(self):
        return f"Quantity({self.number} {self.unit.symbol})"
    
    def __add__(self, other):
        if isinstance(other, Quantity):
            if self.unit == other.unit:
                return Quantity(self.number + other.number, self.unit)
            else:
                raise MismatchedUnitsError
        else:
            return NotImplemented
    
    def __sub__(self, other):
        if isinstance(other, Quantity):
            if self.unit == other.unit:
                return Quantity(self.number + other.number, self.unit)
            else:
                raise MismatchedUnitsError
        else:
            return NotImplemented

    def __mul__(self, other):
        if isinstance(other, (int, float, dec)):
            return Quantity(self.number * other, self.unit)
        elif isinstance(other, Quantity):
            return Quantity(self.number * other.number, self.unit * other.unit)
        # Check if it's a unit with duck typing
        elif hasattr(other, "_isinbase"):
            # Note that Q * U only needs to be defined to allow syntax like 3 * units.m * units.s**-2
            return Quantity(self.number, self.unit * other)
        else:
            return NotImplemented
    
    def __rmul__(self, other):
        if isinstance(other, (int, float, dec)):
            return Quantity(other * self.number, self.unit)
        else:
            return NotImplemented
    
    def __div__(self, other):
        if isinstance(other, (int, float, dec)):
            return Quantity(self.number / other, self.unit)
        elif isinstance(other, Quantity):
            return Quantity(self.number / other.number, self.unit / other.unit)
        # Check if it's a unit with duck typing
        elif hasattr(other, "_isinbase"):
            return Quantity(self.number, self.unit / other)
        else:
            return NotImplemented
    
    def __rdiv__(self, other):
        if isinstance(other, (int, float, dec)):
            return Quantity(other / self.number, self.unit)
        else:
            return NotImplemented
    
    def __neg__(self):
        return Quantity(-1 * self.number, self.unit)
    
    def __pos__(self):
        return self
    
    def base(self):
        return Quantity(self.number, self.unit.base())