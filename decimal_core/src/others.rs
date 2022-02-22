use quote::quote;

use crate::utils::string_to_ident;
use crate::DecimalCharacteristics;

pub fn generate_others(characteristics: DecimalCharacteristics) -> proc_macro::TokenStream {
    let DecimalCharacteristics {
        struct_name,
        underlying_type,
        ..
    } = characteristics;

    let name_str = &struct_name.to_string();

    let module_name = string_to_ident("tests_others", &name_str);

    proc_macro::TokenStream::from(quote!(
        impl<T: Decimal> Others<T> for #struct_name
        where
            T::U: TryInto<#underlying_type>,
        {
            fn mul_up(self, rhs: T) -> #struct_name {
                #struct_name::new(
                    self.get().checked_mul(
                        rhs.get()
                            .try_into()
                            .unwrap_or_else(|_| std::panic!("value of rhs can't fit into underlying type in `MulUp`")),
                    ).unwrap()
                    .checked_add(T::almost_one()).unwrap()
                    .checked_div(T::one()).unwrap()
                    .try_into().unwrap()
                )
            }
        }

        #[cfg(test)]
        pub mod #module_name {
            use super::*;

            #[test]
            fn test_mul_up() {
                let a = #struct_name::new(2);
                let b = #struct_name::new(#struct_name::one());
                assert_eq!(a.mul_up(b), #struct_name::new(2));
            }
        }
    ))
}