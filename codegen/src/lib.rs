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
    box_patterns,
    box_syntax,
    format_args_capture,
    never_type,
    or_patterns,
    trait_alias,
    try_blocks,
    type_alias_impl_trait,
    proc_macro_diagnostic,
    proc_macro_quote,
    proc_macro_span,
    doc_cfg
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
            type_alias_impl_trait,
        ))
    ),
    include = "../README.md"
)]

mod attributes;

use attributes::*;
use proc_macro::TokenStream;
use syn::{
    parse,
    Error,
};

type Result<T, E = Error> = std::result::Result<T, E>;

///
#[proc_macro_attribute]
pub fn test(args: TokenStream, item: TokenStream) -> TokenStream {
    match parse::<TestAttributeArgs>(args) {
        Ok(args) => match parse::<TestAttribute>(item) {
            Ok(attribute) => attribute.with_args(args).into(),
            Err(error) => error.to_compile_error().into(),
        },
        Err(error) => error.to_compile_error().into(),
    }
}
