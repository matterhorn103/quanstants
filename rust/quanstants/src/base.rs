use crate::dimensions::Dimensions;

pub trait Unit: std::fmt::Debug {
    fn symbol(&self) -> &str;

    fn name(&self) -> &str;

    //fn alt_names(&self) -> &[&str];

    fn preceding_space(&self) -> bool;

    fn dimensions(&self) -> Dimensions;
}

/// SI base units and other base units that are not defined in terms of other units.
///
/// The key property of a `BaseUnit` is that a request to express it in base units, to
/// cancel it, or to express it canonically, simply returns a `Quantity` of unity times
/// the `BaseUnit`.
#[derive(Debug)]
pub struct BaseUnit {
    symbol: &'static str,
    name: &'static str,
    //alt_names: Vec<&'a str>,
    dimensions: Dimensions,
}

impl Unit for BaseUnit {
    fn symbol(&self) -> &str {
        self.symbol
    }
    
    fn name(&self) -> &str {
        self.name
    }
    
    //fn alt_names(&self) -> &[&str] {
    //    &self.alt_names
    //}
    
    fn preceding_space(&self) -> bool {
        todo!()
    }

    fn dimensions(&self) -> Dimensions {
        self.dimensions
    }
}

static METRE: BaseUnit = BaseUnit {
    symbol: "m",
    name: "metre",
    //alt_names: vec!["meter"],
    dimensions: Dimensions {L: 1, M: 0, T: 0, I: 0, Θ: 0, N: 0, J: 0 },
};

static SECOND: BaseUnit = BaseUnit {
    symbol: "s",
    name: "second",
    //alt_names: vec![],
    dimensions: Dimensions {L: 0, M: 1, T: 0, I: 0, Θ: 0, N: 0, J: 0 },
};

static KILOGRAM: BaseUnit = BaseUnit {
    symbol: "kg",
    name: "kilogram",
    //alt_names: vec!["kilo"],
    dimensions: Dimensions {L: 0, M: 0, T: 1, I: 0, Θ: 0, N: 0, J: 0 },
};

static AMPERE: BaseUnit = BaseUnit {
    symbol: "A",
    name: "ampere",
    //alt_names: vec!["amp"],
    dimensions: Dimensions {L: 0, M: 0, T: 0, I: 1, Θ: 0, N: 0, J: 0 },
};

static MOLE: BaseUnit = BaseUnit {
    symbol: "mol",
    name: "mole",
    //alt_names: vec![],
    dimensions: Dimensions {L: 0, M: 0, T: 0, I: 0, Θ: 0, N: 1, J: 0 },
};

static JOULE: BaseUnit = BaseUnit {
    symbol: "J",
    name: "joule",
    //alt_names: vec![],
    dimensions: Dimensions {L: 0, M: 0, T: 0, I: 0, Θ: 0, N: 0, J: 1 },
};
