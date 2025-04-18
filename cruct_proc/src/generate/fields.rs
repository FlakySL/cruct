use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

use crate::parse::FieldParams;

pub fn generate_field_initialization(
    field: &FieldParams,
    field_ident: &Ident,
    config_key: &str,
    field_type: &Type,
) -> TokenStream {
    let env_check = field
        .env_override
        .as_ref()
        .map(|env_var| {
            quote! {
                std::env::var(#env_var)
                    .ok()
                    .map(|s| cruct_shared::parser::ConfigValue::Value(s))
            }
        })
        .unwrap_or_else(|| quote! { None });

    let config_lookup = if field.insensitive {
        quote! {
            section.iter()
                .find(|(k, _)| k.eq_ignore_ascii_case(#config_key))
                .map(|(_, v)| v.clone())
        }
    } else {
        quote! {
            section.get(#config_key).cloned()
        }
    };

    let value_source = quote! {
        #env_check
            .or_else(|| #config_lookup)
    };

    let value_parsing = match &field.default {
        Some(default) => quote! {
            match #value_source {
                Some(config_value) => {
                    <#field_type as cruct_shared::FromConfigValue>::from_config_value(&config_value)
                        .map_err(|e| cruct_shared::parser::ParserError::NestedError {
                            section: #config_key.to_string(),
                            source: Box::new(e),
                        })?
                }
                None => #default
            }
        },
        None => quote! {
            {
                let config_value = #value_source
                        .ok_or_else(|| cruct_shared::parser::ParserError::MissingField(#config_key.to_string()))?;

                <#field_type as cruct_shared::FromConfigValue>::from_config_value(&config_value)
                    .map_err(|e| cruct_shared::parser::ParserError::NestedError {
                        section: #config_key.to_string(),
                        source: Box::new(e),
                    })?
            }
        },
    };

    quote! {
        #field_ident: {
            #value_parsing
        }
    }
}
