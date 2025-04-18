use cruct_shared::FileFormat;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Error as SynError, Expr, ExprLit, Lit, MetaNameValue, Result as SynResult, Token};

/// This struct represents a parsed version of the `cruct` macro parameters.
pub struct MacroParams {
    /// A glob of the path that defines where the macro should be looking for
    /// that configuration file.
    ///
    /// **The query can only return one file**
    pub path: String,

    /// Which is the file format that should be used to parse the configuration
    /// file.
    pub format: Option<FileFormat>,
}

impl Parse for MacroParams {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let params = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

        let mut path = None;
        let mut format = None;

        for param in params {
            let key = param
                .path
                .to_token_stream()
                .to_string();

            match (key.as_str(), &param.value) {
                ("path", Expr::Lit(ExprLit { lit: Lit::Str(value), .. })) => {
                    path = Some(value.value());
                },

                ("format", Expr::Lit(ExprLit { lit: Lit::Str(value), .. })) => {
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
                },

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
                        "Unknown parameter '{name}'. Known parameters include\n- path: &str\n- \
                         format: Json | Toml | Yml"
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
