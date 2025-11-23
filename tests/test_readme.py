import decimal
from decimal import Decimal as dec # noqa: F401
from fractions import Fraction as frac # noqa: F401

from quanstants import qu  # type: ignore # noqa: F401


class TestREADME:
    """Here we parse the README and test each example statement automatically.
    
    A single method here tests all examples in a subsection of "Usage", with multiple
    assert statements per test.
    The results are thus not very helpful in the event of a failure, so it is still
    necessary to add manual tests below for diagnostics.
    However, for the common case that an example is added and a corresponding test
    is forgotten, it will still be tested by this automatic test.
    In this way, this test class provides a guarantee 
    """
    with open("README.md", "r", encoding="utf-8") as f:
        readme = [line.rstrip() for line in f.readlines()]
    subsections = {}
    current_subsection = None
    parsing_usage = False
    for line in readme:
        if line[:3] == "## ":
            if line == "## Usage":
                parsing_usage = True
            else:
                parsing_usage = False
        elif line[:4] == "### ":
            current_subsection = line[4:]
            subsections[current_subsection] = list()
        elif parsing_usage and current_subsection is not None:
            subsections[current_subsection].append(line)
    
    def run_subsection(self, subsection):
        # Go through markdown, ignore lines outside of code blocks
        # Execute lines in code blocks if they are input statements
        # If a line is an output statement, check that it is the result of
        # executing the previous line
        in_code_block = False
        last_executed = ""
        for n, line in enumerate(subsection):
            if line == "```python":
                in_code_block = True
            elif line == "```":
                in_code_block = False
            elif in_code_block:
                if "Error" in line.split()[0]:
                    continue
                elif "Error" in subsection[n+1].split()[0]:
                    continue
                elif line[:4] == ">>> ":
                    print(f"Executing: {line}")
                    last_executed = line[4:]
                    exec(last_executed)
                else:
                    print(f"Supposed result: {line}")
                    if last_executed.startswith("print("):
                        print("(Swapped `print()` for `str()`)")
                        last_executed = last_executed.replace("print(", "str(")
                        assert eval(last_executed) == line
                    else:
                        assert repr(eval(last_executed)) == line
            else:
                continue
    
    def test_quantity_creation(self):
        self.run_subsection(self.subsections["Quantity creation"])
    
    def test_units_and_prefixes(self):
        self.run_subsection(self.subsections["Units and prefixes"])
    
    def test_uncertainties(self):
        self.run_subsection(self.subsections["Uncertainties"])
    
    def test_parsing(self):
        self.run_subsection(self.subsections["Parsing strings"])
    
    def test_arithmetic(self):
        self.run_subsection(self.subsections["Arithmetic"])
    
    def test_conversion(self):
        self.run_subsection(self.subsections["Conversion"])
    
    def test_equalities(self):
        self.run_subsection(self.subsections["Equalities"])
    
    def test_constants(self):
        self.run_subsection(self.subsections["Constants"])
    
    def test_rounding(self):
        self.run_subsection(self.subsections["Rounding"])
        # Reset decimal rounding behaviour
        decimal.getcontext().rounding = decimal.ROUND_HALF_EVEN
    
    def test_temperatures(self):
        self.run_subsection(self.subsections["Temperatures"])
    
    def test_logs(self):
        self.run_subsection(self.subsections["Logarithmic scales"])
    
    # Don't test the config section as the tomls make it complicated
