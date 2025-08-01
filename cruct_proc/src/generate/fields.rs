use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type, TypePath};

use crate::parse::FieldParams;

/// Generates initialization logic for a single configuration field.
/// This includes support for overrides (CLI/env), config file lookup, and
/// default values, and produces the corresponding token stream for inclusion in
/// the derived struct implementation.
pub fn generate_field_initialization(
    field: &FieldParams,
    field_ident: &Ident,
    config_key: &str,
    field_type: &Type,
) -> TokenStream {
    let override_chain = build_override_chain(field);
    let config_lookup = build_config_lookup(field, config_key);
    let is_scalar = is_scalar_type(field_type);

    let parse_logic = if let Some(default_val) = &field.default {
        parse_with_default(field_type, config_key, default_val, &override_chain, &config_lookup)
    } else if is_scalar {
        parse_scalar(field_type, config_key, &override_chain, &config_lookup)
    } else {
        parse_nested(field_type, config_key, &override_chain, &config_lookup)
    };

    quote! { #field_ident: #parse_logic }
}

/// Determines whether the given type is considered a scalar type for parsing
/// purposes. Scalars include primitives, `String`, and `Vec<T>`.
fn is_scalar_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        let name = &path
            .segments
            .last()
            .unwrap()
            .ident
            .to_string()[..];

        return matches!(
            name,
            "bool"
                | "char"
                | "u8"
                | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "i8"
                | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "f32"
                | "f64"
                | "String"
        ) || (name == "Vec"
            && path
                .segments
                .len()
                == 1);
    }

    false
}

/// Builds the override resolution chain for a field, combining CLI and ENV
/// logic.
fn build_override_chain(field: &FieldParams) -> TokenStream {
    let cli = if let Some(flag) = &field.arg_override {
        quote! {
            std::env::args().skip(1)
                .find_map(|arg| {
                    let prefix = concat!("--", #flag, "=");
                    arg.strip_prefix(prefix)
                        .map(|v| cruct_shared::parser::ConfigValue::Value(v.to_string()))
                })
        }
    } else {
        quote! { None }
    };

    let env = if let Some(var) = &field.env_override {
        quote! {
            std::env::var(#var)
                .ok()
                .map(|s| cruct_shared::parser::ConfigValue::Value(s))
        }
    } else {
        quote! { None }
    };

    quote! { #cli.or_else(|| #env) }
}

/// Builds the expression used to look up a field in the configuration map.
/// Supports case-insensitive lookup if configured.
fn build_config_lookup(field: &FieldParams, key: &str) -> TokenStream {
    if field.insensitive {
        quote! {
            map.iter()
               .find(|(k, _)| k.eq_ignore_ascii_case(#key))
               .map(|(_, v)| v.clone())
        }
    } else {
        quote! { map.remove(#key) }
    }
}

/// Generates parsing logic for a field with a default value.
/// Falls back to default if not present in overrides or config.
fn parse_with_default(
    ty: &Type,
    key: &str,
    default_val: &syn::Expr,
    override_chain: &TokenStream,
    config_lookup: &TokenStream,
) -> TokenStream {
    quote! {
        {
            let maybe = #override_chain.or_else(|| #config_lookup);
            if let Some(val) = maybe {
                <#ty as cruct_shared::FromConfigValue>::from_config_value(&val)
                    .map_err(|e| cruct_shared::parser::ParserError::NestedError {
                        section: #key.to_string(), source: Box::new(e)
                    })?
            } else {
                let sec = cruct_shared::ConfigValue::Section(map.clone());
                <#ty as cruct_shared::FromConfigValue>::from_config_value(&sec)
                    .unwrap_or(#default_val)
            }
        }
    }
}

/// Generates parsing logic for scalar fields without a default value.
/// Returns a MissingField or TypeMismatch error if necessary.
fn parse_scalar(
    ty: &Type,
    key: &str,
    override_chain: &TokenStream,
    config_lookup: &TokenStream,
) -> TokenStream {
    quote! {
        {
            let maybe = #override_chain.or_else(|| #config_lookup);
            if let Some(val) = maybe {
                <#ty as cruct_shared::FromConfigValue>::from_config_value(&val)
                    .map_err(|_| cruct_shared::parser::ParserError::TypeMismatch {
                        field: #key.to_string(), expected: stringify!(#ty).into()
                    })?
            } else {
                return Err(cruct_shared::parser::ParserError::MissingField(
                    #key.to_string(),
                ));
            }
        }
    }
}

/// Generates parsing logic for nested structs without a default value.
/// If the key is not found, attempts to use the entire config map as a section.
fn parse_nested(
    ty: &Type,
    key: &str,
    override_chain: &TokenStream,
    config_lookup: &TokenStream,
) -> TokenStream {
    quote! {
        {
            let maybe = #override_chain.or_else(|| #config_lookup);
            if let Some(val) = maybe {
                <#ty as cruct_shared::FromConfigValue>::from_config_value(&val)
                    .map_err(|e| cruct_shared::parser::ParserError::NestedError {
                        section: #key.to_string(), source: Box::new(e)
                    })?
            } else {
                let sec = cruct_shared::ConfigValue::Section(map.clone());
                <#ty as cruct_shared::FromConfigValue>::from_config_value(&sec)
                    .map_err(|e| cruct_shared::parser::ParserError::NestedError {
                        section: #key.to_string(), source: Box::new(e)
                    })?
            }
        }
    }
}
