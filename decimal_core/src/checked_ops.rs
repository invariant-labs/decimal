use quote::quote;

use crate::utils::string_to_ident;
use crate::DecimalCharacteristics;

pub fn generate_checked_ops(characteristics: DecimalCharacteristics) -> proc_macro::TokenStream {
    let DecimalCharacteristics {
        struct_name,
        underlying_type,
        scale,
        ..
    } = characteristics;

    let name_str = &struct_name.to_string();
    let underlying_str = &underlying_type.to_string();

    let module_name = string_to_ident("tests_checked_ops_", &name_str);

    proc_macro::TokenStream::from(quote!(
        impl CheckedOps for #struct_name {
            fn checked_add(self, rhs: Self) -> std::result::Result<Self, String> {
                Ok(Self::new(
                    self.get().checked_add(rhs.get())
                    .ok_or_else(|| "decimal: (self + rhs) additional overflow")?
                ))
            }
        }

        // impl CheckedOps for #struct_name {
        //     fn checked_sub(self, rhs: Self) -> std::result::Result<Self, String> {
        //         Ok(Self::new(
        //             self.get().checked_sub(rhs.get())
        //             .ok_or_else(|| "decimal: (self - rhs) subtraction underflow")?
        //         ))
        //     }
        // }

        #[cfg(test)]
        pub mod #module_name {
            use super::*;

            #[test]
            fn test_checked_add() {
                let a = #struct_name::new(24);
                let b = #struct_name::new(11);

                assert_eq!(a.checked_add(b), Ok(#struct_name::new(35)));
            }
        }
    ))
}
