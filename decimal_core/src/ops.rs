use quote::quote;
use syn::Ident;

use crate::utils::string_to_ident;

pub fn generate_ops(
    struct_name: Ident,
    // field_name: Ident,
    underlying_type: proc_macro2::TokenStream,
) -> proc_macro::TokenStream {
    let name_str = &struct_name.to_string();

    let module_name = string_to_ident("tests_", &name_str);

    proc_macro::TokenStream::from(quote!(
        impl Add for #struct_name {
            type Output = #struct_name;

            fn add(self, rhs: Self) -> #struct_name {
                Self::new(self.get().checked_add(rhs.get()).unwrap())
            }
        }

        impl Sub for #struct_name {
            type Output = #struct_name;

            fn sub(self, rhs: Self) -> #struct_name {
                Self::new(self.get().checked_sub(rhs.get()).unwrap())
            }
        }

        impl<T: Decimal> Mul<T> for #struct_name
        where
            T::U: TryInto<#underlying_type>,
        {
            type Output = #struct_name;

            fn mul(self, rhs: T) -> #struct_name {
                #struct_name::new(
                    self.get()
                        .checked_mul(
                            rhs.get()
                                .try_into()
                                .unwrap_or_else(|_| panic!("could not parse")),
                        )
                        .unwrap()
                        .checked_div(T::one())
                        .unwrap(),
                )
            }
        }

        impl<T: Decimal> Div<T> for #struct_name
        where
            T::U: TryInto<#underlying_type>,
        {
            type Output = #struct_name;

            fn div(self, rhs: T) -> #struct_name {
                #struct_name::new(
                    self.get()
                        .checked_mul(T::one())
                        .unwrap()
                        .checked_div(
                            rhs.get()
                                .try_into()
                                .unwrap_or_else(|_| panic!("could not parse")),
                        )
                        .unwrap(),
                )
            }
        }

        #[cfg(test)]
        pub mod #module_name {
            use super::*;

            #[test]
            fn test_add () {
                let a = #struct_name::new(1);
                let b = #struct_name::new(1);
                assert_eq!(a + b, #struct_name::new(2));
            }

            #[test]
            fn test_sub () {
                let a = #struct_name::new(1);
                let b = #struct_name::new(1);
                assert_eq!(a - b, #struct_name::new(0));
            }

            #[test]
            fn test_mul () {
                let a = #struct_name::new(2);
                let b = #struct_name::new(#struct_name::one());
                assert_eq!(a * b, #struct_name::new(2));
            }

            #[test]
            fn test_div () {
                let a = #struct_name::new(2);
                let b = #struct_name::new(#struct_name::one());
                assert_eq!(a / b, #struct_name::new(2));
            }
        }
    ))
}
