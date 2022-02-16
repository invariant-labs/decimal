// use crate::decimal::{Decimal, DENOMINATOR, SCALE};

// impl Decimal {
//     pub fn new(value: u128) -> Decimal {
//         Decimal { v: value }
//     }

//     pub fn from_integer(integer: u128) -> Decimal {
//         Decimal::new(integer * DENOMINATOR)
//     }

//     pub fn one() -> Decimal {
//         Decimal::new(DENOMINATOR)
//     }

//     pub fn from_decimal(val: u128, scale: u8) -> Decimal {
//         if SCALE > scale {
//             Decimal::new(val * 10u128.pow((SCALE - scale).into()))
//         } else {
//             let denominator = 10u128.checked_pow((scale - SCALE).into()).unwrap();
//             Decimal::new(val.checked_div(denominator).unwrap())
//         }
//     }
// }

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn test_new() {
//         {
//             let d = Decimal::new(0);
//             assert_eq!(d.v, 0);
//         }
//         {
//             let d = Decimal::new(1);
//             assert_eq!(d.v, 1);
//         }
//         {
//             let d = Decimal::new(5554544532543567587432);
//             assert_eq!(d.v, 5554544532543567587432);
//         }
//         {
//             let d = Decimal::new(u128::MAX);
//             assert_eq!(d.v, u128::MAX);
//         }
//     }

//     #[test]
//     fn test_from_integer() {
//         {
//             let d = Decimal::from_integer(0);
//             assert_eq!(d.v, 0);
//         }
//         {
//             let d = Decimal::from_integer(1);
//             assert_eq!(d.v, DENOMINATOR);
//         }
//         {
//             let d = Decimal::from_integer(7354123473);
//             assert_eq!(d.v, 7354123473 * DENOMINATOR);
//         }
//         {
//             let d = Decimal::from_integer(u64::MAX as u128);
//             assert_eq!(d.v, u64::MAX as u128 * DENOMINATOR);
//         }
//     }

//     #[test]
//     fn test_one() {
//         let d = Decimal::one();
//         assert_eq!(d.v, DENOMINATOR);
//     }

//     #[test]
//     fn test_from_decimal() {
//         {
//             let d = Decimal::from_decimal(0, 0);
//             assert_eq!(d.v, 0);
//         }
//         {
//             let d = Decimal::from_decimal(0, 30);
//             assert_eq!(d.v, 0);
//         }
//         {
//             let d = Decimal::from_decimal(1, 0);
//             assert_eq!(d, Decimal::one());
//         }
//         {
//             let d = Decimal::from_decimal(345345, 30);
//             assert_eq!(d, Decimal::new(0));
//         }
//         {
//             let d = Decimal::from_decimal(930923538929, 8);
//             assert_eq!(d.v, 930923538929 * 10u128.pow((SCALE - 8u8).into()));
//         }
//         {
//             let d = Decimal::from_decimal(930923538929, 20);
//             assert_eq!(d.v, 930923538929 / 10u128.pow((20 - SCALE).into()));
//         }
//     }
// }
