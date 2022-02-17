use quote::quote;

use crate::DecimalCharacteristics;

pub fn generate_base(characteristics: DecimalCharacteristics) -> proc_macro::TokenStream {
    let DecimalCharacteristics {
        struct_name,
        underlying_type,
        scale: parsed_scale,
        field_name,
        ..
    } = characteristics;

    let denominator = 10u128.pow(parsed_scale as u32);
    let almost_denominator = denominator.checked_sub(1).unwrap();

    proc_macro::TokenStream::from(quote!(
        impl Decimal for #struct_name {
            type U = #underlying_type;

            fn scale(&self) -> u8 {
                #parsed_scale
            }

            fn get(&self) -> #underlying_type {
                self.#field_name
            }

            fn new(value: #underlying_type) -> #struct_name {
                let mut created = #struct_name::default();
                created.#field_name = value;
                created
            }

            fn here<T: TryFrom<#underlying_type>>(&self) -> T {
                match T::try_from(self.#field_name) {
                    Ok(v) => v,
                    Err(_) => panic!("could not parse {} to {}", "T", "u8"),
                }
            }

            fn one<T: TryFrom<u128>>() -> T {
                match T::try_from(#denominator) {
                    Ok(v) => v,
                    Err(_) => panic!("denominator wouldn't fit into this type",),
                }
            }

            fn almost_one<T: TryFrom<u128>>() -> T {
                match T::try_from(#almost_denominator) {
                    Ok(v) => v,
                    Err(_) => panic!("denominator wouldn't fit into this type",),
                }
            }
        }
    ))
}
