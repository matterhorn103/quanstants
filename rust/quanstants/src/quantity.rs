use crate::{base::Unit, dimensions::Dimensions};

#[derive(Debug)]
pub struct Quantity {
    number: f64,
    unit: Box<dyn Unit>,
    uncertainty: f64,
}

impl Quantity {
    pub fn new<T: Unit>(number: f64, unit: T, uncertainty: f64) -> Self {
        Self {
            number,
            unit: Box::new(unit),
            uncertainty,
        }
    }
}
