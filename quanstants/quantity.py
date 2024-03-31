from decimal import Decimal as dec

class Quantity:
    def __init__(self, number: int | float | dec, unit):
        self.number = number
        self.unit = unit
    
    def __str__(self):
        return f"Quantity({self.number} {self.unit})"