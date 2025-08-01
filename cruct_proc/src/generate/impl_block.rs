use cruct_shared::FileFormat;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitStr};

use crate::generate::generate_field_initialization;
use crate::parse::{FieldParams, MacroParams, StructField};

/// Generate the implementation block for a struct annotated with `#[cruct]`.
///
/// This includes:
///     1. A `Loader` type with builder methods (`with_cli`, `with_config`).
///     2. A `load_from` method that deserializes a `ConfigValue::Section` into
///        your struct.
///     3. An implementation of `FromConfigValue` so your struct can be used
///        anywhere a nested struct is expected (for flat-nesting support).
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
                #[cfg(feature = "json")]
                Some(FileFormat::Json) => quote! { Some(cruct_shared::FileFormat::Json) },

                #[cfg(feature = "toml")]
                Some(FileFormat::Toml) => quote! { Some(cruct_shared::FileFormat::Toml) },

                #[cfg(feature = "yaml")]
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
        /// Builder type for loading a `<#struct_name>` from CLI, ENV, and config files.
        pub struct #loader_name {
            builder: cruct_shared::ConfigBuilder,
        }

        impl #struct_name {
            /// Create a new loader for this struct.
            pub fn loader() -> #loader_name {
                #loader_name {
                    builder: cruct_shared::ConfigBuilder::new()
                }
            }
        }

        impl #loader_name {
            /// Add a CLI source with the given priority.
            pub fn with_cli(mut self, priority: u8) -> Self {
                self.builder = self.builder.add_source(
                    cruct_shared::CliSource::new(priority)
                );
                self
            }

            /// Add all `load_config(...)` sources specified on the struct.
            pub fn with_config(mut self) -> Self {
                #(#config_adds)*
                self
            }

            /// Load and merge all sources, then deserialize into the target struct.
            ///
            /// # Errors
            /// Returns a `ParserError` if any required field is missing, or
            /// if any parsing or nested error occurs.
            pub fn load(self) -> Result<#struct_name, cruct_shared::ParserError> {
                let cfg_val = self.builder.load()?;
                #struct_name::load_from(&cfg_val)
            }
        }

        impl #struct_name {
            /// Deserialize from a `ConfigValue` (must be a `Section`).
            ///
            /// This is called internally by `load`, or when flattening nested structs.
            ///
            /// # Errors
            /// - `TypeMismatch` if the top-level value is not a section.
            /// - Nested errors for each field via `NestedError`.
            pub fn load_from(
                config: &cruct_shared::ConfigValue
            ) -> Result<Self, cruct_shared::ParserError> {
                use cruct_shared::{ConfigValue, ParserError};

                // Ensure the provided `ConfigValue` is a section, cloning the map
                let mut map = match config {
                    ConfigValue::Section(m) => m.clone(),
                    _ => {
                        return Err(ParserError::TypeMismatch {
                            field: "root".into(),
                            expected: "section".into(),
                        })
                    }
                };

                Ok(Self {
                    #(#field_inits),*
                })
            }
        }

        /// Allow this struct itself to be treated as a nested config value.
        /// This supports flat-nested loading when a struct appears inside another.
        impl cruct_shared::FromConfigValue for #struct_name {
            fn from_config_value(
                value: &cruct_shared::ConfigValue
            ) -> Result<Self, cruct_shared::ParserError> {
                #struct_name::load_from(value)
            }
        }
    }
}
