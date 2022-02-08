use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro_attribute]
pub fn decimal(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut c = item.clone();

    let parsed_scale = match attr.to_string().parse::<u8>() {
        Ok(scale) => scale,
        Err(_) => panic!("print_macro: invalid scale"),
    };

    let scale_definition =
        TokenStream::from_str(&format!("pub const SCALE: u8 = {};", parsed_scale)).unwrap();

    c.extend(scale_definition);
    c.extend(
        TokenStream::from_str(
            "impl D {
                fn get_scale(&self) -> u8{
                    SCALE
                }
            }",
        )
        .unwrap(),
    );

    c
}
