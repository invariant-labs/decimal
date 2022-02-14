use proc_macro2::TokenStream;

#[derive(Debug, Clone)]
pub struct DecimalCharacteristics {
    pub struct_name: syn::Ident,
    pub field_name: TokenStream, // cannot be Ident because of tuple structs
    pub underlying_type: TokenStream,
    pub scale: u8,
}
