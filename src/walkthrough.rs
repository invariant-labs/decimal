// Welcome in walkthrough file of the decimal library
// Below you can find a short example of use of this library
// For more visit the `Invariant protocol's` repository

#[cfg(test)]
mod walkthrough {
    use crate::decimal;
    use crate::traits::*;
    use crate::U256; // used as the default type

    #[decimal(2, u128)] // second argument is the `big type`, checkout readme to know more
    #[derive(Default, PartialEq, Debug, Clone, Copy)]
    struct Percentage(u8);

    #[decimal(4)]
    #[derive(Default, PartialEq, Debug, Clone, Copy)]
    struct Price(u128);

    #[test]
    fn example_price_after_discount() {
        let price = Price::from_integer(10); // this corresponds with 10 * 10^k so 10^7 in this case
        let discount = Percentage::new(10); // using new doesn't account for decimal places so 0.10 here

        // addition expects being called for left and right values being of the same type
        // multiplication doesn't so you can be used like this:
        let price = price * (Percentage::from_integer(1) - discount); // the resulting type is always the type of the left value

        assert_eq!(price, Price::from_integer(9)); // asserts work without a problem
    }

    #[test]
    fn example_find_discount() {
        let original_price = Price::from_integer(10);
        let price_after_discount = Price::from_integer(8);

        let ratio = (original_price - price_after_discount) / original_price;
        let ratio = Percentage::from_decimal(ratio); // this will change scale to 2 so ratio is a percentage
        assert_eq!(ratio, Percentage(20)); // other way to declare a tuple struct, works same as `Percentage::new()`
    }

    #[test]
    fn example_price_rounding() {
        // Rounding is easier to show on small values
        let original_price = Price(5); // corresponds to 0.0005

        // There is one more useful way to declare a decimal: from_scale()
        // First argument is the value, second one is the shift of it to the right, like so:
        let half_price_coupon = Percentage::from_scale(5u8, 1); // corresponds 5 / 10^1, so 0.5

        // let price_after_discount = original_price * half_price_coupon; // expects 0.0003
        // The line above would round down, not great for our shop, let's round up:

        let price_after_discount = original_price.mul_up(half_price_coupon);
        assert_eq!(price_after_discount, Price(3));

        let price_after_discount = price_after_discount.mul_up(half_price_coupon); // corresponds to 0.0002
        let price_after_discount = price_after_discount.mul_up(half_price_coupon); // result will be exact if rounding is not needed
        assert_eq!(price_after_discount, Price(1));
    }

    #[test]
    #[should_panic]
    fn example_overflow_without_being_too_big() {
        let percentage = Percentage(110); // 110%

        // The line below will panic, check out readme to understand why
        let _squared = percentage * percentage;
    }

    #[test]
    fn example_prepare_for_overflow() {
        let percentage = Percentage(110); // 110%

        // using `big type` for calculations (more on that in the readme)
        let squared = percentage.big_mul(percentage);
        assert_eq!(squared, Percentage(121));
    }

    #[test]
    fn example_implement_additional_feature() {
        // Additional features can be easily added like so:
        impl Percentage {
            fn square(self) -> Self {
                self.big_mul(self)
            }
        }

        let percentage = Percentage(110); // 110%
        let squared = percentage.square();
        assert_eq!(squared, Percentage(121));
    }

    #[test]
    fn example_extract_overflowing_value() {
        // This would be rarely needed, but it's possible to handle overflow like so:
        let percentage = Percentage::from_integer(2); // 200%

        // let squared = percentage.big_mul(percentage);
        // this wouldn't help here, as 400 > u8::MAX

        let squared = percentage.big_mul_to_value(percentage);
        // now we a value and can use it for example like so:
        let inverse = Percentage(100).big_div_by_number(squared);

        assert_eq!(inverse, Percentage(25));
    }
}
