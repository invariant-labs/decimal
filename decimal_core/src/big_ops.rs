use quote::quote;

use crate::utils::string_to_ident;
use crate::DecimalCharacteristics;

pub fn generate_big_ops(characteristics: DecimalCharacteristics) -> proc_macro::TokenStream {
    let DecimalCharacteristics {
        struct_name,
        big_type,
        ..
    } = characteristics;

    let name_str = &struct_name.to_string();

    let module_name = string_to_ident("tests_big_ops_", &name_str);

    proc_macro::TokenStream::from(quote!(
        impl<T: Decimal> BigOps<T> for #struct_name
        where
            T::U: TryInto<#big_type>,
        {
            fn big_mul(self, rhs: T) -> #struct_name {
                #struct_name::new(
                    #big_type::try_from(self.get()).unwrap()
                        .checked_mul(
                            rhs.get()
                                .try_into().unwrap_or_else(|_| std::panic!("rhs value could not be converted to big type in `big_mul`")),
                        ).unwrap()
                        .checked_div(
                            T::one()
                        ).unwrap()
                        .try_into().unwrap()
                )
            }

            fn big_mul_up(self, rhs: T) -> #struct_name {
                #struct_name::new(
                    #big_type::try_from(self.get()).unwrap()
                        .checked_mul(
                            rhs.get()
                                .try_into().unwrap_or_else(|_| std::panic!("rhs value could not be converted to big type in `big_mul_up`")),
                        ).unwrap()
                        .checked_add(T::almost_one()).unwrap()
                        .checked_div(
                            T::one()
                        ).unwrap()
                        .try_into().unwrap()
                )
            }

            fn big_div(self, rhs: T) -> #struct_name {
                #struct_name::new(
                    #big_type::try_from(self.get()).unwrap()
                        .checked_mul(
                            T::one()
                        ).unwrap()
                        .checked_div(
                            rhs.get()
                                .try_into().unwrap_or_else(|_| std::panic!("rhs value could not be converted to big type in `big_div`")),
                        ).unwrap()
                        .try_into().unwrap()
                )
            }

            fn big_div_up(self, rhs: T) -> #struct_name {
                #struct_name::new(
                    #big_type::try_from(self.get()).unwrap()
                        .checked_mul(
                            T::one()
                        ).unwrap()
                        .checked_add(
                            rhs.get()
                                .try_into().unwrap_or_else(|_| std::panic!("rhs value could not be converted to big type in `big_div_up`"))
                                .checked_sub(#big_type::from(1u128)).unwrap()
                        ).unwrap()
                        .checked_div(
                            rhs.get()
                                .try_into().unwrap_or_else(|_| std::panic!("rhs value could not be converted to big type in `big_div_up`")),
                        ).unwrap()
                        .try_into().unwrap()
                )
            }
        }

        #[cfg(test)]
        pub mod #module_name {
            use super::*;

            #[test]
            fn test_big_mul () {
                let a = #struct_name::new(2);
                let b = #struct_name::new(#struct_name::one());
                assert_eq!(a.big_mul(b), #struct_name::new(2));
            }

            fn test_big_mul_up () {
                let a = #struct_name::new(2);
                let b = #struct_name::new(#struct_name::one());
                assert_eq!(a.big_mul_up(b), #struct_name::new(2));
            }

            #[test]
            fn test_big_div () {
                let a = #struct_name::new(2);
                let b = #struct_name::new(#struct_name::one());
                assert_eq!(a.big_div(b), #struct_name::new(2));
            }

            #[test]
            fn test_big_div_up () {
                let a = #struct_name::new(2);
                let b = #struct_name::new(#struct_name::one());
                assert_eq!(a.big_div_up(b), #struct_name::new(2));
            }
        }
    ))
}
