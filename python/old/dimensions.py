from .exceptions import IncompleteDimensionsError
from .unicode import generate_superscript

# Tried using Counters for this but addition via dict comprehension was much faster
# (286 ns vs 1.8 µs) as was an equality (36 ns vs 960 ns)
# Sadly UserDict is also slow (755 ns and 3 µs respectively)

# With dict comprehensions and the set of dimensions:
# add takes 518 ns,
# mul takes 439 ns,
# eq takes 35 ns

#class DimensionalExponents:
#    def __init__(self, *args, **kwargs):
#        self.dims = dict(*args, **kwargs)
#    
#    def __add__(self, other):
#        # 894 ns
#        self.dims = {d: self.dims[d] + other.dims[d] for d in dimensions}
#        return self
#
#    def __mul__(self, other):
#        # 5.4 µs
#        self.dims = {d: self.dims[d] * other for d in dimensions}
#        return self
    
class Dimensions(dict):
    """Dictionary holding the dimensional exponents of a unit or quantity.
    
    Simply a subclass of `dict` with addition/subtraction/multiplication operations
    defined so that it acts like a faster version of `collections.Counter`.

    The keys and values can be provided by any of the normal methods as for `dict`, but
    if no arguments are passed at all, a dimensionless `Dimensions` will be returned
    with all values as 0.

    The keys of the dict are the seven SI dimensions by their symbol (so length is "L",
    time is "T" etc. - note that the key for temperature is the Unicode theta "Θ").

    The values of the dict are exponents of type `int` or `Fraction`, and can be zero
    and negative as well as positive.

    If all values are 0, the dict represents a dimensionless unit or quantity.

    Very little of `dict`'s functionality has been overridden, so `Dimensions` usually
    behaves exactly the same. Amongst other things this means that an instance of
    `Dimensions` compares equal to a normal `dict` with the same keys and values.
    """

    # Pre-generated list to make iteration fast
    _dimensions_list = ["L", "M", "T", "I", "Θ", "N", "J"]

    # Empty dict for fast comparisons and fast instantiation of empty instance
    _dimensionless = {"L": 0, "M": 0, "T": 0, "I": 0, "Θ": 0, "N": 0, "J": 0}

    # In an ideal world both of the above would be read-only but since chaining
    # @property and @classmethod has been deprecated making them semi-private with
    # underscores will have to do

    def __init__(self, *args, **kwargs):

        # If no positional arguments are passed, create a new dimensionless Dimensions
        # then add anything specified as kwargs
        if len(args) == 0:
            super().__init__(Dimensions._dimensionless, **kwargs)
        # Otherwise the passed iterable needs to have all 7 dimensions
        elif len(args[0]) == 7:
            super().__init__(*args, **kwargs)
        else:
            raise IncompleteDimensionsError("If a mapping or iterable is provided, all seven base dimensions must be specified.")
    
    def __str__(self) -> str:
        """Return the dimension as a nice string."""
        if self == Dimensions._dimensionless:
            return "(dimensionless)"
        else:
            result = ""
            for dimension, exponent in self.items():
                if exponent != 0:
                    result += dimension
                    if exponent != 1:
                        result += generate_superscript(exponent)
            return result

    def __add__(self, other):
        # 880 ns
        for d in Dimensions._dimensions_list:
            self[d] = self[d] + other[d]
        return self
        # 1.3 µs
        #return Dimensions({k: self[k] + other[k] for k in dimensions_list})

    def __sub__(self, other):
        for d in Dimensions._dimensions_list:
            self[d] = self[d] - other[d]
        return self
    
    def __mul__(self, other):
        # 5.8 µs
        #if other == 0:
        #    return Dimensions()
        #elif isinstance(other, int):
        #    orig = self.copy()
        #    if other > 0:
        #        for i in range(other - 1):
        #            self += orig
        #    else:
        #        for i in range(other - 1):
        #            self -= orig
        #    return self
        # 5.3 µs!
        #for d in dimensions_list:
        #    self[d] = self[d] * other
        #return self
        # 1.3 µs, no idea why
        return Dimensions({d: self[d] * other for d in Dimensions._dimensions_list})


# Function to turn a tuple or other iterable of factors into a Dimensions dict
def generate_dimensions(
        components: tuple[tuple, ...] | None = None,
        units: tuple | None = None,
    ) -> Dimensions:
    new_dimensions = Dimensions()
    if components:
        for unit, exponent in components:
            if exponent == 1:
                new_dimensions += unit.dimensions
            elif exponent == -1:
                new_dimensions -= unit.dimensions
            elif exponent == 0:
                continue
            else:
                new_dimensions += (unit.dimensions * exponent)
    elif units:
        for unit in units:
            new_dimensions += unit.dimensions
    #for unit, exponent in components:
    #    for d in new_dimensions.keys():
    #        if d in unit.dimensions:
    #            new_dimensions[d] += (
    #                unit.dimensions[d] * exponent
    #            )
    return new_dimensions