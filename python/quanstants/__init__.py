from quanstants._quanstants import (
    BaseUnit,
    Dimensions,
    Prefix,
)

__all__ = [
    BaseUnit,
    Dimensions,
    Prefix,
]

metre = BaseUnit("m", "metre", Dimensions(1, 0, 0, 0, 0, 0, 0))
