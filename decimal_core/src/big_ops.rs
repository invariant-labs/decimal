use quote::quote;

use crate::utils::string_to_ident;
use crate::DecimalCharacteristics;

pub fn generate_ops(characteristics: DecimalCharacteristics) -> proc_macro::TokenStream {
    let DecimalCharacteristics {
        struct_name,
        underlying_type,
        big_type,
        ..
    } = characteristics;

    let name_str = &struct_name.to_string();

    let module_name = string_to_ident("tests_", &name_str);

    proc_macro::TokenStream::from(quote!(
        // impl<T: Decimal> BigOps<T> for #struct_name
        //     T::U: TryInto<#underlying_type>,
        // {

        //     fn big_mul(self, rhs: T) -> #struct_name {
        //         #big_type::try_from(self.get()).unwrap()
        //             .checked_mul(
        //                 rhs.get()
        //                     .try_into().unwrap()
        //                 )
        //                 .unwrap().checked_div(T::one()).unwrap().into()

        //         #struct_name::new(
        //             self.get()
        //                 .checked_mul(
        //                     rhs.get()
        //                         .try_into()
        //                         .unwrap_or_else(|_| panic!("could not parse")),
        //                 )
        //                 .unwrap()
        //                 .checked_div(T::one())
        //                 .unwrap(),
        //         )
        //     }

        //     fn big_div(self, rhs: T) -> #struct_name {
        //         #struct_name::new(
        //             self.get()
        //                 .checked_mul(T::one())
        //                 .unwrap()
        //                 .checked_div(
        //                     rhs.get()
        //                         .try_into()
        //                         .unwrap_or_else(|_| panic!("could not parse")),
        //                 )
        //                 .unwrap(),
        //         )
        //     }
        // }

        // #[cfg(test)]
        // pub mod #module_name {
        //     use super::*;

        //     #[test]
        //     fn test_add () {
        //         let a = #struct_name::new(1);
        //         let b = #struct_name::new(1);
        //         assert_eq!(a + b, #struct_name::new(2));
        //     }

        //     #[test]
        //     fn test_sub () {
        //         let a = #struct_name::new(1);
        //         let b = #struct_name::new(1);
        //         assert_eq!(a - b, #struct_name::new(0));
        //     }

        //     #[test]
        //     fn test_mul () {
        //         let a = #struct_name::new(2);
        //         let b = #struct_name::new(#struct_name::one());
        //         assert_eq!(a * b, #struct_name::new(2));
        //     }

        //     #[test]
        //     fn test_div () {
        //         let a = #struct_name::new(2);
        //         let b = #struct_name::new(#struct_name::one());
        //         assert_eq!(a / b, #struct_name::new(2));
        //     }
        // }
    ))
}
