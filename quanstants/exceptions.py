"""A central module for all errors that quanstants raises."""


class AlreadyDefinedError(AttributeError):
    pass

class AlreadyPrefixedError(TypeError):
    pass

class IncompleteDimensionsError(ValueError):
    pass

class MismatchedUnitsError(ValueError):
    pass

class NotATemperatureError(ValueError):
    pass

class NotDimensionlessError(ValueError):
    pass

class ParsingError(SyntaxError):
    pass