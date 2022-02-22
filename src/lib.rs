mod traits;
mod uint;

pub use crate::uint::U256;

pub use decimal_core::decimal;
pub use traits::*;

#[cfg(test)]
#[decimal(3, u128)]
#[derive(Default, std::fmt::Debug, Clone, Copy, PartialEq)]
struct R(u32);

#[cfg(test)]
#[decimal(1)]
#[derive(Default, std::fmt::Debug, Clone, Copy, PartialEq)]
struct Q {
    v: u16,
}

#[cfg(test)]
#[decimal(0)]
#[derive(Default, std::fmt::Debug, Clone, Copy, PartialEq)]
struct N(u8);

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_from_decimal() {
        let r = R(42);
        let q = Q { v: 144 };
        let n = N(3);

        assert_eq!(R::from_decimal(r), r);
        assert_eq!(R::from_decimal(q), R(14400));
        assert_eq!(R::from_decimal(n), R(3000));

        assert_eq!(Q::from_decimal(r), Q { v: 0 });
        assert_eq!(Q::from_decimal(q), q);
        assert_eq!(Q::from_decimal(n), Q { v: 30 });

        assert_eq!(N::from_decimal(n), n);
        assert_eq!(N::from_decimal(q), N(14));
    }

    #[test]
    fn test_from_decimal_up() {
        let r = R(42);
        let q = Q { v: 144 };
        let n = N(3);

        assert_eq!(R::from_decimal_up(r), r);
        assert_eq!(R::from_decimal_up(q), R(14400));
        assert_eq!(R::from_decimal_up(n), R(3000));

        assert_eq!(Q::from_decimal_up(r), Q { v: 1 });
        assert_eq!(Q::from_decimal_up(q), q);
        assert_eq!(Q::from_decimal_up(n), Q { v: 30 });

        assert_eq!(N::from_decimal_up(n), n);
        assert_eq!(N::from_decimal_up(q), N(15));
    }

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
    fn test_big_mul() {
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
            let a = R::new(879132);
            let b = Q::new(9383);
            let d = a.big_mul(b);
            let u = a.big_mul_up(b);

            let expected = R(824889555);
            assert_eq!(d, expected);
            assert_eq!(u, expected + R(1));
        }
    }

    #[test]
    fn test_big_div() {
        // precision
        {
            let a = Q::from_integer(1);
            let b = R::from_integer(1);
            let d = a.big_div(b);
            let u = a.big_div_up(b);
            assert_eq!(d, Q::from_integer(1));
            assert_eq!(u, Q::from_integer(1));
        }
        // simple
        {
            let a = Q::from_integer(6);
            let b = R::from_integer(3);
            let d = a.big_div(b);
            let u = a.big_div_up(b);
            assert_eq!(d, Q::from_integer(2));
            assert_eq!(u, Q::from_integer(2));
        }
        // big
        {
            let a = Q::new(2u16.pow(15));
            let b = R::from_integer(1);
            let d = a.big_div(b);
            let u = a.big_div_up(b);

            let expected = Q::new(2u16.pow(15));
            assert_eq!(d, expected);
            assert_eq!(u, expected);
        }
        // random
        {
            let a = R::new(824889555);
            let b = Q::new(9383);
            let d = a.big_div(b);
            let u = a.big_div_up(b);

            let expected = R(879131);
            assert_eq!(d, expected);
            assert_eq!(u, expected + R(1));
        }
    }
}
