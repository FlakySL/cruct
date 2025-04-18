use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Error as SynError, Expr, ExprLit, Lit, MetaNameValue, Result as SynResult, Token};

/// This struct represents a specific field configuration, used along the
/// `MacroParameters` struct.
///
/// **A parameter can only be found once.**
#[derive(Default, Debug)]
pub struct FieldParams {
    /// A name override for the parameter.
    pub name: Option<String>,

    /// Whether the parameter query is case-sensitive or insensitive.
    pub insensitive: bool,

    /// An environment variable name to replace or set the field if
    /// found.
    pub env_override: Option<String>,

    /// A default value for the field.
    pub default: Option<String>,
}

impl Parse for FieldParams {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let params = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

        let mut name = None;
        let mut default = None;
        let mut insensitive = None;
        let mut env_override = None;

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

                ("default", Expr::Lit(ExprLit { lit: Lit::Str(value), .. })) => {
                    default = Some(value.value());
                },

                (name @ ("name" | "insensitive" | "env_override" | "default"), value) => {
                    Err(SynError::new_spanned(
                        value,
                        format!(
                            "Invalid value type for '{name}' expected '{}'",
                            match name {
                                "name" => "&str",
                                "insensitive" => "bool",
                                "env_override" => "&str",
                                "default" => "&str",
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
            default,
        })
    }
}
