use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

use crate::parse::FieldParams;

/// Generates code to initialize a field based on configuration sources.
///
/// This function constructs a token stream that initializes a field by checking
/// values from multiple prioritized configuration sources: command line
/// arguments, environment variables, and configuration files. If a value is not
/// found in any of these sources, it either uses a default value (if provided)
/// or throws an error for missing configuration.
///
/// * `field`: Metadata about the field being initialized, including its
///   override options, default value, and case sensitivity.
/// * `field_ident`: The identifier for the field in the generated code.
/// * `config_key`: The key used to look up the field's value in the
///   configuration sources.
/// * `field_type`: The expected type of the field, which is used for parsing
///   the configuration value.
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

    let arg_check = field
        .arg_override
        .as_ref()
        .map(|flag| {
            quote! {
                std::env::args()
                    .skip(1)
                    .find_map(|arg| {
                        let prefix = concat!("--", #flag, "=");

                        arg.strip_prefix(prefix)
                           .map(|v| cruct_shared::parser::ConfigValue::Value(v.to_string()))
                    })
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

    // Priority: CLI arg override → env_override → config file lookup
    let value_source = quote! {
        #arg_check
            .or_else(|| #env_check)
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
