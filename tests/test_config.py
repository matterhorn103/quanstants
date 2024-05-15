from pathlib import Path

import pytest

from quanstants import (
    units as qu,
    prefixes as qp,
    constants as qc,
    Quantity,
    quanfig,
)

class TestToml:
    def test_units_toml(self):
        quanfig.load_toml(["units"], "tests/custom_unit.toml")
        assert qu.thm == Quantity("3.595E+16", qu.kg * qu.m**2 * qu.s**-2)
    
    def test_constants_toml(self):
        quanfig.load_toml(["constants"], "tests/custom_constant.toml")
        assert repr(qc.european_swallow_airspeed_velocity) == "Constant(swallow_airspeed_velocity = 9 m s⁻¹)"