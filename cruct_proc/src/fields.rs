use syn::{ItemStruct, Type, Result as SynResult};

use crate::parameters::FieldParameters;

/// This represents a parsed struct field
/// that optionally might contain parameters
/// on how to resolve configuration for
/// that parameter.
pub struct StructField {
    parameters: Option<FieldParameters>,
    name: String,
    data_type: Type
}

impl StructField {
    /// This method parses all the fields from a struct,
    /// along with the parameters attribute these might
    /// have.
    ///
    /// ## Parameters
    /// - item: A parsed ItemStruct token stream.
    ///
    /// ## Returns
    /// A vector of all parsed struct fields.
    pub fn from_struct(item: &ItemStruct) -> SynResult<Vec<Self>> {
        let mut fields = Vec::new();

        for field in &item.fields {
            fields.push(Self {
                parameters: field
                    .attrs
                    .iter()
                    .find(|attr| attr
                        .path()
                        .is_ident("field")
                    )
                    .map(|attr| attr.parse_args::<FieldParameters>())
                    .transpose()?,

                name: field
                    .ident
                    .as_ref()
                    .unwrap()
                    .to_string(),

                data_type: field
                    .ty
                    .clone()
            });
        }

        Ok(fields)
    }
}

pub fn remove_field_attrs(item: &mut ItemStruct) {
    for ref mut field in &mut item.fields {
        field
            .attrs
            .retain(|attr| !attr
                .path()
                .is_ident("field")
            );
    }
}

