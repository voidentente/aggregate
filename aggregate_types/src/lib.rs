//! Data structures used to represent the output of `Aggregate`.

/// The "untyped" type that maps from a field name to a field.
///
/// Contrary to a fully untyped system, this offers more guidance,
/// and contrary to a fully typed system, does not litter single-use types.
pub type Fields = std::collections::HashMap<String, Field>;

/// Represents a structure-level tuple of attributes and fields.
///
/// ```
/// /// This is my struct.     <--- `attrs` value
/// struct MyStruct {
///   /// This is my field.    <--- `fields` member
///   my_field: bool,
/// }
/// ```
#[derive(Default)]
pub struct Struct {
    pub attrs: Vec<syn::Attribute>,
    pub fields: Fields,
}

#[cfg(feature = "serde")]
impl serde::Serialize for Struct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Struct", 2)?;
        state.serialize_field(
            "attrs",
            &self
                .attrs
                .iter()
                .map(|attr| attr.tokens.to_string())
                .collect::<Vec<String>>(),
        )?;
        state.serialize_field("fields", &self.fields)?;
        state.end()
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

/// Represents a field-level tuple of attributes and an inner structure.
/// ```
/// /// This is my struct.
/// struct MyStruct {
///   /// This is my field.    <--- `attrs` value
///   my_field: MyInner,
/// }
///
/// /// This is my inner struct.    <--- `inner.attrs` value
/// struct MyInner {
///   /// My inner field attribute  <--- `inner.fields` member
///   my_inner_field: bool,
/// }
/// ```
#[derive(Default)]
pub struct Field {
    pub attrs: Vec<syn::Attribute>,
    pub inner: Option<Struct>,
}

#[cfg(feature = "serde")]
impl serde::Serialize for Field {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Field", 2)?;
        state.serialize_field(
            "attrs",
            &self
                .attrs
                .iter()
                .map(|attr| attr.tokens.to_string())
                .collect::<Vec<String>>(),
        )?;
        state.serialize_field("inner", &self.inner)?;
        state.end()
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
