from decimal import Decimal as dec

def get_uncertainty(numerical_result, operation, quantityA, quantityB, correlation):
    """Find the uncertainty in the result of an operation between two quantities.
    
    For a given mathematical operation f, as specified amongst a few options by `operation`,
    calculates the uncertainty sigma_c in the result of QuantityC = f(QuantityA, QuantityB), where:
        A, B, C are the numerical value of each Quantity
        sigma_a, sigma_b, sigma_c are their respective uncertainties
        sigma_ab is the covariance = rho_ab * sigma_a * sigma_b, where
        rho_ab is the correlation between A and B
    """
    A = quantityA.number
    B = quantityB.number
