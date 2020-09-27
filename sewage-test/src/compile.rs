//! Compile test helpers

use std::{
    convert::AsRef,
    path::Path,
};
use trybuild::TestCases;

/// Try build from a root path.
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

/// Try build using the configured thing
pub fn try_build<BuilderFn>(build: BuilderFn)
where
    BuilderFn: FnOnce(&TestCases),
{
    let cases = TestCases::new();
    build(&cases);
}
