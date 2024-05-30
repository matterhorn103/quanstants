"""A central module for all errors that quanstants raises."""

class QuanstantsError(Exception):
    """Superclass from which all errors that quanstants raises are derived."""
    pass

class AlreadyDefinedError(QuanstantsError):
    pass

class AlreadyPrefixedError(QuanstantsError):
    pass

class MismatchedUnitsError(QuanstantsError):
    pass

class NotATemperatureError(QuanstantsError):
    pass

class NotDimensionlessError(QuanstantsError):
    pass

class ParsingError(QuanstantsError):
    pass