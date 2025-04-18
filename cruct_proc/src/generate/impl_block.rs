use cruct_shared::FileFormat;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::generate::generate_field_initialization;
use crate::parse::{FieldParams, MacroParams, StructField};

pub fn generate_impl_block(
    struct_name: &Ident,
    params: &MacroParams,
    fields: &[StructField],
) -> TokenStream {
    let path = &params.path;
    let format_match = match &params.format {
        Some(file_format) => {
            let variant = match file_format {
                FileFormat::Json => quote! { cruct_shared::FileFormat::Json },
                FileFormat::Toml => quote! { cruct_shared::FileFormat::Toml },
                FileFormat::Yml => quote! { cruct_shared::FileFormat::Yml },
            };

            quote! { #variant }
        },
        None => quote! { /* auto-detect via extension */
            {
                use cruct_shared::{get_parser_by_extension, ParserError};
                let ext = std::path::Path::new(#path)
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_lowercase());
                let parser = get_parser_by_extension(ext.as_deref().unwrap_or_default())
                    .ok_or_else(|| ParserError::InvalidFileFormat(
                        ext.unwrap_or_else(|| "unknown".into())
                    ))?;

                parser.format()
            }
        },
    };

    let field_inits = fields
        .iter()
        .map(|field| {
            let field_ident = &field.ident;
            let config_key = field
                .params
                .as_ref()
                .and_then(|p| {
                    p.name
                        .as_ref()
                })
                .unwrap_or(&field.name);

            let default_params = FieldParams::default();
            let params_ref: &FieldParams = field
                .params
                .as_ref()
                .unwrap_or(&default_params);

            generate_field_initialization(params_ref, field_ident, config_key, &field.ty)
        });

    quote! {
        impl #struct_name {
            pub fn load() -> Result<Self, cruct_shared::ParserError> {
                use cruct_shared::{FileFormat, ParserError, get_parser_by_extension};

                let format: FileFormat = #format_match;
                let parser = get_parser_by_extension(&format.to_string())
                    .ok_or_else(|| ParserError::InvalidFileFormat(format.to_string()))?;
                let config = parser.load(#path)?;

                Self::load_from(&config)
            }

            pub fn load_from(config: &cruct_shared::ConfigValue) -> Result<Self, cruct_shared::ParserError> {
                use cruct_shared::{ConfigValue, ParserError};

                let ConfigValue::Section(section) = config else {
                    return Err(ParserError::TypeMismatch {
                        field: "root".into(),
                        expected: "section".into()
                    });
                };

                Ok(Self {
                    #(#field_inits),*
                })
            }
        }
    }
}
