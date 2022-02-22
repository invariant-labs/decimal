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
            where T: TryInto<#underlying_type>
        {
            fn from_integer(integer: T) -> Self {
                Self::new(
                    integer.try_into().unwrap_or_else(|_| std::panic!("integer too to create decimal"))
                    .checked_mul(
                        Self::one()
                    ).unwrap()
                )
            }

            fn from_scale(val: T, scale: u8) -> Self {
                Self::new(
                    if #scale > scale {
                        val.try_into().unwrap_or_else(|_| std::panic!("decimal: can't convert value"))
                        .checked_mul(
                            (10 as #underlying_type).checked_pow((#scale - scale) as u32).unwrap()
                        )
                    } else {
                        val.try_into().unwrap_or_else(|_| std::panic!("decimal: can't convert value"))
                            .checked_div(
                                (10 as #underlying_type).checked_pow((scale - #scale) as u32).unwrap()
                            )
                    }.unwrap()
                )
            }

            fn from_scale_up(val: T, scale: u8) -> Self {
                Self::new(
                    if #scale > scale {
                        val.try_into().unwrap_or_else(|_| std::panic!("decimal: can't convert value"))
                        .checked_mul(
                            (10 as #underlying_type).checked_pow((#scale - scale) as u32).unwrap()
                        )
                    } else {
                        let denominator = (10 as #underlying_type).checked_pow((scale - #scale) as u32).unwrap();
                        val.try_into().unwrap_or_else(|_| std::panic!("decimal: can't convert value"))
                            .checked_add(
                                denominator.checked_sub(
                                    #underlying_type::try_from(1).unwrap()
                                ).unwrap()
                            ).unwrap()
                            .checked_div(
                                denominator
                            )
                    }.unwrap()
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
