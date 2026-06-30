from decimal import Decimal as dec
from textwrap import wrap

from .config import quanfig


def split_number(number: float | dec) -> tuple[str, str, str]:
    """Split a float or Decimal's string into integer, fraction, and exponent parts."""
    number_string = str(number)
    # Split into pre and post decimal point
    parts = number_string.split(".")
    if len(parts) > 1:
        integer = parts[0]
        # Split off exponential part if there is one
        if "e" in parts[1]:
            fraction_plus_exponent = parts[1].split("e")
        elif "E" in parts[1]:
            fraction_plus_exponent = parts[1].split("E")
        else:
            fraction_plus_exponent = [parts[1]]
        fraction = fraction_plus_exponent[0]
        if len(fraction_plus_exponent) > 1:
            exponent = fraction_plus_exponent[1]
        else:
            exponent = ""
    else:
        # There was no decimal point
        fraction = ""
        if "e" in parts[0]:
            integer_plus_exponent = parts[0].split("e")
        elif "E" in parts[0]:
            integer_plus_exponent = parts[0].split("E")
        else:
            integer_plus_exponent = [parts[0]]
        integer = integer_plus_exponent[0]
        if len(integer_plus_exponent) > 1:
            exponent = integer_plus_exponent[1]
        else:
            exponent = ""

    return integer, fraction, exponent


def truncate_digits(
        integer: str,
        fraction: str,
        threshold: int = 8,
    ) -> tuple[str, str, str]:
    """Limit a long decimal in length and abbreviate truncated digits as an ellipsis."""
    
    places = len(fraction)
    # Work out number of sigfigs
    figures_integer = 0
    figures = 0
    for digit in integer:
        if digit != "0" or figures > 0:
            figures_integer += 1
            figures += 1
    for digit in fraction:
        if digit != "0" or figures > 0:
            figures += 1
    
    # With threshold = 2 we want to truncate e.g.
    # 2.456 to 2 d.p. -> 2.45…
    # 0.0002456 to 2 s.f. -> 0.00024…
    # 0.0002 not at all
    # 245.6 not at all
    # 0.2456 to either 2 d.p. or 2 s.f. -> 0.24…
    if places > threshold and figures > threshold:
        ellipsis = "…"
        if places < figures:
            fraction = fraction[:threshold]
        else:
            n_to_keep = threshold - figures_integer
            fraction = fraction[:n_to_keep]
    else:
        ellipsis = ""

    return integer, fraction, ellipsis


def group_digits(
        integer: str,
        fraction: str,
        which: str = "ALL",
        n: int = 3,
        sep: str = " ",
    ) -> tuple[str, str]:
    """Group digits into groups of n, separated by the given separator."""

    if n == 0:
        return integer, fraction
    
    if which in ["ALL", "INTEGER"]:
        n_digits = len(integer)
        n_leading = n_digits % n
        if n_leading > 0:
            groups = [integer[:n_leading]]
        else:
            groups = []
        if n_digits >= 5:
            groups += wrap(integer[n_leading:], n)
            integer = sep.join(groups)

    if which in ["ALL", "FRACTION"]:
        n_digits = len(fraction)
        if n_digits >= 5:
            groups = wrap(fraction, n)
            fraction = sep.join(groups)

    return integer, fraction


def format_parts(
        integer: str,
        fraction: str,
        exponent: str,
        truncate: int = 8,
        group: int = 3,
        group_which: str = "ALL",
        group_sep: str = " ",
    ) -> tuple[str, str, str, str]:
    """Prepare the parts of a float or Decimal's string for printing."""

    # By default there is no ellipsis
    ellipsis = ""

    # If no options were passed, there is nothing to do
    if (truncate | group) is False:
        return integer, fraction, exponent
    
    if truncate:
        integer, fraction, ellipsis = truncate_digits(integer, fraction, threshold=truncate)

    if group:
        integer, fraction = group_digits(integer, fraction, which=group_which, sep=group_sep)
    
    return integer, fraction, ellipsis, exponent


def reassemble(
        integer: str,
        fraction: str | None = None,
        ellipsis: str | None = None,
        exponent: str | None = None,
    ) -> str:
    """Join the constituent parts of a float or Decimal with the appropriate symbols."""
    result = integer
    if fraction:
        result += "." + fraction
        if ellipsis:
            result += ellipsis
    if exponent:
        result += "E" + exponent
    
    return result


def format_number(
        number,
        truncate=8,
        group=3,
        group_which="ALL",
        group_sep=" ",
    ) -> str:
    """Prepare a float or Decimal's string for printing."""
    
    parts = split_number(number)

    integer, fraction, ellipsis, exponent = format_parts(
        *parts,
        truncate=truncate,
        group=group,
        group_which=group_which,
        group_sep=group_sep
    )

    # Reassemble constituent parts
    result = reassemble(integer, fraction, ellipsis, exponent)
    
    return result


def format_quantity(
        quantity,
        truncate=8,
        group=3,
        group_which="ALL",
        group_sep=" ",
        uncertainty_style="PLUSMINUS",
    ):
    """Prepare a string for printing from a quantity object."""

    number_parts = split_number(quantity.number)
    uncertainty_parts = split_number(quantity._uncertainty)

    num_integer, num_fraction, num_ellipsis, num_exponent = format_parts(
        *number_parts,
        truncate=truncate,
        group=group,
        group_which=group_which,
        group_sep=group_sep,
    )
    uncert_integer, uncert_fraction, uncert_ellipsis, uncert_exponent = format_parts(
        *uncertainty_parts,
        truncate=truncate,
        group=group,
        group_which=group_which,
        group_sep=group_sep,
    )

    elide_any = bool(num_ellipsis) | bool(uncert_ellipsis)
    if not quantity._uncertainty:
        num_string = reassemble(num_integer, num_fraction, num_ellipsis, num_exponent)
        return f"{num_string}{quantity.unit._preceding_space*" "}{quantity.unit}"
    
    # Under certain conditions format uncertainty in style "3.023(6) m" but only if:
    # - the uncertainty has the same resolution as the number (check via the exponent,
    # where more negative (smaller) exponent means more precise)
    # - there are no ellipses
    elif (
        uncertainty_style == "PARENTHESES"
        and elide_any is False
        and quantity.resolution() == quantity.uncertainty.resolution()
    ):
        num_string = reassemble(num_integer, num_fraction)
        # Enclose significant figures of uncertainty in parentheses
        figures_uncert = ""
        for digit in uncert_integer + uncert_fraction:
            if digit != "0" or figures_uncert:
                figures_uncert += digit
        bracketed_uncert = f"({figures_uncert})"
        # Add exponention symbol back in
        if num_exponent:
            num_exponent = "E" + num_exponent
        # Insert uncertainty before exponent
        return f"{num_string}{bracketed_uncert}{num_exponent}{quantity.unit._preceding_space*" "}{quantity.unit}"
    
    else:
        num_string = reassemble(num_integer, num_fraction, num_ellipsis, num_exponent)
        uncert_string = reassemble(uncert_integer, uncert_fraction, uncert_ellipsis, uncert_exponent)
        return f"{num_string} ± {uncert_string} {quantity.unit}"