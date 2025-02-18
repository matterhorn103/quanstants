import astropy
import pytest

from quanstants import units as qu
from quanstants.interfaces.astropy import astropy_to_quanstants, from_astropy


class TestConversion:
    def test_from_astropy(self):
        for astropy_unit in astropy_to_quanstants.keys():
            from_astropy(astropy_unit)

    def test_base(self):
        for astropy_unit, quanstants_unit in astropy_to_quanstants.items():
            print(astropy_unit)
            try:
                astro_base = astropy_unit.decompose()
            except astropy.units.core.UnitConversionError:
                continue
            assert qu.parse(str(astro_base)) == from_astropy(astropy_unit).base() == quanstants_unit.base()