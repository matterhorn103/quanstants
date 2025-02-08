use crate::{unit::UnitType, Dimensions, UnitRegistry};

pub enum UnitSet {
    Unitless,
    Base,
    SI,
}

pub fn add_set(registry: &mut UnitRegistry, set: UnitSet) {
    match set {
        UnitSet::Unitless => add_unitless(registry),
        UnitSet::Base => add_base(registry),
        UnitSet::SI => todo!(),
    }
}

fn add_unitless(registry: &mut UnitRegistry) {
    registry.add_unit(
        Some(0),
        UnitType::Unitless,
        String::from("(unitless)"),
        String::from("unitless"),
        vec![],
        Dimensions {L: 0, M: 0, T: 0, I: 0, Θ: 0, N: 0, J: 0 },
    );
}

fn add_base(registry: &mut UnitRegistry) {
    registry.add_unit(
        Some(1),
        UnitType::Base,
        String::from("m"),
        String::from("metre"),
        vec![String::from("meter")],
        Dimensions {L: 1, M: 0, T: 0, I: 0, Θ: 0, N: 0, J: 0 },
    );
    registry.add_unit(
        Some(1),
        UnitType::Base,
        String::from("kg"),
        String::from("kilogram"),
        vec![String::from("kilo")],
        Dimensions {L: 0, M: 1, T: 0, I: 0, Θ: 0, N: 0, J: 0 },
    );
    registry.add_unit(
        Some(3),
        UnitType::Base,
        String::from("s"),
        String::from("second"),
        vec![],
        Dimensions {L: 0, M: 0, T: 1, I: 0, Θ: 0, N: 0, J: 0 },
    );
    registry.add_unit(
        Some(4),
        UnitType::Base,
        String::from("A"),
        String::from("ampere"),
        vec![String::from("amp")],
        Dimensions {L: 0, M: 0, T: 0, I: 1, Θ: 0, N: 0, J: 0 },
    );
    // TODO Add kelvin
    registry.add_unit(
        Some(6),
        UnitType::Base,
        String::from("mol"),
        String::from("mole"),
        vec![],
        Dimensions {L: 0, M: 0, T: 0, I: 0, Θ: 0, N: 1, J: 0 },
    );
    registry.add_unit(
        Some(7),
        UnitType::Base,
        String::from("J"),
        String::from("joule"),
        vec![],
        Dimensions {L: 0, M: 0, T: 0, I: 0, Θ: 0, N: 0, J: 1 },
    );
}
