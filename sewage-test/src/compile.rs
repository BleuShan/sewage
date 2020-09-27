//! Compile test helpers

use std::{
    convert::AsRef,
    path::Path,
};
use trybuild::TestCases;

/// Execute the compilation test cases from a root path.
/// Any rust source files under a folder named pass or fail will pass repectively pass or fail.
pub fn try_build_root<PathRef>(root_path: PathRef)
where
    PathRef: AsRef<Path>,
{
    assert!(root_path.as_ref().exists());
    let root = root_path.as_ref().display().to_string();
    try_build(|cases| {
        cases.pass(format!("{root}/**/pass/**/*.rs"));
        cases.compile_fail(format!("{root}/**/fail/**/*.rs"));
    })
}

/// Execute the compilation test cases as configured.
pub fn try_build<BuilderFn>(build: BuilderFn)
where
    BuilderFn: FnOnce(&TestCases),
{
    let cases = TestCases::new();
    build(&cases);
}
