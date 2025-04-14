//! Procedural macro implementation for configuration loading
//!
//! This crate provides the `#[cruct]` attribute macro that generates
//! configuration loading implementation for structs.

use cruct_shared::FileFormat;
use fields::{StructField, remove_field_attrs};
use parameters::MacroParameters;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

mod fields;
mod parameters;

/// Main procedural macro attribute
///
/// # Usage
/// ```ignore
/// #[cruct(path = "config.toml", format = "Toml")]
/// struct Config {
///     #[field(name = "http_port")]
///     port: u16,
/// }
/// ```
#[proc_macro_attribute]
pub fn cruct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let options = parse_macro_input!(attr as MacroParameters);
    let mut item = parse_macro_input!(item as ItemStruct);

    let fields = match StructField::from_struct(&item) {
        Ok(fields) => fields,
        Err(err) => return err.to_compile_error().into(),
    };

    remove_field_attrs(&mut item);

    let struct_name = &item.ident;
    let path = &options.path;

    let field_inits = fields.iter().map(|field| {
        let field_ident = syn::Ident::new(&field.name, proc_macro2::Span::call_site());
        let config_key = field
            .parameters
            .as_ref()
            .and_then(|p| p.name.as_ref())
            .unwrap_or(&field.name);

        match &field.parameters.as_ref().and_then(|p| p.default.as_ref()) {
            Some(default) => {
                quote! {
                    #field_ident: {
                        let raw_value = config.get(#config_key)
                            .map(|s| s.as_str())
                            .unwrap_or(#default);

                        raw_value
                            .parse()
                            .map_err(|_| cruct_shared::ParserError::TypeMismatch {
                                field: #config_key.to_string(),
                                expected: stringify!(#field_ident).to_string()
                            })?
                    }
                }
            }
            None => {
                quote! {
                    #field_ident: config
                        .get(#config_key)
                        .ok_or_else(|| cruct_shared::ParserError::MissingField(
                            #config_key.to_string()
                        ))?
                        .parse()
                        .map_err(|_| cruct_shared::ParserError::TypeMismatch {
                            field: #config_key.to_string(),
                            expected: stringify!(#field_ident).to_string()
                        })?
                }
            }
        }
    });

    let format_match = match &options.format {
        Some(file_format) => {
            let variant = match file_format {
                FileFormat::Json => quote! { cruct_shared::FileFormat::Json },
                FileFormat::Toml => quote! { cruct_shared::FileFormat::Toml },
                FileFormat::Yml => quote! { cruct_shared::FileFormat::Yml },
            };

            quote! { #variant }
        }
        None => quote! {
            {
                use cruct_shared::{default_registry, ParserError};

                let ext = std::path::Path::new(#path)
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_lowercase());

                let registry = default_registry();
                let parser = registry
                    .get_by_extension(ext.as_deref().unwrap_or_default())
                    .ok_or_else(|| ParserError::InvalidFileFormat(
                        ext.unwrap_or_else(|| "unknown".into())
                    ))?;

                parser.format()
            }
        },
    };

    let expanded = quote! {
        #item

        impl #struct_name {
            pub fn load() -> Result<Self, cruct_shared::ParserError> {
                use cruct_shared::{FileFormat, ParserError};

                let format: FileFormat = #format_match;
                let registry = cruct_shared::parser::default_registry();

                let parser = registry
                    .get_by_extension(&format.to_string())
                    .ok_or_else(|| ParserError::InvalidFileFormat(
                        format.to_string()
                    ))?;

                let config = parser.load(#path)?;

                Ok(Self {
                    #(#field_inits),*
                })
            }
        }
    };

    expanded.into()
}
