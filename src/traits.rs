use std::fmt::Debug;

pub trait Decimal {
    type U: Debug + Default;

    fn scale(&self) -> u8;
    fn get(&self) -> Self::U;
    fn new(value: Self::U) -> Self;
    fn here<Y: TryFrom<Self::U>>(&self) -> Y;
    fn one<T: TryFrom<u128>>() -> T;
    fn almost_one<T: TryFrom<u128>>() -> T;
}

pub trait BigOps<T> {
    fn big_mul(self, rhs: T) -> Self;
    fn big_mul_up(self, rhs: T) -> Self;
    fn big_div(self, rhs: T) -> Self;
    fn big_div_up(self, rhs: T) -> Self;
}

pub trait Factories<T> {
    fn from_integer(integer: T) -> Self;
    fn from_decimal(integer: T, scale: u8) -> Self;
}
