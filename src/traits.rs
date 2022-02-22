use std::fmt::Debug;

pub trait Decimal {
    type U: Debug + Default;

    fn get(&self) -> Self::U;
    fn new(value: Self::U) -> Self;
    fn here<Y: TryFrom<Self::U>>(&self) -> Y;
    fn scale() -> u8;
    fn one<T: TryFrom<u128>>() -> T;
    fn almost_one<T: TryFrom<u128>>() -> T;
}

pub trait BigOps<T> {
    fn big_mul(self, rhs: T) -> Self;
    fn big_mul_up(self, rhs: T) -> Self;
    fn big_div(self, rhs: T) -> Self;
    fn big_div_up(self, rhs: T) -> Self;
}

pub trait Others<T> {
    fn mul_up(self, rhs: T) -> Self;
    // fn div_up(self, rhs: T) -> Self;
    // fn pow(self, rhs: T) -> Self;
}

pub trait Factories<T> {
    fn from_integer(integer: T) -> Self;
    fn from_scale(integer: T, scale: u8) -> Self;
    fn from_scale_up(integer: T, scale: u8) -> Self;
}

pub trait BetweenDecimals<T> {
    fn from_decimal(other: T) -> Self;
    fn from_decimal_up(other: T) -> Self;
}
