use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

// use parse_decimal::{parse_struct, StructType};

use syn::parse_macro_input;

mod ops;
mod utils;

use ops::*;

#[proc_macro_attribute]
pub fn decimal(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_scale = match attr.to_string().parse::<u8>() {
        Ok(scale) => scale,
        Err(_) => panic!("print_macro: invalid scale"),
    };

    assert!(parsed_scale <= 38, "scale too big");

    let k = item.clone();
    let decimal_struct = parse_macro_input!(k as syn::ItemStruct);

    let fields = decimal_struct.fields;
    let first_field = fields.iter().next().unwrap();

    let underlying_type = first_field.ty.to_token_stream();

    let field_name = match first_field.ident.clone() {
        Some(ident) => quote! {#ident},
        None => quote! {0},
    };

    let struct_name = decimal_struct.ident;
    let denominator = 10u128.pow(parsed_scale as u32);

    let struct_implementation = quote!(

        impl Decimal<#underlying_type> for #struct_name {
            fn get_scale(&self) -> u8 {
                #parsed_scale
            }

            fn get_value(&self) -> #underlying_type {
                self.#field_name.into()
            }

            fn get_one<T: TryFrom<u128>>(&self) -> T {
                match T::try_from(#denominator) {
                    Ok(v) => v,
                    Err(_) => panic!("could get one from a decimal",),
                }
            }
        }

        // this should be an impl of From but getting errors :(
        impl #struct_name {
            fn new(value: #underlying_type) -> #struct_name {
                let mut created = #struct_name::default();
                created.#field_name = value;
                created
            }

            fn one(value: #underlying_type) -> #struct_name {
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
        }

    );

    let mut result = item.clone();
    result.extend(proc_macro::TokenStream::from(quote! {
        #struct_implementation
    }));

    result.extend(generate_ops(struct_name));

    result
}
