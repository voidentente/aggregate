//! Data structures used to represent the output of `Aggregate`.

mod compat;
#[cfg(feature = "shortcuts")]
mod shortcuts;

use std::collections::HashMap;
use syn::{Attribute, Type};

/// Maps from field/variation identifier to field.
pub type FieldMap = HashMap<String, Field>;

/// Maps from field identifier to type.
/// In order to group this into variations,
/// the wrapper `Descendants` is used.  
pub type DescendantMap = HashMap<String, Type>;

/// Newtype around a FieldMap that implements `quote::ToTokens`.
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Fields(pub FieldMap);

/// Newtype around a vector of attributes that implements `quote::ToTokens`.
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Attributes(pub Vec<Attribute>);

/// Represents a structure-level tuple of attributes and fields.
///
/// ```
/// /// This is my struct.     <--- `attrs` value
/// struct MyStruct {
///   /// This is my field.    <--- `fields` member
///   my_field: bool,
/// }
/// ```
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Amalgamate {
    pub fields: Fields,
    pub attrs: Attributes,
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
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Field {
    pub attrs: Attributes,
    pub inner: Option<Amalgamate>,
}

/// A newtype around DescendantMap that implements `quote::ToTokens`
/// and tracks its belonging to a collection ("variation") for use with enums.
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Descendants {
    pub map: DescendantMap,
    pub variation: Option<String>,
}
