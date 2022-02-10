pub mod decimal;
pub mod operations;
pub mod traits;
pub mod uint;

pub use crate::uint::U256;

use decimal_core::decimal;

use std::convert::TryInto;

use std::ops::Add;

pub trait Decimal<T> {
    fn get_scale(&self) -> u8;
    fn get_value(&self) -> T;
    fn get_one<Y: TryFrom<u128>>(&self) -> Y;
}

fn universal_into<Y, T: TryInto<Y>>(a: T) -> Y {
    match a.try_into() {
        Ok(v) => v,
        Err(_) => panic!("could not parse {} to {}", "T", "u8"),
    }
}

#[decimal(3)]
#[derive(Default, Debug, PartialEq)]
struct N(u32);

#[decimal(2)]
#[derive(Default, Debug, PartialEq)]
struct K {
    v: u32,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn tr() {
        assert_eq!(universal_into::<u8, u16>(255), 255);
        assert_eq!(universal_into::<u8, u8>(255), 255);
    }

    #[test]
    fn test_add() {
        let a = N(1);
        let b = N(1);
        assert_eq!(a + b, N(2));
    }

    #[test]
    fn default() {
        assert_eq!(N::new(1), N(1));
    }

    #[test]
    fn test_get_one() {
        let n = N(0);
        assert_eq!(n.get_one::<u128>(), 1000);
    }

    #[test]
    fn test_mull() {
        let n = N(200);
        let k = K { v: 300 };
        assert_eq!(
            (n.get_value().checked_mul(k.here()).unwrap())
                .checked_div(k.get_one())
                .unwrap(),
            600
        );
    }

    #[test]
    pub fn test_from() {
        let n = N(0);
        let k: u8 = n.here();
        assert_eq!(k, 0)
    }

    #[test]
    pub fn flow() {
        let d = N(42);

        assert_eq!(d.get_scale(), 3);
        assert_eq!(d.get_value(), 42);

        let t = K { v: 42 };

        assert_eq!(t.get_scale(), 2);
        assert_eq!(t.get_value(), 42);
    }
}
