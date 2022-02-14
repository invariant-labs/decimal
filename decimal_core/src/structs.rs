use proc_macro2::TokenStream;

pub struct DecimalCharacteristics {
    pub struct_name: syn::Ident,
    pub first_field: TokenStream, // cannot be Ident because of tuple structs
    pub underlying_type: TokenStream,
    pub scale: u8,
}
