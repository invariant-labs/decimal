use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn decimal(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut c = item.clone();

    let parsed_scale = match attr.to_string().parse::<u8>() {
        Ok(scale) => scale,
        Err(_) => panic!("print_macro: invalid scale"),
    };

    let scale_definition = quote!(pub const SCALE: u8 = #parsed_scale;);

    let get_scale_definition = quote!(

        impl D {
            fn get_scale(&self) -> u8{
                SCALE
            }
        }

    );

    c.extend(proc_macro::TokenStream::from(quote! {
        #scale_definition
        #get_scale_definition
    }));

    c
}
