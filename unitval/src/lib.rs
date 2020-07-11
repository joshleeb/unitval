pub use unitval_derive::UnitVal;

use std::io;

pub trait AsUnitVal {
    fn as_unitval(&self) -> &'static str;
}

pub trait FromUnitVal: Sized {
    fn from_unitval(value: &str) -> io::Result<Self>;
}
