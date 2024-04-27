import sys

def parse(file):
    output = []
    with open(file) as f:
        lines = f.read().splitlines()
    lines = [line for line in lines if len(line) > 0]
    lines = [line for line in lines if line[0].isalpha()]
    for line in lines:
        parts = [part.strip() for part in line.split("  ")]
        parts = [part for part in parts if len(part) > 0]
        name = "_".join(parts[0].split()).replace("-", "_").replace("/", "_in_").replace(".", "").replace("(", "").replace(")", "")
        number = "".join(parts[1].split())
        uncertainty = "".join(parts[2].split())
        if len(parts) > 3:
            unit_raw = [f"u.{x}".replace("^", "**") for x in parts[3].split()]
            unit = " * ".join(unit_raw)
        else:
            unit = "unitless"
        definition = f'{name} = Constant(None, "{name}", "{number}", {unit}, uncertainty="{uncertainty}", canon_symbol=False)\n'
        output.append(definition)
    print(output)
    return output


if __name__ == "__main__":
    input = sys.argv[1]
    output = parse(input)
    output_name = f"{input}.py"
    with open(output_name, "w") as f:
        f.writelines(output)