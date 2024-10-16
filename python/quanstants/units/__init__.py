"""Namespace to contain all the units, making them useable with qu.m notation."""

import inspect

from ..config import quanfig
from ..exceptions import AlreadyDefinedError, ParsingError
from ..unicode import exponent_parser

# Note there is no need to import units from other modules as they are
# added to this namespace programmatically


def add(name: str, unit):
    """Add a `Unit` object to the module under the provided name.
    This method provides a safe way to add units to the namespace.
    Names in the module's namespace cannot be overwritten in this way, and attempting to add a unit
    under a name that is already defined will raise an `AlreadyDefinedError`.
    If it is necessary to redefine a name, do it by setting the variable in the normal way i.e.
    `units.already_assigned_name = new_value`.
    """
    if name in globals():
        raise AlreadyDefinedError
    globals()[name] = unit

def list_names(include_prefixed=True, prefixed_only=False):
    """Return a list of all unit names in the namespace, in human-readable format i.e. as strings.
    By specifying the appropriate options, prefixed units can be included or filtered out, or only
    prefixed units can be requested.
    Essentially just return the value of `globals().keys()` but with anything that isn't a unit
    filtered out.
    Note that a) the values returned are variable names as strings, not the `Unit` objects themselves
    and b) a unit is typically listed multiple times under different names, as well as under its symbol
    if it has `canon_symbol=True`.
    """
    filtered_names = {name for name, obj in globals().items() if hasattr(obj, "alt_names")}
    prefixed = {
        name for name in filtered_names if hasattr(name, "prefix")
    }
    if prefixed_only:
        return list(prefixed)
    elif include_prefixed:
        return list(filtered_names)
    else:
        return list(filtered_names - prefixed)
    
def list_units(include_prefixed=True, prefixed_only=False):
    """Return a list of all `Unit` objects currently in the registry.
    By specifying the appropriate options, prefixed units can be included or filtered out, or only
    prefixed units can be requested.
    Unlike `list_names()`, the values are `Unit` objects not strings, and each unit is only listed
    once, regardless of how many names it is registered under in the registry.
    """
    filtered_names = list_names(include_prefixed, prefixed_only)
    unique_units = {globals()[name] for name in filtered_names}
    return list(unique_units)

def search(search_string: str) -> dict:
    """Return all unit names in the namespace for which the provided search string is found in its metadata.
    The results are returned as a dictionary, with each item being the results of each search method
    separated into lists of exact matches and substring matches.
    For now the function just searches in each unit's symbol and name.
    Searching for "ft" means, for example, that all units with the symbol "ft" will be returned, not
    just the unit found under `units.ft`.
    """
    symbol_results = {"exact": [], "partial": []}
    name_results = {"exact": [], "partial": []}
    all_names = list_names()
    for name in all_names:
        if search_string == name:
            name_results["exact"].append(name)
        elif search_string in name:
            name_results["partial"].append(name)
        if search_string == globals()[name].symbol:
            symbol_results["exact"].append(name)
        elif search_string in globals()[name].symbol:
            symbol_results["partial"].append(name)
    # Order each subset by length of name
    symbol_results = {k: sorted(v, key=len) for k, v in symbol_results.items()}
    name_results = {k: sorted(v, key=len) for k, v in name_results.items()}
    results = {"symbol": symbol_results, "name": name_results}
    return results

def match(search_string: str):
    """Return the best match for the provided search string."""
    if search_string in globals():
        return globals()[search_string]
    else:
        search = search(search_string)
        if len(search["symbol"]["exact"]) > 0:
            return globals()[search["symbol"]["exact"][0]]
        elif len(search["name"]["exact"]) > 0:
            return globals()[search["name"]["exact"][0]]
        elif len(search["symbol"]["partial"]) > 0:
            return globals()[search["symbol"]["partial"][0]]
        elif len(search["name"]["partial"]) > 0:
            return globals()[search["name"]["partial"][0]]
        
def _create_term(unit_string, exponent_string):
    if len(unit_string) < 1:
        return
    exponent_string.replace("−", "-")
    if len(exponent_string) < 1:
        exponent = 1
    else:
        try:
            exponent = int(exponent_string)
        except ValueError:
            exponent = exponent_parser(exponent_string)
    term = globals()[unit_string] ** exponent
    return term

def parse(string: str):
    """Take a string of unit symbols or names and digits and convert to an appropriate Unit object.
    Units may be specified by their symbols (which may be non-ASCII Unicode) or by one of their
    defined names in the registry (which consist of ASCII letters, numbers, and underscores "_" only).
    The string should ideally take a form such as `"kg m2 s-1"`, with terms separated by spaces and
    exponents given as ASCII integers directly appended to the respective unit, but various deviations
    are tolerated, as follows:
    Inverse units are ideally shown using negative exponents, but a _single_ U+002F "/" "SOLIDUS"
    character (a normal slash) is allowed within the string e.g. `"kg m2 / s"` for the above.
    Similar characters are also checked for:
    U+2044 "⁄" "FRACTION SLASH", U+2215 "∕" "DIVISION SLASH"
    If a slash is used, **all** components to the right of the slash are considered to together
    comprise the divisor, e.g. `"J / kg s"` is treated as `"J kg-1 s-1"`.
    Whitespace either side of the slash is ignored.
    Using more than one slash will throw an error.
    Brackets should _not_ be used within the string; they are completely ignored and do not affect the
    order of operations, e.g. `"(J / kg) s"` is still parsed as `"J kg-1 s-1"`.
    Multiplication is expected primarily as whitespace but can also be represented with dots.
    The set of characters considered whitespace is the same as that described by the docs for
    `str.isspace()`, so many different space characters are tolerated.
    Any of the following dot-like characters may be also be used:
    U+002E "." "FULL STOP", U+00B7 "·" "MIDDLE DOT", U+22C5 "⋅" "DOT OPERATOR",
    U+2022 "•" "BULLET", U+2219 "∙" "BULLET_OPERATOR"
    U+002A "*" "ASTERISK", U+2217 "∗" "ASTERISK OPERATOR"
    In addition, the current value of `quanfig.UNIT_SEPARATOR` is always valid.
    A symbol is not necessary to indicate an exponent, but preceeding an exponent by
    U+005E "^" "CIRCUMFLEX ACCENT" or two asterisks ** is tolerated.
    However, in no case should there be any whitespace between a unit and its exponent.
    The characters U+002D "-" "HYPHEN-MINUS" or U+207B "⁻" "SUPERSCRIPT MINUS" are expected for
    negative exponents, but U+2212 "−" "MINUS SIGN" is also tolerated.
    Fractional exponents may be indicated either:
    * by a superscript numerator, subscript denominator, and a dividing U+2044 "⁄" "FRACTION SLASH"
    e.g. `"⁻¹⁄₂"` - this is the style printed by `quanstants`
    * by normal ASCII integers separated by a normal slash e.g. `"-1/2"` - this is the same as the
    style printed by `str(Fraction(-1, 2))`
    """
    multiplication_chars = [
        ".",
        "·",
        "⋅",
        "•",
        "∙",
        "*",
        "∗",
        quanfig.UNIT_SEPARATOR,
    ]
    division_chars = ["/", "⁄", "∕"]
    exponent_chars = ["^"]
    minus_chars = ["-", "⁻", "−"]
    ignored_chars = ["(", ")", "[", "]", "{", "}"]
    # Prepare string by trimming whitespace
    string = string.strip()
    current_unit = ""
    current_exponent = ""
    terms = []
    divisor_terms = []
    current_terms = terms
    for index, char in enumerate(string):
        # Catch letters early, though there are non-letter characters that are valid for unit symbols
        if char.isalpha():
            current_unit += char
        # Note that isdigit() also returns True for superscript digits
        elif char.isdigit():
            current_exponent += char
        elif char in minus_chars:
            current_exponent += char
        elif (char in multiplication_chars) or char.isspace():
            new_term = _create_term(current_unit, current_exponent)
            if new_term is not None:
                current_terms.append(new_term)
            current_unit = ""
            current_exponent = ""
        elif char in division_chars:
            new_term = _create_term(current_unit, current_exponent)
            if new_term is not None:
                current_terms.append(new_term)
            current_unit = ""
            current_exponent = ""
            # Add subsequent terms to divisor instead
            current_terms = divisor_terms
        elif char in exponent_chars:
            continue
        elif char in ignored_chars:
            continue
        else:
            current_unit += char
    # Make sure final working unit gets parsed too
    final_term = _create_term(current_unit, current_exponent)
    if final_term is not None:
        current_terms.append(final_term)
    # Arithmetic with terms to give final CompoundUnit
    try:
        parsed_unit = terms[0]
    except IndexError:
        raise ParsingError("No units could be successfully parsed.")
    for term in terms[1:]:
        parsed_unit *= term
    if len(divisor_terms) > 0:
        divisor_unit = divisor_terms[0]
        for term in divisor_terms[1:]:
            divisor_unit *= term
        parsed_unit = parsed_unit / divisor_unit
    return parsed_unit

def get_total(request: str, include_prefixed=True, prefixed_only=False):
    """Return the total number of defined names, units, or prefixed units according to the request.
    `request` should be "names", "units", or "prefixed".
    """
    if request == "names":
        return len(list_names(include_prefixed, prefixed_only))
    elif request == "units":
        return len(list_units(include_prefixed, prefixed_only))
