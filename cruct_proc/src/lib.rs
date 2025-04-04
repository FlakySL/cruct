use fields::{remove_field_attrs, StructField};
use parameters::MacroParameters;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

mod fields;
mod parameters;
mod loader;

#[proc_macro_attribute]
pub fn cruct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let options = parse_macro_input!(attr as MacroParameters);
    let mut item = parse_macro_input!(item as ItemStruct);

    let fields = match StructField::from_struct(&item) {
        Ok(fields) => fields,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    remove_field_attrs(&mut item);

    quote! {
        #item

        
    }
        .into()
}
