use quote::quote;

// use crate::utils::string_to_ident;
use crate::DecimalCharacteristics;

pub fn generate_tests(characteristics: DecimalCharacteristics) -> proc_macro::TokenStream {
    let DecimalCharacteristics {
        struct_name,
        ..
    } = characteristics;

    proc_macro::TokenStream::from(quote!(
        impl Test for #struct_name {
            fn mock() -> Result<u8, String> {
                Ok(0u8)
            }
        }
    ))
}
