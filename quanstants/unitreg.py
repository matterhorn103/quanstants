class UnitAlreadyDefinedError(Exception):
    pass

# Namespace class to contain all the units, making them useable with unit.m notation
class UnitReg:
    def add(self, name: str, unit):
        """Add a `Unit` object to the registry under the provided name.
        
        This method provides a safe way to add units to the registry.
        Names in the registry's namespace cannot be overwritten in this way, and attempting to add a unit
        under a name that is already defined will raise a `UnitAlreadyDefinedError`.
        If it is necessary to redefine a name, do it by setting the attribute in the normal way i.e.
        `UnitReg().already_assigned_name = new_value`.
        """
        if hasattr(self, name):
            raise UnitAlreadyDefinedError(f"{name} is already defined!")
        setattr(self, name, unit)
    
    def list_names(self, include_prefixed=True, prefixed_only=False):
        """Return a list of all unit names in the namespace, in human-readable format i.e. as strings.
        
        By specifying the appropriate options, prefixed units can be included or filtered out, or only
        prefixed units can be requested.
        Essentially just return the value of `self.__dict__.keys()` but with anything that isn't a unit
        filtered out.
        Note that a) the values returned are variable names as strings, not the `Unit` objects themselves
        and b) a unit is typically listed multiple times under different names, as well as under its symbol
        if it has `canon_symbol=True`.
        """
        filtered_names = {name for name in self.__dict__.keys() if name[0] != "_"}
        prefixed = {name for name in filtered_names if hasattr(getattr(self, name), "prefix")}
        if prefixed_only:
            return list(prefixed)
        elif include_prefixed:
            return list(filtered_names)
        else:
            return list(filtered_names - prefixed)
    
    def list_units(self, include_prefixed=True, prefixed_only=False):
        """Return a list of all `Unit` objects currently in the registry.
        
        By specifying the appropriate options, prefixed units can be included or filtered out, or only
        prefixed units can be requested.
        Unlike `list_names()`, the values are `Unit` objects not strings, and each unit is only listed
        once, regardless of how many names it is registered under in the registry.
        """
        filtered_names = self.list_names(include_prefixed, prefixed_only)
        unique_units = {self.__dict__[name] for name in filtered_names}
        return list(unique_units)

    def search(self, search_string: str) -> dict:
        """Return all unit names in the namespace for which the provided search string is found in its metadata.
        
        The results are returned as a dictionary, with each item being the results of each search method
        separated into lists of exact matches and substring matches.
        For now the function just searches in each unit's symbol and name.
        Searching for "ft" means, for example, that all units with the symbol "ft" will be returned, not
        just the unit found under `UnitReg().ft`.
        """
        symbol_results = {"exact": [], "partial": []}
        name_results = {"exact": [], "partial": []}
        all_names = self.list_names()
        for name in all_names:
            if search_string == name:
                results["name"]["exact"].append(name)
            elif search_string in name:
                results["name"]["partial"].append(name)
            if search_string == getattr(self, name).symbol:
                results["symbol"]["exact"].append(name)
            elif search_string in getattr(self, name).symbol:
                results["symbol"]["partial"].append(name)
        # Order each subset by length of name
        symbol_results = {k: v.sort(key=len) for k, v in symbol_results.items()}
        name_results = {k: v.sort(key=len) for k, v in name_results.items()}
        results = {"symbol": symbol_results, "name": name_results}
        return results

    def match(self, search_string: str):
        """Return the best match for the provided search string."""
        if hasattr(self, search_string):
            return getattr(self, search_string)
        else:
            search = self.search(search_string)
            if len(search["symbol"]["exact"]) > 0:
                return getattr(self, search["symbol"]["exact"][0])
            elif len(search["name"]["exact"]) > 0:
                return getattr(self, search["name"]["exact"][0])
            elif len(search["symbol"]["partial"]) > 0:
                return getattr(self, search["symbol"]["partial"][0])
            elif len(search["name"]["partial"]) > 0:
                return getattr(self, search["name"]["partial"][0])

    def parse(self, string: str):
        """Take a string of unit symbols or names and digits and convert to an appropriate Unit object.
        
        The string should ideally take a form such as "kg m2 s-1", with terms separated by spaces and
        exponents given as ASCII integers directly appended to the respective unit, but various deviations
        are tolerated, as follows:

        Inverse units are ideally shown using negative exponents, but a _single_ U+002F "/" "SOLIDUS"
        character is allowed within the string e.g. "kg m2 / s" for the above.
        Similar characters are also checked for:
        U+2044 "⁄" "FRACTION SLASH", U+2215 "∕" "DIVISION SLASH"
        If a slash is used, **all** components to the right of the slash are considered to together comprise
        the divisor, e.g. "J / kg s" is treated as "J kg-1 s-1".
        Whitespace either side of the slash is ignored.
        Using more than one slash will throw an error.

        Brackets should _not_ be used within the string; they are completely ignored and do not affect the
        order of operations, e.g. "(J / kg) s" is still parsed as "J kg-1 s-1".

        Multiplication of terms can be shown in various ways, but consistency is required.
        Multiplication is expected primarily as whitespace (as implemented by `split()` without an
        argument) so most different space characters are tolerated.
        If whitespace is not found, the parser will attempt to split the terms by other characters until
        a split has been made (or none of the characters were found), in the following order:
        U+002E "." "FULL STOP", U+00B7 "·" "MIDDLE DOT", U+22C5 "⋅" "DOT OPERATOR",
        U+2022 "•" "BULLET", U+2219 "∙" "BULLET_OPERATOR"
        Asterisk characters * must _not_ be used.

        Exponents preceded by U+005E "^" "CIRCUMFLEX ACCENT" or two asterisks ** are tolerated, but in no
        case should there be any whitespace between a unit and its exponent.
        The character U+002D "-" "HYPHEN-MINUS" is expected for negative exponents, but
        U+2212 "−" "MINUS SIGN" is also tolerated.

        """
        # Remove any brackets
        string = string.replace("(", "").replace(")", "")
        # Split into dividend and divisor, if a slash is used
        divided_string = string.split("/").split("⁄").split("∕")
        # Remove any whitespace
        fraction_terms = [term.strip() for term in divided_string]
        # Split multiplied terms
        for fraction_term in fraction_terms:
            split_string = [fraction_term]
            while len(split_string) == 1:
                split_string = fraction_term.split()
                split_string = fraction_term.split(".")
                split_string = fraction_term.split("·")
                split_string = fraction_term.split("⋅")
                split_string = fraction_term.split("•")
                split_string = fraction_term.split("∙")
                break
            
            
    
    def get_total(self, request: str, include_prefixed=True, prefixed_only=False):
        """Return the total number of defined names, units, or prefixed units according to the request.
        
        `request` should be "names", "units", or "prefixed".
        """
        if request == "names":
            return len(self.list_names(include_prefixed, prefixed_only))
        elif request == "units":
            return len(self.list_units(include_prefixed, prefixed_only))
        