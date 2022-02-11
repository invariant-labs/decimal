use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::utils::string_to_ident;

pub fn generate_ops(
    struct_name: Ident,
    // field_name: Ident,
    // underlying_type: TokenStream,
) -> proc_macro::TokenStream {
    let name_str = &struct_name.to_string();

    let module_name = string_to_ident("tests_", &name_str);

    proc_macro::TokenStream::from(quote!(
        impl Add for #struct_name {
            type Output = #struct_name;

            fn add(self, rhs: Self) -> #struct_name {
                Self::new(self.get_value().checked_add(rhs.get_value()).unwrap())
            }
        }

        impl Sub for #struct_name {
            type Output = #struct_name;

            fn sub(self, rhs: Self) -> #struct_name {
                Self::new(self.get_value().checked_sub(rhs.get_value()).unwrap())
            }
        }

        impl Mul for #struct_name {
            type Output = #struct_name;

            fn mul(self, rhs: Self) -> #struct_name {
                Self::new(self.get_value().checked_mul(rhs.get_value()).unwrap().checked_div(rhs.get_one()).unwrap())
            }

        }

        impl Div for #struct_name {
            type Output = #struct_name;

            fn div(self, rhs: Self) -> #struct_name {
                Self::new(self.get_value().checked_mul(rhs.get_one()).unwrap().checked_div(rhs.get_value()).unwrap())
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
                let a = #struct_name::new(1);
                let b = #struct_name::new(a.get_one());
                assert_eq!(a * b, #struct_name::new(1));
            }

            #[test]
            fn test_div () {
                let a = #struct_name::new(1);
                let b = #struct_name::new(a.get_one());
                assert_eq!(a / b, #struct_name::new(1));
            }
        }
    ))
}
