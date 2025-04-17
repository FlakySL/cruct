use syn::{Attribute, ItemStruct, Result as SynResult, Type};

use crate::parameters::FieldParameters;

/// This represents a parsed struct field that optionally might contain
/// parameters on how to resolve configuration for
/// that parameter.
pub struct StructField {
    /// Optional parameters associated with the field, used for resolving
    /// configuration.
    pub parameters: Option<FieldParameters>,
    /// The name of the field as a string.
    pub name: String,
    /// The type of the field.
    pub data_type: Type,
}

impl StructField {
    /// Parses all fields from a struct, extracting any associated parameters.
    ///
    /// This function iterates over each field in the provided `ItemStruct`,
    /// checking for attributes that specify field parameters. If such
    /// attributes are found, they are parsed and stored alongside the
    /// field's name and type.
    ///
    /// ## Parameters
    /// - `item`: A reference to a parsed `ItemStruct` token stream.
    ///
    /// ## Returns
    /// A `SynResult` containing a vector of `StructField` instances, each
    /// representing a field in the struct with its associated parameters,
    /// if any.
    pub fn from_struct(item: &ItemStruct) -> SynResult<Vec<Self>> {
        let mut fields = Vec::new();

        for field in &item.fields {
            fields.push(Self {
                parameters: field
                    .attrs
                    .iter()
                    .find(|attr| is_field_attr(attr))
                    .map(|attr| attr.parse_args::<FieldParameters>())
                    .transpose()?,

                name: field
                    .ident
                    .as_ref()
                    .unwrap()
                    .to_string(),

                data_type: field
                    .ty
                    .clone(),
            });
        }

        Ok(fields)
    }
}

/// Check if the attribute's path is identified as "field"
fn is_field_attr(attr: &Attribute) -> bool {
    attr.path()
        .is_ident("field")
}

/// Removes field attributes with the identifier "field" from a struct.
///
/// This function iterates over each field in the provided mutable `ItemStruct`,
/// removing any attributes that have the identifier "field".
///
/// ## Parameters
/// - `item`: A mutable reference to an `ItemStruct` from which field attributes
///   will be removed.
pub fn remove_field_attrs(item: &mut ItemStruct) {
    item.fields
        .iter_mut()
        .for_each(|field| {
            field
                .attrs
                .retain(|attr| !is_field_attr(attr));
        });
}
