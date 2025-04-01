use proc_macro::TokenStream;
use quote::quote;

mod parameters;
mod loader;

#[proc_macro_attribute]
pub fn cruct(item: TokenStream, params: TokenStream) -> TokenStream {
    quote! {}
        .into()
}
