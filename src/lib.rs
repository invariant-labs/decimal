pub mod decimal;
pub mod operations;
pub mod traits;
pub mod uint;

pub use crate::uint::U256;

use decimal_core::decimal;

use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
    panic,
};

use traits::*;

// fn universal_into<Y, T: TryInto<Y>>(a: T) -> Y {
//     match a.try_into() {
//         Ok(v) => v,
//         Err(_) => panic!("could not parse {} to {}", "T", "u8"),
//     }
// }

#[cfg(test)]
#[decimal(3, u128)]
#[derive(Default, Debug, PartialEq, Clone, Copy)]
struct R(u32);

#[cfg(test)]
#[decimal(1)]
#[derive(Default, Debug, PartialEq, Clone, Copy)]
struct Q {
    v: u16,
}

#[cfg(test)]
#[decimal(0)]
#[derive(Default, Debug, PartialEq, Clone, Copy)]
struct N(u8);

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        assert_eq!(N(0) + N(0), N::new(0));
        assert_eq!(N(1) + N(2), N::new(3));
        assert_eq!(R(0) + R(0), R::new(0));
        assert_eq!(R(1) + R(2), R::new(3));

        assert_eq!(N(0) - N(0), N::new(0));
        assert_eq!(N(2) - N(1), N::new(1));
        assert_eq!(R(0) - R(0), R::new(0));
        assert_eq!(R(2) - R(1), R::new(1));

        assert_eq!(N(0) * N(0), N::new(0));
        assert_eq!(N(2) * N::from_integer(1), N::new(2));
        assert_eq!(R(0) * Q::new(0), R::new(0));
        assert_eq!(R(2) * Q::from_integer(1), R::new(2));

        assert_eq!(N(0) / N(1), N::new(0));
        assert_eq!(N(4) / N::from_integer(2), N::new(2));
        assert_eq!(R(0) / Q::new(1), R::new(0));
        assert_eq!(R(4) / Q::from_integer(2), R::new(2));
    }

    #[test]
    fn test_big_ops() {
        // precision
        {
            let a = Q::from_integer(1);
            let b = R::from_integer(1);
            let d = a.big_mul(b);
            let u = a.big_mul_up(b);
            assert_eq!(d, Q::from_integer(1));
            assert_eq!(u, Q::from_integer(1));
        }
        // simple
        {
            let a = Q::from_integer(2);
            let b = R::from_integer(3);
            let d = a.big_mul(b);
            let u = a.big_mul_up(b);
            assert_eq!(d, Q::from_integer(6));
            assert_eq!(u, Q::from_integer(6));
        }
        // big
        {
            let a = Q::new(2u16.pow(15));
            let b = N::from_integer(1);
            let d = a.big_mul(b);
            let u = a.big_mul_up(b);

            let expected = Q::new(2u16.pow(15));
            assert_eq!(d, expected);
            assert_eq!(u, expected);
        }
        // random
        {
            let a = R::new(879132 * 9383);
            let b = Q::new(9383);
            let d = a.big_mul(b);
            let u = a.big_mul_up(b);

            let expected = R(824889555);
            assert_eq!(d, expected);
            assert_eq!(u, expected + R(1));
        }
    }
}
