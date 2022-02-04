use anchor_lang::prelude::*;
use integer_sqrt::IntegerSquareRoot;
use std::{convert::TryInto, fmt::Display};

use crate::token_amount::*;
use crate::traits::*;
use crate::uint::U256;

pub const SCALE: u8 = 12;
pub const DENOMINATOR: u128 = 10u128.pow(SCALE as u32);

#[zero_copy]
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, AnchorDeserialize, AnchorSerialize)]
pub struct Decimal {
    pub v: u128,
}
// pub struct Decimal::new(pub u128);

impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}",
            self.v.checked_div(DENOMINATOR).unwrap(),
            self.v % DENOMINATOR
        )
    }
}

impl Decimal {
    pub fn new(value: u128) -> Decimal {
        Decimal { v: value }
    }

    pub fn from_integer(integer: u128) -> Decimal {
        Decimal::new(integer * DENOMINATOR)
    }

    pub fn one() -> Decimal {
        Decimal::new(DENOMINATOR)
    }

    pub fn from_decimal(val: u128, scale: u8) -> Decimal {
        if SCALE > scale {
            Decimal::new(val * 10u128.pow((SCALE - scale).into()))
        } else {
            let denominator = 10u128.checked_pow((scale - SCALE).into()).unwrap();
            Decimal::new(val.checked_div(denominator).unwrap())
        }
    }

    pub fn is_zero(self) -> bool {
        self.v == 0
    }

    pub fn from_token_amount(amount: TokenAmount) -> Decimal {
        Decimal::from_integer(amount.0.into())
    }

    pub fn div_up(self, other: Decimal) -> Decimal {
        Decimal::new(
            self.v
                .checked_mul(DENOMINATOR)
                .unwrap()
                .checked_add(other.v.checked_sub(1).unwrap())
                .unwrap()
                .checked_div(other.v)
                .unwrap(),
        )
    }

    pub fn sqrt(self) -> Decimal {
        Decimal::new(self.v.checked_mul(DENOMINATOR).unwrap().integer_sqrt())
    }

    pub fn to_token_floor(self) -> TokenAmount {
        TokenAmount(self.v.checked_div(DENOMINATOR).unwrap().try_into().unwrap())
    }

    pub fn to_token_ceil(self) -> TokenAmount {
        TokenAmount(
            self.v
                .checked_add(DENOMINATOR.checked_sub(1).unwrap())
                .unwrap()
                .checked_div(DENOMINATOR)
                .unwrap()
                .try_into()
                .unwrap(),
        )
    }
}

pub trait Pow<T>: Sized {
    fn pow(self, exp: T) -> Decimal;
}

impl Pow<i128> for Decimal {
    fn pow(self, exp: i128) -> Decimal {
        let one = Decimal::one();
        let one_u256 = U256::from(one.v);

        if exp == 0 {
            return one;
        }

        let mut current_exp = if exp > 0 { exp } else { -exp };
        let mut base = U256::from(self.v);
        let mut result = U256::from(one.v);

        while current_exp > 0 {
            if current_exp % 2 != 0 {
                result = result
                    .checked_mul(base)
                    .unwrap()
                    .checked_div(one_u256)
                    .unwrap();
            }
            current_exp /= 2;
            base = base
                .checked_mul(base)
                .unwrap()
                .checked_div(one_u256)
                .unwrap();
        }
        if exp < 0 {
            result = one_u256
                .checked_mul(DENOMINATOR.into())
                .unwrap()
                .checked_div(result)
                .unwrap();
        }
        Decimal::new(result.as_u128())
    }
}

impl OpsUp<Decimal> for Decimal {
    fn mul_up(self, other: Decimal) -> Decimal {
        Decimal::new(
            self.v
                .checked_mul(other.v)
                .unwrap()
                .checked_add(DENOMINATOR.checked_sub(1).unwrap())
                .unwrap()
                .checked_div(DENOMINATOR)
                .unwrap(),
        )
    }

    fn div_up(self, other: Decimal) -> Decimal {
        Decimal::new(
            self.v
                .checked_mul(DENOMINATOR)
                .unwrap()
                .checked_add(other.v.checked_sub(1).unwrap())
                .unwrap()
                .checked_div(other.v)
                .unwrap(),
        )
    }
}

impl OpsUp<TokenAmount> for Decimal {
    fn mul_up(self, other: TokenAmount) -> TokenAmount {
        TokenAmount(
            self.v
                .checked_mul(other.0 as u128)
                .unwrap()
                .checked_add(DENOMINATOR.checked_sub(1).unwrap())
                .unwrap()
                .checked_div(DENOMINATOR)
                .unwrap()
                .try_into()
                .unwrap(),
        )
    }

    fn div_up(self, other: TokenAmount) -> TokenAmount {
        TokenAmount(0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ops() {
        assert_eq!(
            Decimal::from_integer(0) + Decimal::from_integer(2),
            Decimal::from_integer(2)
        );
        assert_eq!(
            Decimal::from_integer(2) - Decimal::from_integer(1),
            Decimal::from_integer(1)
        );
        assert_eq!(
            Decimal::from_integer(2) * Decimal::from_integer(2),
            Decimal::from_integer(4)
        );
        assert_eq!(
            Decimal::from_integer(111) / Decimal::from_integer(37),
            Decimal::from_integer(3)
        );
    }

    #[test]
    fn test_ord() {
        assert!(Decimal::from_integer(1) < Decimal::from_integer(2));
        assert!(Decimal::from_integer(2) > Decimal::from_integer(1));
        assert!(Decimal::from_integer(1) <= Decimal::from_integer(2));
        assert!(Decimal::from_integer(1) >= Decimal::from_integer(1));
        assert!(Decimal::from_integer(1) == Decimal::from_integer(1));
        assert!(Decimal::from_integer(1) != Decimal::from_integer(2));
    }

    #[test]
    fn test_mul_up() {
        // mul of little
        {
            let a = Decimal::new(1);
            let b = Decimal::new(1);
            assert_eq!(a.mul_up(b), Decimal::new(1));
        }
        // mul calculable without precision loss
        {
            let a = Decimal::from_integer(1);
            let b = Decimal::from_integer(3) / Decimal::new(10);
            assert_eq!(a.mul_up(b), b);
        }
        {
            let a = Decimal::from_integer(1);
            let b = TokenAmount(1);
            assert_eq!(a.mul_up(b), TokenAmount(1));
        }
        {
            let a = Decimal::from_integer(3) / Decimal::from_integer(10);
            let b = TokenAmount(3);
            assert_eq!(a.mul_up(b), TokenAmount(1));
        }
    }

    #[test]
    fn test_div_up() {
        // div of zero
        {
            let a = Decimal::new(0);
            let b = Decimal::new(1);
            assert_eq!(a.div_up(b), Decimal::new(0));
        }
        // div check rounding up
        {
            let a = Decimal::new(1);
            let b = Decimal::from_integer(2);
            assert_eq!(a.div_up(b), Decimal::new(1));
        }
        // div big number
        {
            let a = Decimal::new(200_000_000_001);
            let b = Decimal::from_integer(2);
            assert_eq!(a.div_up(b), Decimal::new(100_000_000_001));
        }
        {
            let a = Decimal::new(42);
            let b = Decimal::from_integer(10);
            assert_eq!(a.div_up(b), Decimal::new(5));
        }
    }

    #[test]
    fn test_sqrt() {
        assert!(Decimal::from_integer(9)
            .sqrt()
            .gt(&Decimal::new(DENOMINATOR * 7 / 5)));
        assert!(Decimal::from_integer(2)
            .sqrt()
            .lt(&Decimal::new(DENOMINATOR * 3 / 2)));
    }

    #[test]
    fn test_pow() {
        // Zero base
        {
            let base = Decimal::new(0);
            let exp = 100;
            let result = base.pow(exp);
            assert_eq!(result, Decimal::new(0));
        }
        // Zero exponent
        {
            let base = Decimal::from_integer(10);
            let exp = 0;
            let result = base.pow(exp);
            let expected = Decimal::from_integer(1);
            assert_eq!(result, expected);
        }
        // 2^17
        {
            let base = Decimal::from_integer(2);
            let exp = 17;
            let result = base.pow(exp);
            // should be 131072
            let expected = Decimal::from_integer(131072);
            assert_eq!(result, expected);
        }
        // 1.00000002^525600
        {
            let base = Decimal::from_decimal(1_000_000_02, 8);
            let exp = 525600;
            let result = base.pow(exp);
            // expected 1.010567433635
            // real     1.0105674450753...
            let expected = Decimal::new(1010567433635);
            assert_eq!(result, expected);
        }
        // 1.000000015^2
        {
            let base = Decimal::from_decimal(1_000_000_015, 9);
            let exp = 2;
            let result = base.pow(exp);
            // expected 1.000000030000
            // real     1.0000000300000002...
            let expected = Decimal::new(1000000030000);
            assert_eq!(result, expected);
        }
        // 1^525600
        {
            let base = Decimal::from_integer(1);
            let exp = 525600;
            let result = base.pow(exp);
            // expected not change value
            let expected = Decimal::from_integer(1);
            assert_eq!(result, expected);
        }
        // 1^(-1)
        {
            let base = Decimal::from_integer(1);
            let exp = -1;
            let result = base.pow(exp);
            let expected = Decimal::from_integer(1);
            assert_eq!(result, expected);
        }
        // 2^(-3)
        {
            let base = Decimal::from_integer(2);
            let exp = -12;
            let result = base.pow(exp);
            let expected = Decimal::new(244140625);
            assert_eq!(result, expected);
        }
        // 3^(-5)
        {
            let base = Decimal::from_integer(3);
            let exp = -5;
            let result = base.pow(exp);
            // expected 4.115226337
            // real     4.11522633744855967...
            let expected = Decimal::new(4115226337);
            assert_eq!(result, expected);
        }
        // 0.03^(-2)
        {
            let base = Decimal::from_decimal(0_03, 2);
            let exp = -2;
            let result = base.pow(exp);
            let expected = Decimal::new(1111111111111111);
            assert_eq!(result, expected);
        }
        // 0.99^(-365)
        {
            let base = Decimal::from_decimal(0_99, 2);
            let exp = -365;
            let result = base.pow(exp);
            // expected 3.9188078734077
            // real     3.9188078730559...
            let expected = Decimal::new(39188078734077);
            assert_eq!(result, expected);
        }
        // 1.001^(-100000)
        {
            let base = Decimal::from_decimal(1_000_1, 4);
            let exp = -100_000;
            let result = base.pow(exp);
            // expected 0.000045422634
            // real     0.0000454226338...
            let expected = Decimal::new(45422634);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_to_token() {
        // equal
        {
            let d = Decimal::from_integer(1);

            assert_eq!(d.to_token_floor(), TokenAmount(1));
            assert_eq!(d.to_token_ceil(), TokenAmount(1));
        }
        // little over
        {
            let d = Decimal::from_integer(1) + Decimal::new(1);

            assert_eq!(d.to_token_floor(), TokenAmount(1));
            assert_eq!(d.to_token_ceil(), TokenAmount(2));
        }
        // little below
        {
            let d = Decimal::from_integer(2) - Decimal::new(1);

            assert_eq!(d.to_token_floor(), TokenAmount(1));
            assert_eq!(d.to_token_ceil(), TokenAmount(2));
        }
    }
}
