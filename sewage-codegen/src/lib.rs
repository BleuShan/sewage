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
    proc_macro_diagnostic,
    proc_macro_span,
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

mod attributes;
mod prelude;

use attributes::TestAttribute;
use prelude::*;

///
#[proc_macro_attribute]
pub fn test(args: TokenStream, item: TokenStream) -> TokenStream {
    match TestAttribute::parse(args, item) {
        Ok(attribute) => attribute.into(),
        Err(error) => {
            Diagnostic::spanned(error.span().unwrap(), Level::Error, error.to_string()).emit();
            TokenStream::new()
        }
    }
}
