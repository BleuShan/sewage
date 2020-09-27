#![warn(
    clippy::all,
    clippy::pedantic,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    unreachable_pub
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
    trivial_bounds,
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

mod macros;

pub mod error;
pub mod reexports;
pub mod std_traits;
pub mod tracing;
