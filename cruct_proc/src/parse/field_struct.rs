use syn::spanned::Spanned;
use syn::{Attribute, Error as SynError, Ident, ItemStruct, Result as SynResult, Type};

use super::FieldParams;

/// This represents a parsed struct field that optionally might contain
/// parameters on how to resolve configuration for
/// that parameter.
pub struct StructField {
    /// Optional parameters associated with the field, used for resolving
    /// configuration.
    pub params: Option<FieldParams>,

    /// The name of the field as a string.
    pub name: String,

    /// The type of the field.
    pub ty: Type,

    /// The identifier of the field.
    pub ident: Ident,
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
            let ident = field
                .ident
                .as_ref()
                .ok_or_else(|| SynError::new(field.span(), "Unnamed field not supported"))?;

            fields.push(Self {
                name: ident.to_string(),
                ident: ident.clone(),
                ty: field
                    .ty
                    .clone(),
                params: field
                    .attrs
                    .iter()
                    .find(|attr| is_field_attr(attr))
                    .map(|attr| attr.parse_args::<FieldParams>())
                    .transpose()?,
            });
        }

        Ok(fields)
    }
}

/// Check if the attribute's path is identified as "env_override",
/// "shell_override", "name", "insensitive", "default" and "description"
/// - "env_override" is used to override the field value from an environment
///   variable
/// - "shell_override" is used to override the field value from a shell command
/// - "name" is used to specify the name of the field in the configuration file
/// - insensitive" is used to specify if the field name should be treated as
///   case-insensitive
/// - description" is a metadata attribute that provides a description of the
///   field
fn is_field_attr(attr: &Attribute) -> bool {
    let allowed_attrs = [
        "env_override",
        "shell_override",
        "name",
        "insensitive",
        "default",
        "description",
        "field",
    ];

    attr.path()
        .get_ident()
        .is_some_and(|ident| {
            allowed_attrs
                .iter()
                .any(|allowed| ident == allowed)
        })
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
