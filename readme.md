# Decimal library

This is a Rust fixed-point numeric library targeting blockchain. It was created purely for practical reasons as a fast and simple way to use checked math with given decimal precision.

It has achieved present form over several iterations, first being implemented inside _Synthetify protocol_.
The current version leverages macros, traits and generics to exchange dozens of lines of error prone code with a single line and generating the rest. In this form it is used inside of _Invariant protocol_ and was audited as a part of it.

It allows a definition of multiple types with different precisions and primitive types and calculations in between them, see below for a quick example.

## Quickstart

The library is used by adding a macro `#[decimal(k)]`, where _k_ is a desired decimal precision (number decimal places after the dot).

This macro generates an implementation of several generic traits for each struct it is called on allowing basic operations in between them.

### Basic example

Having imported the library you can declare a type like so:

    #[decimal(2)]
    #[derive(Default, PartialEq, Debug, Clone, Copy)]
    struct Percentage(u32);

-   `#[decimal(3, u128)]` - call to the macro generating the code for decimal
-   `#[derive(Default, Debug, Clone, Copy, PartialEq)]` - derivation of some common built-in traits (these five are needed)
-   `struct R(u32);` - declaration of the struct itself

### Deserialization

Named structs can be deserialized without a problem like so:

    #[decimal(6)]
    #[zero_copy]
    #[derive(AnchorSerialize, AnchorDeserialize, ...)]
    pub struct Price {
        pub v: u128,
    }

### Basic operations

All methods generated by the macro use _checked math_ and panic on overflow. Operators are overloaded where possible for ease of use.

Basic example using types defined above would look like this:

    let price = Price::from_integer(10); // this corresponds with 10 * 10^k so 10^7 in this case

    let discount = Percentage::new(10); // using new doesn't account for decimal places so 0.10 here

    // addition expects being called for left and right values being of the same type
    // multiplication doesn't so you can be used like this:
    let price = price * (Percentage::from_integer(1) - discount); // the resulting type is always the type of the left value

For more examples continue to the `walkthrough.rs`

## Parameters for the macro

As mentioned the first argument of macro controls the amount of places after the dot. It can range between 0 and 38. It can be read by `Price::scale()`.
The second one is optional and can be a bit harder to grasp

### The Big Type

The second argument taken has a weird name of a _big type_. It sets the type that is to be used when calling the methods with \__big_\_ in the name. Its purpose is to avoid _temporary overflows_ so an overflow that occurs while calculating theS return value despite that value fitting in the given type. Consider the example below

    #[decimal(2)]
    #[derive(Default, std::fmt::Debug, Clone, Copy, PartialEq)]
    struct Percentage(u8);

    let p = Percentage(110); // 110% or 1.1

    // assert_eq!(p * p, Percentage(121));      <- this would panic
    assert_eq!(p.big_mul(p), Percentage(121));  <- this will work fine

To understand why it works like that look at the multiplication of decimal does under the hood:

# What happens inside (on the math side)

Most of this library uses really basic math, a few things that might not be obvious are listed below

## Keeping the scale

An multiplication of two percentages (scale of 2) using just the values would look like this:

-   <img src="https://latex.codecogs.com/gif.latex?\dfrac{x \div 10^2}{y \div 10^2} = \dfrac{x}{y}" />

Using numbers it would look like this:

$10\% \div 10\% = 10\div10=1\%$

Which is obviously wrong. What we need is multiplying everything by $10^{scale}$ at every division. So it should look like this

$\dfrac{x \div 10^{scale}}{y \div 10^{scale}} \times 10^{scale} = \dfrac{x}{y} \div 10^{scale}$

Which checks out with the example above

In general at every multiplication of values there needs to be a division, and vice versa. This was the first purpose of this library - to abstract it away to make for less code, bugs and wasted time.

The important thing here is that multiplication has to occur before division to keep the precision, but this is also abstracted away.

## Rounding errors

By default every method rounds down but has a counterpart ending with _up_ rounding the opposite way.

Rounding works by addition of $denominator - 1$ to the numerator, so the _mul_up_ would look like so:

$\dfrac{x \times y + (10^{scale}-1)}{10^{scale}}$

For example for $10\% \times 1\%$

$\dfrac{10 \times 1 + (10^{2}-1)}{10^{2}} = 109 \div 100 = 1\%$

# What happens inside (on a code level)

As you do know by this point the whole library is in a form of macro. Inside of it is an implementation of several traits in a generic form to allow calling methods between any two of the implementations.

-   `Decimal` - all other traits are dependent on it, and by implementing it you can you your implementation with any of the other traits. One of use cases my be implementing it on base 2
    -   `type U: Debug + Default;` - an _associated type_, the primitive (or not) where value is kept, the type of first field in the struct on which macro was called
    -   `fn get(&self) -> Self::U;` - the value of a decimal
    -   `fn new(value: Self::U) -> Self;` - the constructor
    -   `fn here<Y: TryFrom<Self::U>>(&self) -> Y;` - same as get, but also 'tries into' the needed value
    -   `fn scale() -> u8;` - the amount of decimal places (given in the macro)
    -   `fn one<T: TryFrom<u128>>() -> T;` - basically $10^{scale}$, evaluated on the compile time
    -   `fn almost_one<T: TryFrom<u128>>() -> T;` - same as above but $-1$, also on compile time
-   `std::ops` - addition, subtraction, multiplication and division together with there assignment counterparts (+=)
-   `pub trait BigOps<T>` - same as above but with previously mentioned big types used when calculating
-   `pub trait Others<T>` - trait for future operations if needed, right now with only two methods
    -   `fn mul_up(self, rhs: T) -> Self;` - multiplication, rounding uo
    -   `fn div_up(self, rhs: T) -> Self;` - division, rounding up
-   `pub trait Factories<T>` - methods used as ctors (excluding new)

    -   `fn from_integer(integer: T) -> Self;` - creates self with value of: $integer \times 10^{scale}$
    -   `fn from_scale(integer: T, scale: u8) -> Self;` - creates self with value of: $integer \times 10^{scale - given\_scale}$
    -   `fn from_scale_up(integer: T, scale: u8) -> Self;` - same as above but with rounding up

-   `pub trait BetweenDecimals<T>` - used for conversion between different types, possibly with different scales
-   `pub trait ToValue<T, B>` and `pub trait ByNumber<B>` - can be used together to take overflowing values outside of a type and then put it inside, shouldn't be needed often
