//! The internal prelude.
//!
//! It mainly serves as a convenience reexport bucket.

pub use derive_more::{
    AsMut,
    AsRef,
    Deref,
    DerefMut,
    Display,
    From,
    FromStr,
    Index,
    IndexMut,
    Into,
    IntoIterator,
    TryInto,
};
pub use std::{
    convert::{
        AsMut,
        AsRef,
        TryFrom,
    },
    error::Error as StdError,
    fmt::{
        self as stdfmt,
        Debug,
        Display,
    },
    ops::{
        Deref,
        DerefMut,
        Index,
        IndexMut,
    },
    str::FromStr,
};
pub use thiserror::Error;
