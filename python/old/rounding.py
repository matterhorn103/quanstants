"""Functions to round Decimals in various modes."""

import math

from decimal import Decimal as dec
from decimal import localcontext

# The package `rounders` provides this same functionality but long-term maintenance is
# unclear, and performance of these functions is signficantly better (~5x faster)
# `rounders` does however support float, Decimal and Fraction, not just Decimal, so
# can consider adopting it for the future

# Note that the default rounding mode of these functions is the same as that of the
# Decimal module itself, not that of quanstants as a whole, so generally needs to be
# overridden in function call

def to_places(num: dec, ndigits=0, pad=False, rounding="ROUND_HALF_EVEN"):
    """Round a Decimal to the specified number of decimal places."""
    current_places = num.as_tuple().exponent * -1
    # Don't round if padding is turned off and the number doesn't have enough places
    if (current_places < ndigits) and (not pad):
        return num
    # Set decimal rounding to the specified method
    # Use in a local context so that user's context isn't overwritten
    with localcontext() as ctx:
        ctx.rounding = rounding
        rounded = round(num, ndigits)
    return rounded


def to_figures(num: dec, ndigits=1, pad=False, rounding="ROUND_HALF_EVEN"):
    """Round a Decimal to the specified number of significant figures."""
    # Sanity check for requested number of sigfigs
    if ndigits < 1:
        return num
    digits = num.as_tuple().digits
    current_sigfigs = len(digits)

    # First deal with request for fewer sigfigs than currently (usual case)
    if ndigits <= current_sigfigs:
        exponent = math.floor(num.log10())
        significand = num / dec(f"1E{exponent}")
        # Set decimal rounding to the specified method
        # Use in a local context so that user's context isn't overwritten
        with localcontext() as ctx:
            ctx.rounding = rounding
            rounded_significand = round(significand, ndigits - 1)
        return rounded_significand * dec(f"1E{exponent}")
    
    # If request is for more sigfigs than currently, only pad if asked to do so
    elif (ndigits > current_sigfigs) and (not pad):
        return num
    elif (ndigits > current_sigfigs) and pad:
        # Add significant zeroes
        n_digits_to_add = ndigits - current_sigfigs
        new_digits = list(digits)
        for i in n_digits_to_add:
            new_digits.append(0)
        new_exponent = num.as_tuple().exponent - n_digits_to_add
        return dec((num.as_tuple().sign, new_digits, new_exponent))
    

def normalize(num: dec, threshold: int = 0):
    """Normalize a Decimal if the number of trailing zeroes is at least `threshold`."""
    if threshold == 0 or str(num)[-threshold:] == "0" * threshold:
        return num.normalize()
    else:
        return num