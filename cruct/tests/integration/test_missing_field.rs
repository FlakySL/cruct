use assay::assay;
use cruct::{ParserError, cruct};

#[assay(
    include = ["tests/fixtures/test_config.toml"],
)]
fn missing_field_errors() {
    #[cruct(load_config(path = "tests/fixtures/test_config.toml"))]
    #[derive(Debug)]
    #[allow(dead_code)]
    struct MissingField {
        present: String,
        absent: u8,
    }

    let result = MissingField::loader()
        .with_cli(0)
        .with_config()
        .load();

    assert!(result.is_err());
    if let Err(e) = result {
        let field = String::from("present");
        let expected = ParserError::MissingField(field).to_string();
        assert_eq!(e.to_string(), expected);
    }
}
