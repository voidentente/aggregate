use std::cell::*;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use crate::types::Amalgamate;
use crate::Aggregate;

/// "Transparent" implementation for common types with one generic.
/// Behave as if the wrapping type doesn't exist, simply forwarding
/// the aggregation call to the generic.
macro_rules! transparent_impl {
    ($ty:ty) => {
        impl<T: Aggregate> Aggregate for $ty {
            fn aggregate() -> &'static Amalgamate {
                T::aggregate()
            }
        }
    };
}

transparent_impl!(Option<T>);
transparent_impl!(Box<T>);
transparent_impl!(Cell<T>);
transparent_impl!(Ref<'_, T>);
transparent_impl!(RefMut<'_, T>);
transparent_impl!(Pin<T>);
transparent_impl!(Arc<T>);
transparent_impl!(Mutex<T>);
