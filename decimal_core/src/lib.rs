use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

mod base;
mod ops;
mod structs;
mod utils;

use ops::*;
use structs::DecimalCharacteristics;
use utils::string_to_ident;

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

    let characteristics = DecimalCharacteristics {
        struct_name: struct_name.clone(),
        field_name: field_name.clone(),
        underlying_type: underlying_type.clone(),
        scale: parsed_scale,
    };

    let mut result = item.clone();

    result.extend(base::generate_base(characteristics.clone()));
    result.extend(ops::generate_ops(characteristics.clone()));

    result
}
