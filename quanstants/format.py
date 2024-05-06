from textwrap import wrap

from .config import quanfig


def group_digits(number):
    number_string = str(number)
    if not quanfig.GROUP_DIGITS:
        return number_string
    # Split into pre and post decimal point
    parts = number_string.split(".")
    # Split off exponential part if there is one
    if len(parts) > 1:
        parts = [parts[0]] + parts[1].split("E")
    else:
        parts = parts[0].split("E")
    if quanfig.GROUP_DIGITS_STYLE in ["all", "integer"]:
        integer = parts[0]
        n_digits = len(integer)
        n_leading = n_digits % 3
        if n_leading > 0:
            groups = [integer[:n_leading]]
        else:
            groups = []
        if n_digits >= 5:
            groups += wrap(integer[n_leading:], 3)
            formatted = quanfig.GROUP_SEPARATOR.join(groups)
        else:
            formatted = integer
    else:
        formatted = parts[0]
    if len(parts) == 2 and parts[1].isnumeric():
        if quanfig.GROUP_DIGITS_STYLE in ["all", "decimal"]:
            decimal = parts[1]
            n_digits = len(decimal)
            if n_digits >= 5:
                groups = wrap(decimal, 3)
                decimal_formatted = quanfig.GROUP_SEPARATOR.join(groups)
            else:
                decimal_formatted = decimal
        else:
            decimal_formatted = parts[1]
        formatted = ".".join([formatted, decimal_formatted])
    if not parts[-1].isnumeric():
        formatted = "E".join([formatted, parts[-1]])
    return formatted
