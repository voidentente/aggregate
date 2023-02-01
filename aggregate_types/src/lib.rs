pub use phf;
pub use quote;
pub use syn;

use once_cell::sync::Lazy;
use phf::OrderedMap;
use syn::Attribute;

pub type LazyAttribute = Lazy<Attribute>;

pub type LazyAmalgamate = Lazy<&'static Amalgamate>;

pub type Attributes = OrderedMap<u16, &'static LazyAttribute>;

pub type Fields = OrderedMap<&'static str, Field>;

pub struct Amalgamate {
    pub attrs: Attributes,
    pub fields: Fields,
}

pub struct Field {
    pub attrs: Attributes,
    pub inner: Option<&'static LazyAmalgamate>,
}
