use cruct_shared::FileFormat;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Error as SynError, Expr, ExprLit, Lit, MetaNameValue, Result as SynResult, Token};
use thiserror::Error;

/// This enum is an error representation
/// for parameter parsing. It implements
/// Display for error descriptions.
#[derive(Error, Debug)]
pub enum ParameterError {
    /// This error is used when a required
    /// parameter is missing.
    #[error("Missing required parameter '{name}'")]
    MissingRequired {
        /// The name of which parameter
        /// is missing.
        name: String,
    },

    /// This error is used when a parameter
    /// has an unexpected type.
    #[error("Invalid parameter type for '{name}', expected '{expected}', found '{found}'")]
    InvalidType {
        /// The name of which parameter
        /// is missing.
        name: String,

        /// An identifier for the expected
        /// type. (not enforced)
        expected: String,

        /// An identifier for the found
        /// type. (not enforced)
        found: String,
    },

    /// This error is used when the macro
    #[error(
        "Couldn't infer the file type, please specify using `type = <type>` when invoking the macro"
    )]
    AmbiguousFileType,
}

/// This struct represents a parsed
/// version of the `cruct` macro
/// parameters.
pub struct MacroParameters {
    /// A glob of the path that
    /// defines where the macro
    /// should be looking for
    /// that configuration file.
    ///
    /// **The query can only return
    /// one file**
    pub path: String,

    /// Which is the file format
    /// that should be used to
    /// parse the configuration file.
    pub format: Option<FileFormat>,
}

/// This struct represents a specific
/// field configuration, used along
/// the `MacroParameters` struct.
///
/// **A parameter can only be found once.**
pub struct FieldParameters {
    /// A name override for the parameter.
    pub name: Option<String>,

    /// Whether the parameter query
    /// is case-sensitive or insensitive.
    pub insensitive: bool,

    /// An environment variable name
    /// to replace or set the field if
    /// found.
    pub env_override: Option<String>,

    pub default: Option<String>,
}

impl Parse for MacroParameters {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let params = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

        let mut path = None;
        let mut format = None;

        for param in params {
            let key = param.path.to_token_stream().to_string();

            match (key.as_str(), &param.value) {
                (
                    "path",
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(value),
                        ..
                    }),
                ) => {
                    path = Some(value.value());
                }

                // ("format", Expr::Path(path)) => {
                //     let ident = path.to_token_stream().to_string();
                //
                //     format = Some(match ident.as_str() {
                //         "Yml" => FileFormat::Yml,
                //         "Json" => FileFormat::Json,
                //         "Toml" => FileFormat::Toml,
                //
                //         _ => Err(SynError::new_spanned(
                //             path,
                //             "Expected one of Yml, Json, Toml",
                //         ))?,
                //     });
                // }
                (
                    "format",
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(value),
                        ..
                    }),
                ) => {
                    let ident = value.value();
                    format = Some(match ident.as_str() {
                        "Yml" | "Yaml" => FileFormat::Yml,
                        "Json" => FileFormat::Json,
                        "Toml" => FileFormat::Toml,
                        _ => Err(SynError::new_spanned(
                            value,
                            "Expected one of: Yml, Yaml, Json, Toml",
                        ))?,
                    });
                }

                (name @ ("path" | "format"), value) => Err(SynError::new_spanned(
                    value,
                    format!(
                        "Invalid value type for '{name}' expected '{}'",
                        match name {
                            "path" => "&str",
                            "format" => "Json | Toml | Yml | Yaml",
                            _ => "",
                        }
                    ),
                ))?,

                (name, _) => Err(SynError::new_spanned(
                    param,
                    format!(
                        "Unknown parameter '{name}'. Known parameters include\n- path: &str\n- format: Json | Toml | Yml"
                    ),
                ))?,
            };
        }

        Ok(Self {
            path: path.ok_or(SynError::new(input.span(), "Missing parameter path"))?,
            format,
        })
    }
}

impl Parse for FieldParameters {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let params = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

        let mut name = None;
        let mut default = None;
        let mut insensitive = None;
        let mut env_override = None;

        for param in params {
            let key = param.path.to_token_stream().to_string();

            match (key.as_str(), &param.value) {
                (
                    "name",
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(value),
                        ..
                    }),
                ) => {
                    name = Some(value.value());
                }

                (
                    "insensitive",
                    Expr::Lit(ExprLit {
                        lit: Lit::Bool(value),
                        ..
                    }),
                ) => {
                    insensitive = Some(value.value());
                }

                (
                    "env_override",
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(value),
                        ..
                    }),
                ) => {
                    env_override = Some(value.value());
                }

                (
                    "default",
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(value),
                        ..
                    }),
                ) => {
                    default = Some(value.value());
                }

                (name @ ("name" | "insensitive" | "env_override" | "default"), value) => {
                    Err(SynError::new_spanned(
                        value,
                        format!(
                            "Invalid value type for '{name}' expected '{}'",
                            match name {
                                "name" => "&str",
                                "insensitive" => "bool",
                                "env_override" => "&str",
                                // "default" => "&str",
                                _ => "",
                            }
                        ),
                    ))?
                }

                (name, _) => Err(SynError::new_spanned(
                    param,
                    format!(
                        "Unknown parameter '{name}'. Known parameters include:\n- name: &str\n- insensitive: bool\n- env_override: &str"
                    ),
                ))?,
            }
        }

        Ok(Self {
            name,
            insensitive: insensitive.unwrap_or(false),
            env_override,
            default,
        })
    }
}
