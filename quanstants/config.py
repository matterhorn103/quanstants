
class QuanstantsConfig:
    ROUNDING_MODE = "ROUND_HALF_UP"         # For other options see Decimal module
    ROUND_TO = "PLACES"                     # Controls the default behaviour of round(Quantity) (not Quantity.round())
    ROUND_PAD = True                        # Like siunitx, determines if extra zeroes should be added to reach desired sigfigs
    CONVERT_FLOAT_AS_STR = True             # True or False
    UNIT_SEPARATOR = " "                    # Any string
    INVERSE_UNIT = "NEGATIVE_SUPERSCRIPT"   # Other option is "SLASH"
    UNICODE_SUPERSCRIPTS = True             # True or False