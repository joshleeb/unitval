pub trait AsUnitVal<T> {
    fn as_unitval(&self) -> T;
}

pub trait FromUnitVal<T> {
    fn from_unitval(value: T) -> Self;
}
