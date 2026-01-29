//! # `duramen-test`
//!
//! Test utilities for duramen.

use std::path::Path;
use std::sync::LazyLock;

#[doc(hidden)]
pub use anstream;
#[doc(hidden)]
pub use insta;
pub use similar_asserts::assert_eq;
use walkdir::WalkDir;

/// Asserts a snapshot after stripping ANSI.
#[macro_export]
macro_rules! assert_snapshot {
    ($value:expr, @$snapshot:literal) => {{
        let value = $crate::anstream::adapter::strip_str(&$value).to_string();
        $crate::insta::assert_snapshot!(value, @$snapshot)
    }};
    ($name:expr, $value:expr, @$snapshot:literal) => {{
        let value = $crate::anstream::adapter::strip_str(&$value).to_string();
        $crate::insta::assert_snapshot!($name, value, @$snapshot)
    }};
    ($value:expr) => {{
        let value = $crate::anstream::adapter::strip_str(&$value).to_string();
        $crate::insta::assert_snapshot!(value)
    }};
    ($name:expr, $value:expr) => {{
        let value = $crate::anstream::adapter::strip_str(&$value).to_string();
        $crate::insta::assert_snapshot!($name, value)
    }};
}

pub const CEDAR_CORPUS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../cedar-integration-tests/corpus-tests"
);

pub const CEDAR_INTEGRATION_TESTS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../cedar-integration-tests/tests"
);

pub const CEDAR_SAMPLE_DATA: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../cedar-integration-tests/sample-data"
);

pub const CEDAR_POLICY_PARSER_TESTS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../cedar/cedar-policy-core/src/parser/testfiles"
);

pub const CEDAR_SCHEMA_PARSER_TESTS: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../cedar/cedar-policy-core/src/validator/cedar_schema/testfiles"
);

pub static POLICIES: LazyLock<Vec<String>> = LazyLock::new(|| {
    walk(
        &[
            CEDAR_CORPUS,
            CEDAR_INTEGRATION_TESTS,
            CEDAR_POLICY_PARSER_TESTS,
        ],
        "cedar",
    )
});

pub static SCHEMAS: LazyLock<Vec<String>> = LazyLock::new(|| {
    walk(
        &[CEDAR_CORPUS, CEDAR_SAMPLE_DATA, CEDAR_SCHEMA_PARSER_TESTS],
        "cedarschema",
    )
});

fn walk(directories: &[&str], extension: &str) -> Vec<String> {
    directories
        .iter()
        .filter(|path| Path::new(path).exists())
        .flat_map(WalkDir::new)
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == extension))
        .filter_map(|entry| std::fs::read_to_string(entry.path()).ok())
        .collect()
}
