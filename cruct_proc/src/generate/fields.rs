use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::parse::FieldParams;

pub fn generate_field_initialization(
    field: &FieldParams,
    field_ident: &Ident,
    config_key: &str,
) -> TokenStream {
    let config_lookup = quote! { config.get(#config_key).cloned() };

    let initial_chain = if let Some(env_var) = &field.env_override {
        let env_check = quote! { std::env::var(#env_var).ok() };
        quote! { #env_check.or_else(|| #config_lookup) }
    } else {
        quote! { #config_lookup }
    };

    let raw_value_expr = match &field.default {
        Some(default) => quote! {
            #initial_chain
                .unwrap_or_else(|| #default.to_string())
        },
        None => quote! {
            #initial_chain
                .ok_or_else(|| cruct_shared::ParserError::MissingField(#config_key.to_string()))?
        },
    };

    quote! {
        #field_ident: {
            let raw_value = #raw_value_expr;
            raw_value.parse().map_err(|_| cruct_shared::ParserError::TypeMismatch {
                field: #config_key.to_string(),
                expected: stringify!(#field_ident).to_string()
            })?
        }
    }
}
