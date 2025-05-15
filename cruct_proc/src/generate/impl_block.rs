use cruct_shared::FileFormat;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitStr};

use crate::generate::generate_field_initialization;
use crate::parse::{FieldParams, MacroParams, StructField};

pub fn generate_impl_block(
    struct_name: &Ident,
    params: &MacroParams,
    fields: &[StructField],
) -> TokenStream {
    let loader_name = Ident::new(&format!("{}Loader", struct_name), struct_name.span());

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
            let params_ref = field
                .params
                .as_ref()
                .unwrap_or(&default_params);
            generate_field_initialization(params_ref, field_ident, config_key, &field.ty)
        });

    let config_adds = params
        .configs
        .iter()
        .map(|cfg| {
            let path_lit = LitStr::new(&cfg.path, Span::call_site());
            let format_ts = match &cfg.format {
                Some(FileFormat::Json) => quote! { Some(cruct_shared::FileFormat::Json) },
                Some(FileFormat::Toml) => quote! { Some(cruct_shared::FileFormat::Toml) },
                Some(FileFormat::Yml) => quote! { Some(cruct_shared::FileFormat::Yml) },
                None => quote! { None },
            };
            quote! {
                self.builder = self.builder.add_source(
                    cruct_shared::ConfigFileSource::new(#path_lit, #format_ts)
                );
            }
        });

    quote! {
        pub struct #loader_name {
            builder: cruct_shared::ConfigBuilder,
        }

        impl #struct_name {
            pub fn loader() -> #loader_name {
                #loader_name {
                    builder: cruct_shared::ConfigBuilder::new()
                }
            }
        }

        impl #loader_name {
            pub fn with_cli(mut self, priority: u8) -> Self {
                self.builder = self.builder.add_source(cruct_shared::CliSource::new(priority));
                self
            }

            pub fn with_config(mut self) -> Self {
                #(#config_adds)*
                self
            }

            pub fn load(self) -> Result<#struct_name, cruct_shared::ParserError> {
                let cfg_val = self.builder.load()?;
                #struct_name::load_from(&cfg_val)
            }
        }

        impl #struct_name {
            pub fn load_from(
                config: &cruct_shared::ConfigValue
            ) -> Result<Self, cruct_shared::ParserError> {
                use cruct_shared::{ConfigValue, ParserError};

                let ConfigValue::Section(section) = config else {
                    return Err(ParserError::TypeMismatch {
                        field: "root".into(),
                        expected: "section".into(),
                    });
                };

                Ok(Self {
                    #(#field_inits),*
                })
            }
        }
    }
}
