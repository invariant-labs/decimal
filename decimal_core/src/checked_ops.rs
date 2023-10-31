use alloc::string::ToString;
use quote::quote;

use crate::utils::string_to_ident;
use crate::DecimalCharacteristics;

pub fn generate_checked_ops(characteristics: DecimalCharacteristics) -> proc_macro::TokenStream {
    let DecimalCharacteristics { struct_name, .. } = characteristics;

    let name_str = &struct_name.to_string();
    let module_name = string_to_ident("tests_checked_ops_", &name_str);

    proc_macro::TokenStream::from(quote!(
        impl CheckedOps for #struct_name {
            fn checked_add(self, rhs: Self) -> core::result::Result<Self, alloc::string::String> {
                Ok(Self::new(
                    self.get().checked_add(rhs.get())
                    .ok_or_else(|| "checked_add: (self + rhs) additional overflow")?
                ))
            }

            fn checked_sub(self, rhs: Self) -> core::result::Result<Self, alloc::string::String> {
                Ok(Self::new(
                    self.get().checked_sub(rhs.get())
                    .ok_or_else(|| "checked_sub: (self - rhs) subtraction underflow")?
                ))
            }

            fn checked_div(self, rhs: Self) -> core::result::Result<Self, alloc::string::String> {
                Ok(Self::new(
                        self.get()
                        .checked_mul(Self::one())
                        .ok_or_else(|| "checked_div: (self * Self::one()) multiplication overflow")?
                        .checked_div(rhs.get())
                        .ok_or_else(|| "checked_div: ((self * Self::one()) / rhs) division by zero")?
                    )
                )
            }
        }




        #[cfg(test)]
        pub mod #module_name {
            use super::*;

            #[test]
            fn test_checked_add() {
                let a = #struct_name::new(24);
                let b = #struct_name::new(11);

                assert_eq!(a.checked_add(b), Ok(#struct_name::new(35)));
            }

            #[test]
            fn test_overflow_checked_add() {
                let max = #struct_name::max_instance();
                let result = max.checked_add(#struct_name::new(1));

                assert_eq!(result, Err("checked_add: (self + rhs) additional overflow".to_string()));
            }

            #[test]
            fn test_checked_sub() {
                let a = #struct_name::new(35);
                let b = #struct_name::new(11);

                assert_eq!(a.checked_sub(b), Ok(#struct_name::new(24)));
            }

            #[test]
            fn test_checked_div() {
                let a = #struct_name::new(2);
                let b = #struct_name::new(#struct_name::one());
                assert_eq!(a.checked_div(b), Ok(#struct_name::new(2)));
            }

            #[test]
            fn test_0_checked_div() {
                let a = #struct_name::new(47);
                let b = #struct_name::new(0);
                let result = a.checked_div(b);
                assert!(result.is_err());
            }

            #[test]
            fn test_underflow_checked_sub() {
                let min = #struct_name::new(0);
                let result = min.checked_sub(#struct_name::new(1));

                assert_eq!(result, Err("checked_sub: (self - rhs) subtraction underflow".to_string()));
            }
        }
    ))
}
