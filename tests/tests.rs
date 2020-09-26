#![feature(
    external_doc,
    box_patterns,
    box_syntax,
    format_args_capture,
    never_type,
    or_patterns,
    trait_alias,
    try_blocks,
    type_alias_impl_trait
)]

use trybuild::TestCases as TryBuildTestCases;

const COMPILE_TEST_ROOT: &str = "tests/compile";

#[test]
fn compile_test() {
    let tests = TryBuildTestCases::new();
    tests.pass(format!("{COMPILE_TEST_ROOT}/**/pass/*"));
    tests.compile_fail(format!("{COMPILE_TEST_ROOT}/**/fail/*"));
}
