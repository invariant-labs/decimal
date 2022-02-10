use proc_macro::TokenStream;
use quote::{quote, ToTokens};

// use parse_decimal::{parse_struct, StructType};

use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn decimal(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsed_scale = match attr.to_string().parse::<u8>() {
        Ok(scale) => scale,
        Err(_) => panic!("print_macro: invalid scale"),
    };

    let k = item.clone();
    let decimal_struct = parse_macro_input!(k as syn::ItemStruct);

    let fields = decimal_struct.fields;
    let first_field = fields.iter().next().unwrap();

    let underlying_type = first_field.ty.to_token_stream();

    let struct_name = decimal_struct.ident;

    let scale_definition = quote!(pub const SCALE: u8 = #parsed_scale;);
    let get_scale_definition = quote!(

        impl #struct_name {
            fn get_scale(&self) -> u8{
                SCALE
            }

            fn get_value(&self) -> #underlying_type {
                self.0
            }
        }


    );

    let mut result = item.clone();
    result.extend(proc_macro::TokenStream::from(quote! {
        #scale_definition
        #get_scale_definition
    }));

    result
}
