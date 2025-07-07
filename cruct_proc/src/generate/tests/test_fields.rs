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
        arg_override: None,
    };
    let ident: Ident = parse_quote! { bar };
    let ty: Type = parse_quote! { u32 };
    let tokens = generate_field_initialization(&params, &ident, "bar", &ty).to_string();

    assert!(tokens.contains("42"));
}

#[test]
fn arg_override_generates_correct_lookup() {
    let params = FieldParams {
        env_override: None,
        name: None,
        insensitive: false,
        default: None,
        arg_override: Some("foo".into()),
    };
    let ident: Ident = parse_quote! { foo };
    let ty: Type = parse_quote! { String };
    let tokens = generate_field_initialization(&params, &ident, "foo", &ty).to_string();

    // Expect to see args().skip(1).find_map for "--flag=" prefix
    assert!(tokens.contains("std :: env :: args () . skip (1) . find_map"));
    assert!(tokens.contains("\"--\" , \"foo\" , \"=\""));
}
