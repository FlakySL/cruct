use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Error as SynError, Expr, ExprLit, Lit, MetaNameValue, Result as SynResult, Token};

/// This struct represents a specific field configuration, used along the
/// `MacroParameters` struct.
///
/// **A parameter can only be found once.**
#[derive(Default)]
pub struct FieldParams {
    /// A name override for the parameter.
    pub name: Option<String>,

    /// Whether the parameter query is case-sensitive or insensitive.
    pub insensitive: bool,

    /// An environment variable name to replace or set the field if
    /// found.
    pub env_override: Option<String>,

    /// A default value for the field.
    pub default: Option<Expr>,

    /// An argument override for the field, used to set the value
    pub arg_override: Option<String>,
}

impl Parse for FieldParams {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let params = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

        let mut name = None;
        let mut default = None;
        let mut insensitive = None;
        let mut env_override = None;
        let mut arg_override = None;

        for param in params {
            let key = param
                .path
                .to_token_stream()
                .to_string();

            match (key.as_str(), &param.value) {
                ("name", Expr::Lit(ExprLit { lit: Lit::Str(value), .. })) => {
                    name = Some(value.value());
                },

                ("insensitive", Expr::Lit(ExprLit { lit: Lit::Bool(value), .. })) => {
                    insensitive = Some(value.value());
                },

                ("env_override", Expr::Lit(ExprLit { lit: Lit::Str(value), .. })) => {
                    env_override = Some(value.value());
                },

                ("arg_override", Expr::Lit(ExprLit { lit: Lit::Str(value), .. })) => {
                    arg_override = Some(value.value());
                },

                ("default", value) => {
                    default = Some(value.clone());
                },

                (name @ ("name" | "insensitive" | "env_override" | "arg_override"), value) => {
                    Err(SynError::new_spanned(
                        value,
                        format!(
                            "Invalid value type for '{name}' expected '{}'",
                            match name {
                                "name" => "&str",
                                "insensitive" => "bool",
                                "arg_override" => "&str",
                                "env_override" => "&str",
                                _ => "",
                            }
                        ),
                    ))?
                },

                (name, _) => Err(SynError::new_spanned(
                    param,
                    format!(
                        "Unknown parameter '{name}'. Known parameters include:\n- name: &str\n- \
                         insensitive: bool\n- env_override: &str"
                    ),
                ))?,
            }
        }

        Ok(Self {
            name,
            insensitive: insensitive.unwrap_or(false),
            env_override,
            arg_override,
            default,
        })
    }
}
