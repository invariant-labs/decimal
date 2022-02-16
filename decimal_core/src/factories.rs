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

    let module_name = string_to_ident("tests_factories", &struct_name.to_string());

    proc_macro::TokenStream::from(quote!(

        impl<T> Factories<T> for #struct_name
            where T: TryInto<#underlying_type>
        {
            fn from_integer(integer: T) -> Self {
                Self::new(
                    integer.try_into().unwrap_or_else(|_| panic!("integer too to create decimal"))
                    .checked_mul(
                        Self::one()
                    ).unwrap()
                )
            }

            fn from_decimal(val: T, scale: u8) -> Self {
                Self::new(
                    if #scale > scale {
                        val.try_into().unwrap_or_else(|_| panic!("decimal: can't convert value"))
                        .checked_mul(
                            (10 as #underlying_type).checked_pow((#scale - scale) as u32).unwrap()

                        )
                    } else {
                        val.try_into().unwrap_or_else(|_| panic!("decimal: can't convert value"))
                            .checked_div(
                                (10 as #underlying_type).checked_pow((scale - #scale) as u32).unwrap()
                            )
                    }.unwrap()
                )
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

            fn test_from_decimal() {
                assert_eq!(
                    #struct_name::from_decimal(0, 0),
                    #struct_name::new(0)
                );

                assert_eq!(
                    #struct_name::from_decimal(0, 3),
                    #struct_name::new(0)
                );

                assert_eq!(
                    #struct_name::from_decimal(1, #scale),
                    #struct_name::new(1)
                );
            }
        }
    ))
}
