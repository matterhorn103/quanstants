from decimal import Decimal as dec
from itertools import combinations
import math

import pytest
from uncertainties import ufloat, umath
import sigfig

from quanstants import (
    units as qu,
)


class TestUncertaintyArithmetic:
    q1 = (20 * qu.m).with_uncertainty(2)
    q2 = (30 * qu.m).with_uncertainty(5)

    def test_addition(self):
        assert round((self.q1 + self.q2).uncertainty, 5) == round(
            "5.3851648071345" * qu.m, 5
        )

    def test_subtraction(self):
        assert round((self.q1 - self.q2).uncertainty, 5) == round(
            "5.3851648071345" * qu.m, 5
        )

    def test_multiplication(self):
        assert round((self.q1 * self.q2).uncertainty, 5) == round(
            "116.619037896906" * qu.m**2, 5
        )

    def test_division(self):
        assert round((self.q1 / self.q2).uncertainty, 5) == round(
            "0.129576708774340" * qu.unitless, 5
        )

    def test_division_reversed(self):
        assert round((self.q2 / self.q1).uncertainty, 5) == round(
            "0.2915475947422" * qu.unitless, 5
        )

    def test_exponention(self):
        assert (self.q1**2).uncertainty == "80" * qu.m**2

    def test_exponention_reverse(self):
        assert round((3 ** (self.q1 / self.q2)).uncertainty, 5) == round(
            "0.296109426930146" * qu.unitless, 5
        )

    def test_natural_log(self):
        assert round(((self.q1 / self.q2).ln()).uncertainty, 5) == round(
            "0.194365063161" * qu.unitless, 5
        )

    def test_log_base10(self):
        assert round(((self.q1 / self.q2).log10()).uncertainty, 5) == round(
            "0.08441167440582" * qu.unitless, 5
        )

    def test_exp(self):
        assert round(((self.q1 / self.q2).exp()).uncertainty, 5) == round(
            "0.25238096660761" * qu.unitless, 5
        )


class TestAgainstUncertaintiesPackage:
    a = ("12" * qu.m).with_uncertainty("0.23")
    b = ("470" * qu.m).with_uncertainty("75.6")
    c = ("2.0493e7" * qu.m).with_uncertainty("0.0364e7")
    d = ("8.9228e-4" * qu.m).with_uncertainty("0.1637e-4")
    e = ("0" * qu.m).with_uncertainty("0.5")
    f = ("-16.7" * qu.m).with_uncertainty("4.227")
    g = ("-4060" * qu.m).with_uncertainty("54")
    h = ("-12.532374e9" * qu.m).with_uncertainty("0.00827e9")
    i = ("-7.121e-2" * qu.m).with_uncertainty("0.444e-2")
    quants = [a, b, c, d, e, f, g, h, i]
    quants_no_zero = [q for q in quants if q.number != 0]

    def test_addition(self):
        results_uncertainties = ()
        results_quanstants = ()
        for combo in combinations(self.quants, 2):
            u0 = ufloat(float(combo[0].number), float(combo[0]._uncertainty))
            u1 = ufloat(float(combo[1].number), float(combo[1]._uncertainty))
            # uncertainties package doesn't actually seem that reliable so don't let it derail our test
            try:
                result_u = u0 + u1
                # Turn each number to a Decimal first and round to 8 dp using
                # Decimal's rounding to ensure equality
                results_uncertainties += (
                    sigfig.round(dec(str(result_u.nominal_value)), 10, warn=False),
                    sigfig.round(dec(str(result_u.std_dev)), 10, warn=False),
                )
            except:  # pragma: no cover
                continue
            result_q = combo[0] + combo[1]
            results_quanstants += (
                sigfig.round(result_q.number, 10, warn=False),
                sigfig.round(result_q._uncertainty, 10, warn=False),
            )
        # Round values close to zero to zero
        results_uncertainties = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_uncertainties
        )
        results_quanstants = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_quanstants
        )
        assert results_uncertainties == results_quanstants

    def test_subtraction(self):
        results_uncertainties = ()
        results_quanstants = ()
        for combo in combinations(self.quants, 2):
            u0 = ufloat(float(combo[0].number), float(combo[0]._uncertainty))
            u1 = ufloat(float(combo[1].number), float(combo[1]._uncertainty))
            try:
                result_u = u0 - u1
                results_uncertainties += (
                    sigfig.round(dec(str(result_u.nominal_value)), 10, warn=False),
                    sigfig.round(dec(str(result_u.std_dev)), 10, warn=False),
                )
            except:  # pragma: no cover
                continue
            result_q = combo[0] - combo[1]
            results_quanstants += (
                sigfig.round(result_q.number, 10, warn=False),
                sigfig.round(result_q._uncertainty, 10, warn=False),
            )
        results_uncertainties = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_uncertainties
        )
        results_quanstants = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_quanstants
        )
        assert results_uncertainties == results_quanstants

    def test_multiplication(self):
        results_uncertainties = ()
        results_quanstants = ()
        for combo in combinations(self.quants_no_zero, 2):
            u0 = ufloat(float(combo[0].number), float(combo[0]._uncertainty))
            u1 = ufloat(float(combo[1].number), float(combo[1]._uncertainty))
            try:
                result_u = u0 * u1
                results_uncertainties += (
                    sigfig.round(dec(str(result_u.nominal_value)), 10, warn=False),
                    sigfig.round(dec(str(result_u.std_dev)), 10, warn=False),
                )
            except:  # pragma: no cover
                continue
            result_q = combo[0] * combo[1]
            results_quanstants += (
                sigfig.round(result_q.number, 10, warn=False),
                sigfig.round(result_q._uncertainty, 10, warn=False),
            )
        results_uncertainties = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_uncertainties
        )
        results_quanstants = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_quanstants
        )
        assert results_uncertainties == results_quanstants

    def test_division(self):
        results_uncertainties = ()
        results_quanstants = ()
        for combo in combinations(self.quants_no_zero, 2):
            u0 = ufloat(float(combo[0].number), float(combo[0]._uncertainty))
            u1 = ufloat(float(combo[1].number), float(combo[1]._uncertainty))
            try:
                result_u = u0 / u1
                results_uncertainties += (
                    sigfig.round(dec(str(result_u.nominal_value)), 10, warn=False),
                    sigfig.round(dec(str(result_u.std_dev)), 10, warn=False),
                )
            except:  # pragma: no cover
                continue
            result_q = combo[0] / combo[1]
            results_quanstants += (
                sigfig.round(result_q.number, 10, warn=False),
                sigfig.round(result_q._uncertainty, 10, warn=False),
            )
        results_uncertainties = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_uncertainties
        )
        results_quanstants = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_quanstants
        )
        assert results_uncertainties == results_quanstants

    def test_division_reversed(self):
        results_uncertainties = ()
        results_quanstants = ()
        for combo in combinations(self.quants_no_zero, 2):
            u0 = ufloat(float(combo[0].number), float(combo[0]._uncertainty))
            u1 = ufloat(float(combo[1].number), float(combo[1]._uncertainty))
            try:
                result_u = u1 / u0
                results_uncertainties += (
                    sigfig.round(dec(str(result_u.nominal_value)), 10, warn=False),
                    sigfig.round(dec(str(result_u.std_dev)), 10, warn=False),
                )
            except:  # pragma: no cover
                continue
            result_q = combo[1] / combo[0]
            results_quanstants += (
                sigfig.round(result_q.number, 10, warn=False),
                sigfig.round(result_q._uncertainty, 10, warn=False),
            )
        results_uncertainties = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_uncertainties
        )
        results_quanstants = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_quanstants
        )
        assert results_uncertainties == results_quanstants

    def test_exponention(self):
        results_uncertainties = ()
        results_quanstants = ()
        for q in self.quants_no_zero:
            u0 = ufloat(float(q.number), float(q._uncertainty))
            try:
                result_u = u0**5
                results_uncertainties += (
                    sigfig.round(dec(str(result_u.nominal_value)), 10, warn=False),
                    sigfig.round(dec(str(result_u.std_dev)), 10, warn=False),
                )
            except:  # pragma: no cover
                continue
            result_q = q**5
            results_quanstants += (
                sigfig.round(result_q.number, 10, warn=False),
                sigfig.round(result_q._uncertainty, 10, warn=False),
            )
        results_uncertainties = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_uncertainties
        )
        results_quanstants = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_quanstants
        )
        assert results_uncertainties == results_quanstants

    def test_exponention_reverse(self):
        results_uncertainties = ()
        results_quanstants = ()
        for combo in combinations(self.quants_no_zero, 2):
            u0 = ufloat(float(combo[0].number), float(combo[0]._uncertainty))
            u1 = ufloat(float(combo[1].number), float(combo[1]._uncertainty))
            try:
                result_u = 5 ** (u1 / u0)
                results_uncertainties += (
                    sigfig.round(dec(str(result_u.nominal_value)), 10, warn=False),
                    sigfig.round(dec(str(result_u.std_dev)), 10, warn=False),
                )
            except:  # pragma: no cover
                continue
            result_q = 5 ** (combo[1] / combo[0])
            results_quanstants += (
                sigfig.round(result_q.number, 10, warn=False),
                sigfig.round(result_q._uncertainty, 10, warn=False),
            )
        results_uncertainties = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_uncertainties
        )
        results_quanstants = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_quanstants
        )
        assert results_uncertainties == results_quanstants

    def test_natural_log(self):
        results_uncertainties = ()
        results_quanstants = ()
        for combo in combinations(self.quants_no_zero, 2):
            u0 = ufloat(float(combo[0].number), float(combo[0]._uncertainty))
            u1 = ufloat(float(combo[1].number), float(combo[1]._uncertainty))
            try:
                result_u = umath.log(u1 / u0)
                results_uncertainties += (
                    sigfig.round(dec(str(result_u.nominal_value)), 10, warn=False),
                    sigfig.round(dec(str(result_u.std_dev)), 10, warn=False),
                )
            except:  # pragma: no cover
                continue
            result_q = (combo[1] / combo[0]).ln()
            results_quanstants += (
                sigfig.round(result_q.number, 10, warn=False),
                sigfig.round(result_q._uncertainty, 10, warn=False),
            )
        results_uncertainties = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_uncertainties
        )
        results_quanstants = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_quanstants
        )
        assert results_uncertainties == results_quanstants

    def test_log_base10(self):
        results_uncertainties = ()
        results_quanstants = ()
        for combo in combinations(self.quants_no_zero, 2):
            u0 = ufloat(float(combo[0].number), float(combo[0]._uncertainty))
            u1 = ufloat(float(combo[1].number), float(combo[1]._uncertainty))
            try:
                result_u = umath.log10(u1 / u0)
                results_uncertainties += (
                    sigfig.round(dec(str(result_u.nominal_value)), 10, warn=False),
                    sigfig.round(dec(str(result_u.std_dev)), 10, warn=False),
                )
            except:  # pragma: no cover
                continue
            result_q = (combo[1] / combo[0]).log10()
            results_quanstants += (
                sigfig.round(result_q.number, 10, warn=False),
                sigfig.round(result_q._uncertainty, 10, warn=False),
            )
        results_uncertainties = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_uncertainties
        )
        results_quanstants = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_quanstants
        )
        assert results_uncertainties == results_quanstants

    def test_exp(self):
        results_uncertainties = ()
        results_quanstants = ()
        for combo in combinations(self.quants_no_zero, 2):
            u0 = ufloat(float(combo[0].number), float(combo[0]._uncertainty))
            u1 = ufloat(float(combo[1].number), float(combo[1]._uncertainty))
            try:
                result_u = umath.exp(u1 / u0)
                results_uncertainties += (
                    sigfig.round(dec(str(result_u.nominal_value)), 10, warn=False),
                    sigfig.round(dec(str(result_u.std_dev)), 10, warn=False),
                )
            except:  # pragma: no cover
                continue
            result_q = (combo[1] / combo[0]).exp()
            results_quanstants += (
                sigfig.round(result_q.number, 10, warn=False),
                sigfig.round(result_q._uncertainty, 10, warn=False),
            )
        results_uncertainties = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_uncertainties
        )
        results_quanstants = tuple(
            x if abs(x) > dec("1e-100") else dec("0") for x in results_quanstants
        )
        assert results_uncertainties == results_quanstants
