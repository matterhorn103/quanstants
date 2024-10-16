struct Value<'a>(f64, &'a BaseUnit);

trait Val {
    fn value(&self) -> Value;
}

pub enum BaseUnit {
    Metre,
    Second,
    Kilogram,
}

impl Val for BaseUnit {
    fn value(&self) -> Value {
        Value(1.0, &self)
    }
}

pub struct Unit<'a> {
    pub symbol: String,
    pub name: String,
    pub def: &'a BaseUnit,
}

impl Val for Unit<'_> {
    fn value(&self) -> Value {
        Value(1.0, self.def)
    }
}
