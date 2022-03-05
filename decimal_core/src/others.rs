use quote::quote;

use crate::utils::string_to_ident;
use crate::DecimalCharacteristics;

pub fn generate_others(characteristics: DecimalCharacteristics) -> proc_macro::TokenStream {
    let DecimalCharacteristics {
        struct_name,
        underlying_type,
        scale,
        ..
    } = characteristics;

    let name_str = &struct_name.to_string();

    let module_name = string_to_ident("tests_others_", &name_str);

    proc_macro::TokenStream::from(quote!(
        impl<T: Decimal> Others<T> for #struct_name
        where
            T::U: TryInto<#underlying_type>,
        {
            fn mul_up(self, rhs: T) -> Self {
                Self::new(
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

            fn div_up(self, rhs: T) -> Self {
                Self::new(
                    self.get().checked_mul(T::one()).unwrap()
                    .checked_add(
                        rhs.get()
                            .try_into().unwrap_or_else(|_| std::panic!("value of rhs can't fit into underlying type in `DivUp`"))
                            .checked_sub(#underlying_type::try_from(1u128).unwrap()).unwrap()
                    ).unwrap()
                    .checked_div(
                        rhs.get()
                            .try_into().unwrap_or_else(|_| std::panic!("value of rhs can't fit into underlying type in `DivUp`")),
                    ).unwrap()
                    .try_into().unwrap()
                )
            }

        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if #scale > 0 {
                    write!(
                        f,
                        "{}.{}",
                        self.get().checked_div(Self::one()).unwrap(),
                        self.get().checked_rem(Self::one()).unwrap()
                    )
                } else {
                    write!(f, "{}", self.get())
                }
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

            #[test]
            fn test_div_up() {
                let a = #struct_name::new(2);
                let b = #struct_name::new(#struct_name::one());
                assert_eq!(a.div_up(b), #struct_name::new(2));
            }
        }
    ))
}
