use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

pub fn generate_ops(
    struct_name: Ident,
    // field_name: Ident,
    // underlying_type: TokenStream,
) -> TokenStream {
    proc_macro::TokenStream::from(quote!(
        impl Add for #struct_name {
            type Output = #struct_name;

            fn add(self, rhs: Self) -> #struct_name {
                Self::new(self.get_value().checked_add(rhs.get_value()).unwrap())
            }
        }
    ))
}
