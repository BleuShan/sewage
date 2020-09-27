//! The prelude for the library.

// Reexport internaly some of shared for convenience
#[allow(unused_imports)]
pub(crate) use shared::{
    reexports::*,
    std_traits::*,
    tracing::prelude::*,
};
