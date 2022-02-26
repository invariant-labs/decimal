use quote::quote;

use crate::utils::string_to_ident;
use crate::DecimalCharacteristics;

pub fn generate_by_number(characteristics: DecimalCharacteristics) -> proc_macro::TokenStream {
    let DecimalCharacteristics {
        struct_name,
        big_type,
        ..
    } = characteristics;

    let name_str = &struct_name.to_string();

    let module_name = string_to_ident("tests_by_number_", &name_str);

    proc_macro::TokenStream::from(quote!(
        impl ByNumber<#big_type> for #struct_name
        {
            fn big_div_up_by_number(self, rhs: #big_type) -> #struct_name {
                #struct_name::new(
                    #big_type::try_from(self.get()).unwrap()
                        .checked_mul(
                            Self::one()
                        ).unwrap()
                        .checked_add(
                            rhs.checked_sub(#big_type::from(1u128)).unwrap()
                        ).unwrap()
                        .checked_div(rhs).unwrap()
                        .try_into().unwrap()
                )
            }
        }

        #[cfg(test)]
        pub mod #module_name {
            use super::*;

            #[test]
            fn test_big_div_up_by_number () {
                let a = #struct_name::new(2);
                let b: #big_type = #struct_name::one();
                assert_eq!(a.big_div_up_by_number(b), #struct_name::new(2));
            }
        }
    ))
}
