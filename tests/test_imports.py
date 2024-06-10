"""Simple tests to confirm the user won't have any problems importing anything."""


from quanstants.units import common


class TestImports:
    def test_init(self):
        import quanstants

    def test_toplevel(self):
        from quanstants import (
            units as qu,
            prefixes as qp,
            constants as qc,
            quanfig,
        )

    def test_all_units(self):
        # Note that some of these get imported at the top level anyway
        from quanstants import units as qu
        from quanstants.units import (
            atomic,
            chemistry,
            computing,
            imperial,
            natural,
            planck,
            temperatures,
            us,
        )

    def test_all_constants(self):
        # Note that some of these get imported at the top level anyway
        from quanstants import constants as qc
        from quanstants.constants import (
            codata2018,
            fundamental,
        )
