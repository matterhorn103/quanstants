from ..unit import DerivedUnit
from ..quantity import Quantity
from .base import *
from .si import litre

# fmt: off

# Import those units which are common to both imperial and US customary systems in-line

# Length
from .imperial import twip
mil = DerivedUnit(None, "mil", Quantity("0.0000254", m)) # Identical to imperial.thou but different name
from .imperial import inch

from .imperial import foot, yard
rod = DerivedUnit("rd", "rod", Quantity("5.0292", m), alt_names=["pole", "perch"])
from .imperial import chain, furlong, mile, league

# Area
from .imperial import acre

# US survey units - the above but defined using the US survey foot. Use NIST's 2023 approximations
us_survey_inch = DerivedUnit("in", "us_survey_inch", Quantity("0.0254000508001", m))    # Not provided by NIST but definition is 1/39.37 m, so provide to 12 sigfigs
us_survey_link = DerivedUnit("li", "us_survey_link", Quantity("0.201168402", m))
us_survey_foot = DerivedUnit("ft", "us_survey_foot", Quantity("0.304800609601", m))
us_survey_fathom = DerivedUnit("ftm", "us_survey_fathom", Quantity("1.828803658", m))
us_survey_rod = DerivedUnit("rd", "us_survey_rod", Quantity("5.029210058", m), alt_names=["us_survey_pole", "us_survey_perch"])
us_survey_chain = DerivedUnit("ch", "us_survey_chain", Quantity("20.116840234", m))
us_survey_furlong = DerivedUnit("fur", "us_survey_furlong", Quantity("201.168402337", m))
us_survey_cable = DerivedUnit(None, "us_survey_cable", Quantity("219.456438913", m), alt_names=["us_survey_cables_length"])
us_survey_mile = DerivedUnit("mi", "us_survey_mile", Quantity("1609.347218694", m), alt_names=["statute_mile"])
us_survey_league = DerivedUnit("lea", "us_survey_league", Quantity("4828.041656083", m))
us_survey_acre = DerivedUnit("ac", "us_survey_acre", Quantity("4046.872609874", m**2))
us_survey_section = DerivedUnit(None, "us_survey_section", Quantity("2589998.47", m**2))

# Capacity
us_fluid_ounce = DerivedUnit("fl oz", "us_fluid_ounce", Quantity("0.0295735295625", litre), canon_symbol=True)

# Nautical units
us_fathom = DerivedUnit("ftm", "us_fathom", Quantity("1.8288", m))
us_cable = DerivedUnit(None, "us_cable", Quantity("219.456", m))
from .imperial import nautical_mile
from .imperial import knot

# Temperature
from .temperatures import degreeFahrenheit

# fmt: on
