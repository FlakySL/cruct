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
            // convert FileFormat to a TokenStream
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
                let cfg_val = {
                    use cruct_shared::{FileFormat};
                    use cruct_shared::source::{ConfigBuilder, ConfigFileSource};
                    #[cfg(feature = "cli")]
                    use cruct_shared::source::ClapSource;

                    let format_opt: Option<FileFormat> = #format_match;  // this expands to Some(FileFormat::Toml)|(None) block

                    let mut builder = ConfigBuilder::new()
                        .add_source(ConfigFileSource::new(#path, format_opt));

                    #[cfg(feature = "cli")]
                    {
                        let matches = cruct_shared::clap::Command::new(env!("CARGO_PKG_NAME"))
                            .get_matches();
                        builder = builder.add_source(ClapSource::new(matches));
                    }

                    builder.load()?
                };

                Self::load_from(&cfg_val)
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
