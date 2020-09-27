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
    try_blocks,
    try_trait,
    type_alias_impl_trait
)]

mod prelude;

use prelude::*;

const COMPILE_TEST_ROOT: &str = "tests/compile";

#[test]
fn compile_test() {
    try_build(COMPILE_TEST_ROOT)
}
