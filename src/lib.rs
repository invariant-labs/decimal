pub mod decimal;
pub mod operations;
pub mod traits;
pub mod uint;

pub use crate::uint::U256;

use decimal_core::decimal;

#[decimal(12)]
struct D(u8);

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn flow() {
        let d = D(42);

        assert_eq!(d.get_scale(), 12);
        assert_eq!(d.get_value(), 42);
    }
}
