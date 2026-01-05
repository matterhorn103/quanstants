use std::fs;

use quanstants::{
    context::Context,
    defs::{DefFile, QuantityDef, UnitDef},
    fraction::Frac,
    SciDecimal,
};

fn def_from_derived_unit(ctx: &Context, unit_name: &str) -> UnitDef {
    let unit = ctx.units.get_by_name(unit_name).unwrap();
    let unit_factors: Vec<(String, Frac)> = unit
        .defining_factors()
        .iter()
        .map(|factor| (factor.unit.name(), factor.exponent))
        .collect();
    UnitDef {
        id: Some(unit.id),
        base: false,
        symbol: Some(unit.symbol(false)),
        name: unit.name(),
        prefix: None,
        aliases: Vec::new(),
        alt_names: Vec::new(),
        source: None,
        dimensions: None,
        value: Some(QuantityDef {
            number: SciDecimal::ONE,
            uncertainty: Some(SciDecimal::ZERO),
            unit: unit_factors,
        }),
    }
}

fn main() {
    let context = Context::new();
    let mut def_file = DefFile::default();
    let units = vec![
        "radian",
        "steradian",
        "hertz",
        "newton",
        "pascal",
        "joule",
        "watt",
        "coulomb",
        "volt",
        "farad",
        "ohm",
        "siemens",
        "weber",
        "tesla",
        "henry",
        "Celsius degree",
        "lumen",
        "lux",
        "becquerel",
        "gray",
        "sievert",
        "katal",
    ];
    for u in units {
        let unit_def = def_from_derived_unit(&context, u);
        def_file.units.insert(u.into(), unit_def);
    }
    let toml_string = toml::to_string(&def_file).unwrap();
    println!("{toml_string}");
    fs::write("output.toml", &toml_string).unwrap();
}
