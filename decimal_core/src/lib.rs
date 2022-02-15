use quote::{quote, ToTokens};
use syn::parse_macro_input;

mod base;
mod ops;
mod structs;
mod utils;

use structs::DecimalCharacteristics;

use crate::utils::string_to_ident;

#[proc_macro_attribute]
pub fn decimal(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let param_string = attr.to_string().clone();
    let params = param_string.split(",").collect::<Vec<_>>();

    // println!("params: {:?}", params[0].parse::<u8>().unwrap());

    let parsed_scale = match params[0].parse::<u8>() {
        Ok(scale) => scale,
        Err(_) => 0,
    };
    println!("here");

    let big_type = match params.len() {
        1 => string_to_ident("", "U256"),
        2 => string_to_ident("", params[1].trim()),
        _ => panic!("decimal: invalid number of parameters"),
    };

    assert!(parsed_scale <= 38, "scale too big");

    let k = item.clone();
    let decimal_struct = parse_macro_input!(k as syn::ItemStruct);

    let fields = decimal_struct.fields;
    let first_field = fields.iter().next().unwrap();

    let underlying_type =
        string_to_ident("", first_field.ty.to_token_stream().to_string().as_str());

    let field_name = match first_field.ident.clone() {
        Some(ident) => quote! {#ident},
        None => quote! {0},
    };

    let struct_name = decimal_struct.ident;

    let characteristics = DecimalCharacteristics {
        struct_name: struct_name.clone(),
        field_name: field_name.clone(),
        underlying_type: underlying_type.clone(),
        big_type: big_type.clone(),
        scale: parsed_scale,
    };

    let mut result = item.clone();

    result.extend(base::generate_base(characteristics.clone()));
    result.extend(ops::generate_ops(characteristics.clone()));

    result
}
