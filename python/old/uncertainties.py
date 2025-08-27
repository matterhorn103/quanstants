from decimal import Decimal as dec

from .config import quanfig
from .rounding import normalize


def get_uncertainty(
    numerical_result,
    operation,
    quantityA,
    quantityB=None,
    numberx=None,
    correlation=0,
    log_base=None,
):
    """Find the uncertainty in the result of an operation on a quantity.

    Some operations require
    For a given mathematical operation f, as chosen from a few options with `operation`,
    calculates the uncertainty sigma_c in the result of either QuantityC = f(QuantityA, QuantityB),
    QuantityC = f(QuantityA, numberx), or QuantityC = f(QuantityA) where:
        A, B, C are the numerical value of each Quantity
        x is the value of numberx, an exact and dimensionless number
        sigma_a, sigma_b, sigma_c are their respective uncertainties
        sigma_ab is the covariance = rho_ab * sigma_a * sigma_b, where
        rho_ab is the correlation between A and B

    Uncertainties are then calculated as follows according to the value of `operation`:
    """
    A, sigma_a = quantityA.number, quantityA._uncertainty
    if quantityB:
        B, sigma_b = quantityB.number, quantityB._uncertainty
    else:
        B, sigma_b = None, dec(0)
    C = numerical_result
    x = numberx
    if quanfig.CONVERT_FLOAT_AS_STR:
        correlation = dec(str(correlation))
    else:
        correlation = dec(correlation)
    sigma_ab = correlation * sigma_a * sigma_b

    # Ideally this would all be implemented using another package -
    # `uncertainties` would be a candidate for example, but it unfortunately sometimes
    # fails inexplicably and it  doesn't support Decimal, so just implement the
    # calculations ourselves for a limited set of operations
    
    # Exact quantities always give an exact quantity
    if sigma_a == sigma_b == 0:
        sigma_c = dec(0)
    # C = A + B
    elif operation == "add":
        if B is None:
            raise TypeError("Operation 'add' is only supported for Q + Q.")
        sigma_c = ((sigma_a**2) + (sigma_b**2) + (2 * sigma_ab)).sqrt()
    # C = A - B
    elif operation == "sub":
        if B is None:
            raise TypeError("Operation 'sub' is only supported for Q - Q.")
        sigma_c = ((sigma_a**2) + (sigma_b**2) - (2 * sigma_ab)).sqrt()
    # C = AB and C = xA
    elif operation == "mul":
        if B is not None:
            sigma_c = (
                abs(C)
                * (
                    ((sigma_a / A) ** 2) + ((sigma_b / B) ** 2) + (2 * (sigma_ab / C))
                ).sqrt()
            )
        elif x is not None:
            sigma_c = x * sigma_a
        else:
            raise TypeError(
                "Operation 'mul' is only supported for Q * Q, x * Q, and Q * x."
            )
    # C = A/B and C = A/x
    elif operation == "truediv":
        if B is not None:
            sigma_c = (
                abs(C)
                * (
                    ((sigma_a / A) ** 2) + ((sigma_b / B) ** 2) - (2 * (sigma_ab / C))
                ).sqrt()
            )
        elif x is not None:
            sigma_c = (1 / x) * sigma_a
        else:
            raise TypeError("Operation 'truediv' is only supported for Q/Q and Q/x.")
    # C = x/A
    elif operation == "rtruediv":
        if B is not None:
            raise TypeError("Operation is only supported for x/Q.")
        elif x is not None:
            # Work out uncertainty as if it was C = xA**-1 - ends up just being the relative error times the result
            sigma_c = abs((C * -1 * sigma_a) / A)
    # C = A**B and C = A**x
    elif operation == "pow":
        if B is not None:
            sigma_c = (
                abs(C)
                * (
                    ((B / A) * sigma_a) ** 2
                    + (A.ln() * sigma_b) ** 2
                    + (2 * ((B * A.ln()) / A) * sigma_ab)
                ).sqrt()
            )
        elif x is not None:
            sigma_c = abs((C * x * sigma_a) / A)
    # C = B**A and C = x**A
    elif operation == "rpow":
        if B is not None:
            sigma_c = (
                abs(C)
                * (
                    ((A / B) * sigma_b) ** 2
                    + (B.ln() * sigma_a) ** 2
                    + (2 * ((A * B.ln()) / B) * sigma_ab)
                ).sqrt()
            )
        elif x is not None:
            sigma_c = abs(C) * abs(x.ln() * sigma_a)
    # C = ln(A)
    elif operation == "ln":
        sigma_c = abs(sigma_a / A)
    # C = log10(A)
    elif operation == "log10":
        sigma_c = abs(sigma_a / (dec(10).ln() * A))
    # C = logx(A)
    elif operation == "log":
        sigma_c = abs(sigma_a / (dec(log_base).ln() * A))
    # C = e**A
    elif operation == "exp":
        sigma_c = abs(C) * abs(sigma_a)

    # Ensure no endless trailing zeroes
    #if quanfig.AUTO_NORMALIZE:
    #    sigma_c = normalize(sigma_c, threshold=quanfig.AUTO_NORMALIZE)
    return sigma_c
