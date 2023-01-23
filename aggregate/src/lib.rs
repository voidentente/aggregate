//! "Aggregate" attributes of structs and enums at compile-time
//! to make them available at runtime using a simple Derive macro.

#[cfg(feature = "derive")]
pub use aggregate_derive as derive;
pub use aggregate_types as types;

pub use syn;
pub extern crate proc_macro;

/// This trait can be derived using the `aggregate_derive` crate,
/// which is included by default.
pub trait Aggregate {
    /// `aggregate` does three things:
    ///
    /// 1. Collect all attributes of the current TokenStream.
    /// 2. Call `aggregate` on nested structures.
    /// 3. Return the merged structure.
    ///
    /// To include nested structures, one must mark them using
    /// the `#[aggregate]` helper attribute.
    ///
    /// This is a more permissive alternative to calling `aggregate`
    /// on all fields, which will not work if any field does not
    /// implement `Aggregate`.
    fn aggregate() -> types::Struct;
}
