pub mod decimal;
pub mod operations;
pub mod traits;
pub mod uint;

pub use crate::uint::U256;

use decimal_core::decimal;

use std::convert::TryInto;

trait Decimal<T> {
    fn get_scale(&self) -> u8;
    fn get_value(&self) -> T;
}

fn universal_into<Y, T: TryInto<Y>>(a: T) -> Y {
    match a.try_into() {
        Ok(v) => v,
        Err(_) => panic!("could not parse {} to {}", "T", "u8"),
    }
}

#[decimal(12)]
struct N(u8);

#[decimal(12)]
struct K {
    v: u8,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn tr() {
        assert_eq!(universal_into::<u8, u16>(255), 255);
        assert_eq!(universal_into::<u8, u8>(255), 255);
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

        assert_eq!(d.get_scale(), 12);
        assert_eq!(d.get_value(), 42);

        let t = K { v: 42 };

        assert_eq!(t.get_scale(), 12);
        assert_eq!(t.get_value(), 42);
    }
}
