use crate::uint::{to_U256, U256};
use crate::{
    decimal::{Decimal, DENOMINATOR},
    traits::BigOps,
};

const DENOMINATOR_U256: U256 = to_U256(DENOMINATOR);
const ALMOST_DEN_U256: U256 = to_U256(DENOMINATOR - 1);

impl BigOps<Decimal> for Decimal {
    fn big_mul(&self, other: &Decimal) -> Decimal {
        Decimal::new(
            U256::from(self.v)
                .checked_mul(U256::from(other.v))
                .unwrap()
                .checked_div(DENOMINATOR_U256)
                .unwrap()
                .as_u128(),
        )
    }

    fn big_mul_up(&self, other: &Decimal) -> Decimal {
        Decimal::new(
            U256::from(self.v)
                .checked_mul(U256::from(other.v))
                .unwrap()
                .checked_add(ALMOST_DEN_U256)
                .unwrap()
                .checked_div(DENOMINATOR_U256)
                .unwrap()
                .as_u128(),
        )
    }

    fn big_div(&self, other: &Decimal) -> Decimal {
        Decimal::new(
            U256::from(self.v)
                .checked_mul(DENOMINATOR_U256)
                .unwrap()
                .checked_div(U256::from(other.v))
                .unwrap()
                .as_u128(),
        )
    }

    fn big_div_up(&self, other: &Decimal) -> Decimal {
        Decimal::new(
            U256::from(self.v)
                .checked_mul(DENOMINATOR_U256)
                .unwrap()
                .checked_add(U256::from(other.v.checked_sub(1).unwrap()))
                .unwrap()
                .checked_div(U256::from(other.v))
                .unwrap()
                .as_u128(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_mul() {
        // precision
        {
            let a = Decimal::from_integer(1);
            let b = Decimal::from_integer(1);
            let c = a.big_mul(&b);
            assert_eq!(c, Decimal::from_integer(1));
        }
        // simple
        {
            let a = Decimal::from_integer(2);
            let b = Decimal::from_integer(3);
            let c = a.big_mul(&b);
            assert_eq!(c, Decimal::from_integer(6));
        }
        // big
        {
            let a = Decimal::new(2u128.pow(127));
            let b = Decimal::from_integer(1);
            let c = a.big_mul(&b);
            assert_eq!(c, a);
        }
        // random
        {
            let a = Decimal::new(87932487422289);
            let b = Decimal::from_integer(982383286787);
            let c = a.big_mul(&b);
            // 87932487422289 * 982383286787
            assert_eq!(c, Decimal::new(86383406009264805062995443));
        }
    }

    #[test]
    fn test_big_mul_up() {
        // mul of little
        {
            let a = Decimal::new(1);
            let b = Decimal::new(1);
            assert_eq!(a.big_mul_up(&b), Decimal::new(1));
        }
        // mul calculable without precision loss
        {
            let a = Decimal::from_integer(1);
            let b = Decimal::from_integer(3) / Decimal::new(10);
            assert_eq!(a.big_mul_up(&b), b);
        }
        {
            let a = Decimal::from_integer(3) / Decimal::from_integer(10);
            let b = Decimal::new(3);
            assert_eq!(a.big_mul_up(&b), Decimal::new(1));
        }
        {
            let a = Decimal::new(2u128.pow(127) - 1);
            let b = Decimal::new(999999999999);
            let result = Decimal::new(170141183460299090548226834484152418424);
            assert_eq!(a.big_mul_up(&b), result);
        }
    }

    #[test]
    fn test_big_div() {
        // decimals
        {
            let a = Decimal::new(1);
            let b = Decimal::from_integer(1);
            assert_eq!(a.big_div(&b), Decimal::new(1));
        }
        // mul calculable without precision loss
        {
            let a = Decimal::from_integer(111);
            let b = Decimal::from_integer(37);
            assert_eq!(a.big_div(&b), Decimal::from_integer(3));
        }
        {
            let a = Decimal::from_integer(1);
            let b = Decimal::from_integer(3);
            assert_eq!(a.big_div(&b), Decimal::new(333333333333));
        }
        {
            let a = Decimal::new(2u128.pow(127));
            let b = Decimal::new(973_248708703324);
            let result = Decimal::new(174817784949492774410002348183691207);
            assert_eq!(a.big_div(&b), result);
        }
    }

    #[test]
    fn test_big_div_up() {
        // decimals
        {
            let a = Decimal::new(1);
            let b = Decimal::from_integer(1);
            assert_eq!(a.big_div_up(&b), Decimal::new(1));
        }
        // mul calculable without precision loss
        {
            let a = Decimal::from_integer(111);
            let b = Decimal::from_integer(37);
            assert_eq!(a.big_div_up(&b), Decimal::from_integer(3));
        }
        {
            let a = Decimal::from_integer(1);
            let b = Decimal::from_integer(3);
            assert_eq!(a.big_div_up(&b), Decimal::new(333333333334));
        }
        {
            let a = Decimal::new(2u128.pow(127));
            let b = Decimal::new(973_248708703324);
            let result = Decimal::new(174817784949492774410002348183691208);
            assert_eq!(a.big_div_up(&b), result);
        }
    }
}
