#![warn(
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    clippy::all,
    clippy::pedantic
)]
#![feature(
    external_doc,
    doc_cfg,
    box_patterns,
    box_syntax,
    format_args_capture,
    never_type,
    or_patterns,
    trait_alias,
    try_blocks,
    try_trait,
    type_alias_impl_trait
)]
#![doc(
    test(
        no_crate_inject,
        attr(warn(nonstandard_style, rust_2018_idioms)),
        attr(feature(
            box_patterns,
            box_syntax,
            format_args_capture,
            never_type,
            or_patterns,
            trait_alias,
            try_blocks,
            try_trait,
            type_alias_impl_trait,
        ))
    ),
    include = "../README.md"
)]

pub mod prelude;
pub(crate) mod sealed;
pub use codegen::*;

use crate::prelude::*;

/// Testing
#[derive(Debug, Display)]
pub enum Test {
    /// Some other test
    #[display(fmt = "nothing test")]
    Nothing,
}

#[instrument]
async fn make() {}
