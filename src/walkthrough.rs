// Welcome in walkthrough file of the decimal library
// Below you can find a short example of use of this library
// For more visit the `Invariant protocol's` repository

#[cfg(test)]
mod walkthrough {
    use crate::decimal;
    use crate::traits::*;
    use crate::U256; // used as the default type

    #[decimal(2, u128)]
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
        let _price = price * (Percentage::from_integer(1) - discount); // the resulting type is always the type of the left value
    }
}
