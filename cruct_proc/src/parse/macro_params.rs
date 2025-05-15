use std::cmp::Reverse;

use cruct_shared::FileFormat;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Error as SynError, Expr, ExprLit, Lit, Meta, MetaNameValue, Result as SynResult, Token};

use super::ParameterError;

#[derive(Default)]
pub struct LoadConfig {
    /// A glob of the path that defines where the macro should be looking for
    /// that configuration file.
    ///
    /// **The query can only return one file**
    pub path: String,

    /// Which is the file format that should be used to parse the configuration
    /// file.
    pub format: Option<FileFormat>,

    /// A priority for the configuration file. The lower the number, the
    /// higher the priority.
    pub priority: Option<u8>,
}

/// This struct represents a parsed version of the `cruct` macro parameters.
pub struct MacroParams {
    /// A vector of `LoadConfig` structs, each representing a configuration
    /// file to be loaded.
    pub configs: Vec<LoadConfig>,
}

impl Parse for MacroParams {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut configs = Vec::new();

        // parse zero or more load_config(...) entries, separated by commas
        while !input.is_empty() {
            // parse one Meta, expecting a list: load_config(...)
            let meta = input.parse::<Meta>()?;
            match meta {
                Meta::List(list)
                    if list
                        .path
                        .is_ident("load_config") =>
                {
                    // parse inner args as name=value pairs
                    let pairs: Punctuated<MetaNameValue, Token![,]> =
                        list.parse_args_with(Punctuated::parse_terminated)?;

                    let mut cfg = LoadConfig::default();
                    for name_value in pairs {
                        let key = name_value
                            .path
                            .get_ident()
                            .unwrap()
                            .to_string();

                        match key.as_str() {
                            "path" => match &name_value.value {
                                Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }) => {
                                    cfg.path = lit.value();
                                },
                                other => {
                                    return Err(SynError::new_spanned(
                                        other,
                                        ParameterError::InvalidType {
                                            name: "path".to_string(),
                                            expected: "String".to_string(),
                                            found: other
                                                .to_token_stream()
                                                .to_string(),
                                        },
                                    ));
                                },
                            },
                            "format" => match &name_value.value {
                                Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }) => {
                                    cfg.format = Some(
                                        lit.value()
                                            .parse::<FileFormat>()
                                            .map_err(|e| {
                                                SynError::new(
                                                    lit.span(),
                                                    format!("invalid file format: {}", e),
                                                )
                                            })?,
                                    );
                                },
                                other => {
                                    return Err(SynError::new_spanned(
                                        other,
                                        ParameterError::InvalidType {
                                            name: "format".to_string(),
                                            expected: "String".to_string(),
                                            found: other
                                                .to_token_stream()
                                                .to_string(),
                                        },
                                    ));
                                },
                            },
                            "priority" => match &name_value.value {
                                Expr::Lit(ExprLit { lit: Lit::Int(int_lit), .. }) => {
                                    cfg.priority = Some(int_lit.base10_parse()?);
                                },
                                other => {
                                    return Err(SynError::new_spanned(
                                        other,
                                        ParameterError::InvalidType {
                                            name: "priority".to_string(),
                                            expected: "Integer".to_string(),
                                            found: other
                                                .to_token_stream()
                                                .to_string(),
                                        },
                                    ));
                                },
                            },

                            other => {
                                return Err(SynError::new_spanned(
                                    name_value.path,
                                    format!("unknown key '{}' in load_config", other),
                                ));
                            },
                        }
                    }

                    if cfg
                        .path
                        .is_empty()
                    {
                        return Err(SynError::new_spanned(
                            list,
                            ParameterError::MissingRequired { name: "path".to_string() },
                        ));
                    }

                    configs.push(cfg);

                    // consume an optional trailing comma
                    let _ = input.parse::<Token![,]>();
                },

                other => {
                    return Err(SynError::new_spanned(
                        other,
                        "expected `load_config(path = ..., format = ..., priority = ...)`",
                    ));
                },
            }
        }

        // sort by priority (descending)
        configs.sort_by_key(|c| Reverse(c.priority));

        Ok(MacroParams { configs })
    }
}
