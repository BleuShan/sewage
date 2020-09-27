//! Result handling and extension

pub use std::result::Result as StdResult;

/// an infallible result type.
pub type InfaillibleResult<Ok> = StdResult<Ok, !>;

/// a result type that can only represent failures.
pub type FailureResult<Err> = StdResult<!, Err>;
