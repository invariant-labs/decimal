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

#[decimal(3, u128)]
#[derive(Default, Debug, PartialEq)]
struct N(u32);

#[decimal(2, u128)]
#[derive(Default, Debug, PartialEq)]
struct K {
    v: u32,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        let a = K::new(1);
        let b = N::new(1000);
        assert_eq!(a * b, K::new(1));
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
        assert_eq!(N::one::<u128>(), 1000);
    }

    #[test]
    fn test_mull() {
        let n = N(200);
        let k = K { v: 300 };
        assert_eq!(
            (n.get().checked_mul(k.here()).unwrap())
                .checked_div(K::one())
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

        assert_eq!(d.scale(), 3);
        assert_eq!(d.get(), 42);

        let t = K { v: 42 };

        assert_eq!(t.scale(), 2);
        assert_eq!(t.get(), 42);
    }
}
