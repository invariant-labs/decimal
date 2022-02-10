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

    let field_name = match first_field.ident.clone() {
        Some(ident) => quote! {#ident},
        None => quote! {0},
    };

    let struct_name = decimal_struct.ident;

    let get_scale_definition = quote!(

        impl Decimal<#underlying_type> for #struct_name {
            fn get_scale(&self) -> u8{
                #parsed_scale
            }

            fn get_value(&self) -> #underlying_type {
                self.#field_name
            }


        }

        // this should be an impl of From but getting errors :(
        impl #struct_name {
            fn here<T: TryFrom<#underlying_type>>(&self) -> T {
                match T::try_from(self.#field_name) {
                    Ok(v) => v,
                    Err(_) => panic!("could not parse {} to {}", "T", "u8"),
                }
            }
        }

    );

    println!(
        "CCC {}",
        quote!(
                    // this should be an impl of Into but getting errors :(
                        impl #struct_name {
                            fn here<T: TryFrom<#underlying_type>(&self) -> T {

                                universal_into::<T, #underlying_type>(self.#field_name)

                                // match self.#field_name.try_into() {
                                //     Ok(v) => v,
                                //     Err(_) => panic!("could not parse {} to {}", "T", "u8"),
                                // }
                            }
                        }
        )
        .to_token_stream()
        .to_string()
    );

    let mut result = item.clone();
    result.extend(proc_macro::TokenStream::from(quote! {
        #get_scale_definition
    }));

    result
}
