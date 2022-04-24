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
    let underlying_str = &underlying_type.to_string();

    let module_name = string_to_ident("tests_others_", &name_str);

    proc_macro::TokenStream::from(quote!(
        impl<T: Decimal> Others<T> for #struct_name
        where
            T::U: TryInto<#underlying_type>,
        {
            fn mul_up(self, rhs: T) -> Self {
                Self::new(
                    self.get()
                        .checked_mul(
                            rhs.get()
                                .try_into()
                                .unwrap_or_else(|_| std::panic!("decimal: rhs value can't fit into `{}` type in {}::mul_up()", #underlying_str, #name_str))
                        )
                        .unwrap_or_else(|| std::panic!("decimal: overflow in method {}::mul_up()", #name_str))
                        .checked_add(T::almost_one())
                        .unwrap_or_else(|| std::panic!("decimal: overflow in method {}::mul_up()", #name_str))
                        .checked_div(T::one())
                        .unwrap_or_else(|| std::panic!("decimal: overflow in method {}::mul_up()", #name_str))
                )
            }

            fn div_up(self, rhs: T) -> Self {
                Self::new(
                    self.get()
                        .checked_mul(T::one())
                        .unwrap_or_else(|| std::panic!("decimal: overflow in method {}::div_up()", #name_str))
                        .checked_add(
                            rhs.get()
                                .try_into()
                                .unwrap_or_else(|_| std::panic!("decimal: rhs value can't fit into `{}` type in {}::div_up()", #underlying_str, #name_str))
                                .checked_sub(#underlying_type::try_from(1u128).unwrap())
                                .unwrap_or_else(|| std::panic!("decimal: overflow in method {}::div_up()", #name_str))
                            )
                        .unwrap_or_else(|| std::panic!("decimal: overflow in method {}::div_up()", #name_str))
                        .checked_div(
                            rhs.get()
                                .try_into()
                                .unwrap_or_else(|_| std::panic!("decimal: rhs value can't fit into `{}` type in {}::div_up()", #underlying_str, #name_str))
                        )
                        .unwrap_or_else(|| std::panic!("decimal: overflow in method {}::div_up()", #name_str))
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
                let a = #struct_name::new(1);
                let b = #struct_name::new(#struct_name::one());
                assert_eq!(a.mul_up(b), a);
            }

            #[test]
            fn test_div_up() {
                let a = #struct_name::new(1);
                let b = #struct_name::new(#struct_name::one());
                assert_eq!(a.div_up(b), a);
            }
        }
    ))
}
