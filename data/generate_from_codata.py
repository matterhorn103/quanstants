"""A simple script to turn NIST's text files of constant definitions into Constant definitions.

NIST publish the CODATA recommended values of fundamental physical constants at:
https://physics.nist.gov/cuu/Constants/Table/allascii.txt
as a simple ASCII text file.
This script roughly parses it and creates Python code defining Constant objects. The
parsing is not perfect, so manual tweaks to the output are both expected and necessary.
"""

import sys


def parse(file):
    # Add imports to begin with
    output = [
        "from ..constant import Constant\n",
        "from ..unit import unit_reg as qu\n",
        "from ..prefix import prefix_reg as qp\n",
        "\n",
        "# fmt: off\n",
        "\n",
    ]
    with open(file) as f:
        lines = f.read().splitlines()
    lines = [line for line in lines if len(line) > 0]
    lines = [line for line in lines if line[0].isalpha()]
    for line in lines:
        parts = [part.strip() for part in line.split("  ")]
        parts = [part for part in parts if len(part) > 0]
        name = (
            "_".join(parts[0].split())
            .replace("-", "_")
            .replace("/", "_in_")
            .replace(".", "")
            .replace("(", "")
            .replace(")", "")
        )
        number = "".join(parts[1].split())
        uncertainty = "".join(parts[2].split())
        if len(parts) > 3:
            unit_raw = [f"qu.{x}".replace("^", "**") for x in parts[3].split()]
            unit = " * ".join(unit_raw)
        else:
            unit = "qu.unitless"
        # eV is the only commonly prefixed unit
        if "eV" in unit and unit[:5] != "qu.eV":
            unit = "(qp." + unit[3] + " * " + "qu.eV)" + unit[6:]
        definition = f'{name} = Constant(None, "{name}", "{number}", {unit}, {uncertainty}, canon_symbol=False)\n'
        output.append(definition)
    output.append("\n")
    output.append("# fmt: on\n")
    print(output)
    return output


if __name__ == "__main__":
    input = sys.argv[1]
    output = parse(input)
    output_name = f"{input}.py"
    with open(output_name, "w") as f:
        f.writelines(output)
