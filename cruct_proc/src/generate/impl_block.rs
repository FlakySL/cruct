use cruct_shared::FileFormat;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::generate::generate_field_initialization;
use crate::parse::{FieldParams, MacroParams, StructField};

pub fn generate_impl_block(
    struct_name: &Ident,
    params: &MacroParams,
    fields: &[StructField],
) -> TokenStream {
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

    let mut sources = Vec::new();
    for cfg in &params.configs {
        let path_lit = syn::LitStr::new(&cfg.path, Span::call_site());
        let format_ts = match &cfg.format {
            Some(file_format) => {
                let variant = match file_format {
                    FileFormat::Json => quote! { cruct_shared::FileFormat::Json },
                    FileFormat::Toml => quote! { cruct_shared::FileFormat::Toml },
                    FileFormat::Yml => quote! { cruct_shared::FileFormat::Yml },
                };

                quote! { Some(#variant) }
            },
            // auto-detect via extension - move to the shared module
            None => quote! { None },
        };

        sources.push(quote! {
            builder = builder.add_source(cruct_shared::ConfigFileSource::new(#path_lit, #format_ts));
        });
    }

    quote! {
        impl #struct_name {
            pub fn load() -> Result<Self, cruct_shared::ParserError> {
                use cruct_shared::{ConfigBuilder, ConfigFileSource};
                #[cfg(feature = "clap")]
                use cruct_shared::ClapSource;
                let mut builder = ConfigBuilder::new();

                #(#sources)*

                #[cfg(feature = "clap")]
                {
                    let matches = cruct_shared::clap::Command::new(env!("CARGO_PKG_NAME")).get_matches();
                    builder = builder.add_source(ClapSource::new(matches));
                }

                let cfg_val = builder.load()?;
                Self::load_from(&cfg_val)
            }

            pub fn load_from(config: &cruct_shared::ConfigValue) -> Result<Self, cruct_shared::ParserError> {
                use cruct_shared::{ConfigValue, ParserError};

                let ConfigValue::Section(section) = config else {
                    return Err(ParserError::TypeMismatch { field: "root".into(), expected: "section".into() });
                };

                Ok(Self {
                    #(#field_inits),*
                })
            }
        }
    }
}
