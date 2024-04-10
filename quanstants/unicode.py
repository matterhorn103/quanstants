from fractions import Fraction as frac

from .config import QuanstantsConfig

# Dictionary of correspondence between Factor.exponent, which is just an integer,
# and the appropriate Unicode symbol. For now, only hard-code up to |9|
_unicode_superscripts = {
    1: "¹",
    2: "²",
    3: "³",
    4: "⁴",
    5: "⁵",
    6: "⁶",
    7: "⁷",
    8: "⁸",
    9: "⁹",
    0: "⁰",
    -1: "⁻¹",
    -2: "⁻²",
    -3: "⁻³",
    -4: "⁻⁴",
    -5: "⁻⁵",
    -6: "⁻⁶",
    -7: "⁻⁷",
    -8: "⁻⁸",
    -9: "⁻⁹",
    }

_unicode_subscripts = {
    1: "₁",
    2: "₂",
    3: "₃",
    4: "₄",
    5: "₅",
    6: "₆",
    7: "₇",
    8: "₈",
    9: "₉",
    0: "₀",
    -1: "₋₁",
    -2: "₋₂",
    -3: "₋₃",
    -4: "₋₄",
    -5: "₋₅",
    -6: "₋₆",
    -7: "₋₇",
    -8: "₋₈",
    -9: "₋₉",
    }

def multidigit(number: int, sub=False):
    result = ""
    for char in str(number):
        if char == "-":
            if sub:
                result += "₋"
            else:
                result += "⁻"
        else:
            if sub:
                result += _unicode_subscripts[int(char)]
            else:
                result += _unicode_superscripts[int(char)]
    return result

# Also provide function to generate a superscript string
def generate_superscript(exponent: int | frac):
    if not QuanstantsConfig.UNICODE_SUPERSCRIPTS:
        superscript = str(exponent)
    elif exponent == 1:
        superscript = ""
    elif exponent.is_integer() and (abs(exponent) <= 9):
        superscript = _unicode_superscripts[int(exponent)]
    elif isinstance(exponent, frac):
        superscript = multidigit(exponent.numerator) + "⁄" + multidigit(exponent.denominator, sub=True)
    else:
        superscript = multidigit(exponent)
    return superscript