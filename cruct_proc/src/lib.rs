//! Procedural macro implementation for configuration loading
//!
//! This crate provides the `#[cruct]` attribute macro that generates
//! configuration loading implementation for structs.

use parse::{MacroParams, StructField, remove_field_attrs};
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

mod generate;
mod parse;

/// Macro to load configuration files into Rust structs.
///
/// # Usage
/// ```ignore
/// #[cruct(load_config(path = "config.toml", format = "Toml"))]
/// struct Config {
///     #[field(name = "http_port", default = 8080)]
///     port: u16,
/// }
/// ```
#[proc_macro_attribute]
pub fn cruct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let params = parse_macro_input!(attr as MacroParams);
    let mut item = parse_macro_input!(item as ItemStruct);

    let fields = match StructField::from_struct(&item) {
        Ok(fields) => fields,
        Err(e) => {
            return e
                .to_compile_error()
                .into();
        },
    };

    remove_field_attrs(&mut item);

    let impl_block = generate::generate_impl_block(&item.ident, &params, &fields);

    let expanded = quote! {
        #item
        #impl_block
    };

    expanded.into()
}
