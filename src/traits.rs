use core::fmt::{Debug, Display};

use alloc::string::String;

pub trait Decimal {
    type U: Debug + Default;

    fn get(&self) -> Self::U;
    fn new(value: Self::U) -> Self;
    fn max_instance() -> Self;
    fn max_value() -> Self::U;
    fn here<Y: TryFrom<Self::U>>(&self) -> Y;
    fn scale() -> u8;
    fn one<T: TryFrom<u128>>() -> T;
    fn checked_one<T: TryFrom<u128>>() -> Result<T, String>
    where
        T::Error: Display;
    fn almost_one<T: TryFrom<u128>>() -> T;
}

pub trait BigOps<T>: Sized {
    fn big_mul(self, rhs: T) -> Self;
    fn big_mul_up(self, rhs: T) -> Self;
    fn big_div(self, rhs: T) -> Self;
    fn checked_big_div(self, rhs: T) -> Result<Self, String>;
    fn big_div_up(self, rhs: T) -> Self;
}

pub trait Others<T> {
    fn mul_up(self, rhs: T) -> Self;
    fn div_up(self, rhs: T) -> Self;
}

pub trait OthersSameType {
    fn sub_abs(self, rhs: Self) -> Self;
}

pub trait Factories<T>: Sized {
    fn from_integer(integer: T) -> Self;
    fn from_scale(integer: T, scale: u8) -> Self;
    fn checked_from_scale(integer: T, scale: u8) -> Result<Self, String>;
    fn from_scale_up(integer: T, scale: u8) -> Self;
}

pub trait BetweenDecimals<T>: Sized {
    fn from_decimal(other: T) -> Self;
    fn checked_from_decimal(other: T) -> Result<Self, String>;
    fn from_decimal_up(other: T) -> Self;
}

pub trait ToValue<T, B> {
    fn big_mul_to_value(self, value: T) -> B;
    fn big_mul_to_value_up(self, value: T) -> B;
}

pub trait FactoriesToValue<T, B> {
    fn checked_from_scale_to_value(integer: T, scale: u8) -> Result<B, String>;
}

pub trait BetweenDecimalsToValue<T, B> {
    fn checked_from_decimal_to_value(other: T) -> Result<B, String>;
}

pub trait ByNumber<B>: Sized {
    fn big_div_by_number(self, number: B) -> Self;
    fn big_div_by_number_up(self, number: B) -> Self;
    fn checked_big_div_by_number(self, number: B) -> Result<Self, String>;
    fn checked_big_div_by_number_up(self, number: B) -> Result<Self, String>;
}

pub trait CheckedOps: Sized {
    fn checked_add(self, rhs: Self) -> Result<Self, String>;
    fn checked_sub(self, rhs: Self) -> Result<Self, String>;
    fn checked_div(self, rhs: Self) -> Result<Self, String>;
}
