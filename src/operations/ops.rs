// use crate::decimal::{Decimal, DENOMINATOR};
// use std::ops::{Add, Div, Mul, Sub};

// impl Add for Decimal {
//     type Output = Decimal;
//     fn add(self, other: Decimal) -> Decimal {
//         Decimal::new(self.v.checked_add(other.v).unwrap())
//     }
// }

// impl Sub for Decimal {
//     type Output = Decimal;
//     fn sub(self, other: Decimal) -> Decimal {
//         Decimal::new(self.v.checked_sub(other.v).unwrap())
//     }
// }

// impl Mul for Decimal {
//     type Output = Decimal;
//     fn mul(self, other: Decimal) -> Decimal {
//         Decimal::new(
//             self.v
//                 .checked_mul(other.v)
//                 .unwrap()
//                 .checked_div(DENOMINATOR)
//                 .unwrap(),
//         )
//     }
// }

// impl Div for Decimal {
//     type Output = Decimal;
//     fn div(self, other: Decimal) -> Decimal {
//         Decimal::new(
//             self.v
//                 .checked_mul(DENOMINATOR)
//                 .unwrap()
//                 .checked_div(other.v)
//                 .unwrap(),
//         )
//     }
// }

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn test_ops() {
//         assert_eq!(
//             Decimal::from_integer(0) + Decimal::from_integer(2),
//             Decimal::from_integer(2)
//         );
//         assert_eq!(
//             Decimal::from_integer(2) - Decimal::from_integer(1),
//             Decimal::from_integer(1)
//         );
//         assert_eq!(
//             Decimal::from_integer(2) * Decimal::from_integer(2),
//             Decimal::from_integer(4)
//         );
//         assert_eq!(
//             Decimal::from_integer(111) / Decimal::from_integer(37),
//             Decimal::from_integer(3)
//         );
//     }
// }
