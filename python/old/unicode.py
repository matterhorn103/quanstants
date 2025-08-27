from fractions import Fraction as frac

from .config import quanfig


# Function to turn a tuple or other iterable of unit terms into a symbol
def generate_symbol(
    components: tuple[tuple, ...],
    sort_by="sign",
    inverse=None,
) -> str:
    # Default to preference set in config for slash vs negative exponents
    inverse = quanfig.INVERSE_UNIT if inverse is None else inverse
    # Create symbol as concatenation of symbols of components, with spaces
    terms = []
    positive_terms = []
    negative_terms = []
    for factor in components:
        term = factor[0].symbol
        if factor[1] >= 0:
            term += generate_superscript(factor[1])
            if sort_by == "sign":
                positive_terms.append(term)
            else:
                terms.append(term)
        elif factor[1] < 0:
            if (inverse == "NEGATIVE_SUPERSCRIPT") or (sort_by != "sign"):
                term += generate_superscript(factor[1])
            elif (inverse == "SLASH") and (sort_by == "sign"):
                term += generate_superscript(-1 * factor[1])
            if sort_by == "sign":
                negative_terms.append(term)
            else:
                terms.append(term)
    if sort_by == "sign":
        if len(negative_terms) > 0:
            if inverse == "NEGATIVE_SUPERSCRIPT":
                return " ".join(positive_terms) + " " + " ".join(negative_terms)
            elif inverse == "SLASH":
                return " ".join(positive_terms) + "/" + " ".join(negative_terms)
        else:
            return " ".join(positive_terms)
    else:
        return " ".join(terms)


# Dictionary of correspondence between a term's exponent, which is just an integer,
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


def generate_superscript(exponent: int | frac) -> str:
    """Generate a superscipt string from an integer or fraction.

    This is used to generate symbols for compound units and therefore to prepare their printed
    representation.
    Using Unicode superscript characters to represent exponents can be turned off by setting
    `quanfig.UNICODE_SUPERSCRIPTS = False`, in which case the result is simply the same as
    `str(exponent)`.
    """
    if not quanfig.UNICODE_SUPERSCRIPTS:
        superscript = str(exponent)
    elif exponent == 1:
        superscript = ""
    # Can't just do int(exponent) because float and Decimal get rounded to an integer by int
    elif (abs(exponent) <= 9) and (isinstance(exponent, int) or exponent.is_integer()):
        superscript = _unicode_superscripts[int(exponent)]
    elif isinstance(exponent, frac):
        superscript = (
            multidigit(exponent.numerator)
            + "⁄"
            + multidigit(exponent.denominator, sub=True)
        )
    else:
        superscript = multidigit(exponent)
    return superscript


def exponent_parser(exponent: str) -> int | frac:
    """Generate an integer or fraction as appropriate from a string of Unicode representing an exponent.

    Does the reverse of `generate_superscript()`.
    """
    # First try the simple case, where the string is normal ASCII
    if exponent.isascii():
        try:
            return int(exponent)
        except ValueError:
            # Is a fraction
            # Cancel to integer if possible
            if frac(exponent).is_integer():
                return int(frac(exponent))
            else:
                return frac(exponent)
    # If not, we are dealing with a string of superscripts
    # Invert super/subscript dicts
    inverse_superscripts = {v: k for k, v in _unicode_superscripts.items()}
    inverse_subscripts = {v: k for k, v in _unicode_subscripts.items()}
    terms_ascii = []
    for term in exponent.split("⁄"):
        term_ascii = ""
        for char in term:
            if char == "⁻":
                term_ascii += "-"
            elif char in inverse_superscripts:
                term_ascii += str(inverse_superscripts[char])
            elif char in inverse_subscripts:
                term_ascii += str(inverse_subscripts[char])
        terms_ascii.append(term_ascii)
    if len(terms_ascii) == 2:
        return frac(int(terms_ascii[0]), int(terms_ascii[1]))
    elif len(terms_ascii) == 1:
        return int(terms_ascii[0])
