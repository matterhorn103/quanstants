
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

# Also provide function to generate a superscript string
def generate_superscript(exponent: int):
    if exponent == 1:
        superscript = ""
    elif abs(exponent) <= 9:
        superscript = _unicode_superscripts[exponent]
    else:
        superscript = ""
        for char in str(int):
            if char == "-":
                superscript += "⁻"
            else:
                superscript += _unicode_superscripts[int(char)]
    return superscript