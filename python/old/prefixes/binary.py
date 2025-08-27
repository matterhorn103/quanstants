"""Binary prefixes from the 1999 IEC standard."""

from ..prefix import Prefix

# fmt: off

# Binary prefixes
kibi = Prefix("Ki", "kibi", 1024**1)
mebi = Prefix("Mi", "mebi", 1024**2)
gibi = Prefix("Gi", "gibi", 1024**3)
tebi = Prefix("Ti", "tebi", 1024**4)
pebi = Prefix("Pi", "pebi", 1024**5)
exbi = Prefix("Ei", "exbi", 1024**6)
zebi = Prefix("Zi", "zebi", 1024**7)
yobi = Prefix("Yi", "yobi", 1024**8)

# fmt: on
