use syn::{Ident, Type, parse_quote, parse_str};

use crate::generate::generate_field_initialization;
use crate::parse::FieldParams;

#[test]
fn default_value_generates_correct_initialization() {
    let params = FieldParams {
        env_override: None,
        name: None,
        insensitive: false,
        default: Some(parse_str("42").unwrap()),
        shell_override: None,
    };
    let ident: Ident = parse_quote! { bar };
    let ty: Type = parse_quote! { u32 };
    let tokens = generate_field_initialization(&params, &ident, "bar", &ty).to_string();

    assert!(tokens.contains("42"));
}
