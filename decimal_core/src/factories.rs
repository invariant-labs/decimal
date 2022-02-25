use quote::quote;

use crate::utils::string_to_ident;
use crate::DecimalCharacteristics;

pub fn generate_factories(characteristics: DecimalCharacteristics) -> proc_macro::TokenStream {
    let DecimalCharacteristics {
        struct_name,
        underlying_type,
        scale,
        ..
    } = characteristics;

    let module_name = string_to_ident("tests_factories_", &struct_name.to_string());

    proc_macro::TokenStream::from(quote!(

        impl<T> Factories<T> for #struct_name
            where
            T: TryInto<u128>,
            T: TryFrom<u128>,
            T: TryInto<#underlying_type>,
            T: From<u8>,
            T: num_traits::ops::checked::CheckedDiv,
            T: num_traits::ops::checked::CheckedAdd,
            T: num_traits::ops::checked::CheckedSub
        {
            fn from_integer(integer: T) -> Self {
                Self::new({
                    let base: #underlying_type = integer.try_into().unwrap_or_else(|_| std::panic!("integer too to create decimal"));
                    base.checked_mul(Self::one()).unwrap()
                })
            }

            fn from_scale(val: T, scale: u8) -> Self {
                Self::new(
                    if #scale > scale {
                        let base: #underlying_type = val.try_into().unwrap_or_else(|_| std::panic!("decimal: can't convert value"));
                        let multiplier: u128 = 10u128.checked_pow((#scale - scale) as u32).unwrap();
                        base.checked_mul(multiplier.try_into().unwrap_or_else(|_| std::panic!("decimal: can't convert value"))).unwrap()
                    } else {
                        let denominator: u128 = 10u128.checked_pow((scale - #scale) as u32).unwrap();
                         val.checked_div(
                            &denominator.try_into().unwrap_or_else(|_| std::panic!("decimal: can't convert value"))
                        ).unwrap().try_into().unwrap_or_else(|_| std::panic!("decimal: can't convert value"))
                    }
                )
            }

            fn from_scale_up(val: T, scale: u8) -> Self {
                Self::new(
                    if #scale > scale {
                        let base: #underlying_type = val.try_into().unwrap_or_else(|_| std::panic!("decimal: can't convert value"));
                        let multiplier: u128 = 10u128.checked_pow((#scale - scale) as u32).unwrap();
                        base.checked_mul(multiplier.try_into().unwrap_or_else(|_| std::panic!("decimal: can't convert value"))).unwrap()
                    } else {
                        let multiplier: u128 = 10u128.checked_pow((scale - #scale) as u32).unwrap();
                        let denominator: T = multiplier.try_into().unwrap_or_else(|_| std::panic!("decimal: can't convert value"));
                        val
                        .checked_add(
                            &denominator.checked_sub(&T::from(1u8)).unwrap()
                        ).unwrap()
                        .checked_div(
                            &denominator
                        ).unwrap()
                        .try_into().unwrap_or_else(|_| std::panic!("decimal: can't convert value"))
                    }
                )
            }
        }

        impl<T: Decimal> BetweenDecimals<T> for #struct_name
        where
            Self: Factories<T::U>,
        {
            fn from_decimal(other: T) -> Self {
                Self::from_scale(other.get(), T::scale())
            }

            fn from_decimal_up(other: T) -> Self {
                Self::from_scale_up(other.get(), T::scale())
            }
        }


        #[cfg(test)]
        pub mod #module_name {
            use super::*;

            #[test]
            fn test_from_integer() {
                assert_eq!(
                    #struct_name::from_integer(0),
                    #struct_name::new(0)
                );
            }

            fn test_from_scale() {
                assert_eq!(
                    #struct_name::from_scale(0, 0),
                    #struct_name::new(0)
                );
                assert_eq!(
                    #struct_name::from_scale_up(0, 0),
                    #struct_name::new(0)
                );

                assert_eq!(
                    #struct_name::from_scale(0, 3),
                    #struct_name::new(0)
                );
                assert_eq!(
                    #struct_name::from_scale_up(0, 3),
                    #struct_name::new(0)
                );

                assert_eq!(
                    #struct_name::from_scale(42, #scale),
                    #struct_name::new(42)
                );
                assert_eq!(
                    #struct_name::from_scale_up(42, #scale),
                    #struct_name::new(42)
                );


                assert_eq!(
                    #struct_name::from_scale(42, #scale),
                    #struct_name::new(42)
                );
                assert_eq!(
                    #struct_name::from_scale_up(42, #scale),
                    #struct_name::new(42)
                );

                let denominator = (10 as #underlying_type).checked_pow((#scale + 1) as u32).unwrap().checked_add(1).unwrap();
                assert_eq!(
                    #struct_name::from_scale(42, #scale + 1),
                    #struct_name::new(4)
                );
                assert_eq!(
                    #struct_name::from_scale_up(42, #scale + 1),
                    #struct_name::new(5)
                );

            }
        }
    ))
}
