//! "Aggregate" attributes of structs and enums at compile-time
//! to make them available at runtime using a simple Derive macro.

#[cfg(feature = "impl")]
mod default;

#[cfg(feature = "derive")]
pub use aggregate_derive as derive;
pub use aggregate_types as types;

pub mod prelude {
    #[cfg(feature = "derive")]
    pub use crate::derive::Aggregate;
    pub use crate::Aggregate;
}

pub trait Aggregate {
    fn aggregate() -> &'static types::Amalgamate;
}
