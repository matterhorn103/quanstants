use num_traits;

pub trait Numeric:
    num_traits::Num + num_traits::NumOps + std::fmt::Display
{}

impl<T> Numeric for T 
where 
    T: num_traits::Num + num_traits::NumOps + std::fmt::Display
{}
