#[doc(hidden)]
pub use unitval_derive::*;

use std::{io, iter::FromIterator};

pub trait AsUnitVal {
    fn as_unitval(&self) -> &'static str;
}

pub fn collect_as<T, B>(value: &[T]) -> B
where
    T: AsUnitVal,
    B: FromIterator<&'static str>,
{
    value.into_iter().map(|x| x.as_unitval()).collect()
}

pub trait FromUnitVal: Sized {
    fn from_unitval(value: &str) -> io::Result<Self>;
}

pub fn collect_from<T, I, B>(iter: I) -> io::Result<B>
where
    T: FromUnitVal,
    I: IntoIterator,
    I::Item: AsRef<str>,
    B: FromIterator<T>,
{
    iter.into_iter().map(|x| FromUnitVal::from_unitval(x.as_ref())).collect()
}
