//! # `duramen-test`
//!
//! Test utilities for duramen.

use core::error::Error;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

#[doc(hidden)]
pub use anstream;
#[doc(hidden)]
pub use datatest_stable;
pub use datatest_stable::Result;
#[doc(hidden)]
pub use insta;
pub use similar_asserts::assert_eq;
use walkdir::WalkDir;

#[doc(hidden)]
pub trait IntoTestResult {
    fn into_test_result(self) -> datatest_stable::Result<()>;
}

impl IntoTestResult for () {
    fn into_test_result(self) -> datatest_stable::Result<()> {
        Ok(())
    }
}

impl<E: Into<Box<dyn Error>>> IntoTestResult for core::result::Result<(), E> {
    fn into_test_result(self) -> datatest_stable::Result<()> {
        self.map_err(Into::into)
    }
}

/// Run tests across the entire corpus.
#[macro_export]
macro_rules! corpus {
    (policy = $policy_fn:ident) => {
        fn __policy_corpus_wrapper(path: &::std::path::Path) -> $crate::datatest_stable::Result<()> {
            let source = ::std::fs::read_to_string(path)?;
            $crate::IntoTestResult::into_test_result($policy_fn(path, &source))
        }

        $crate::datatest_stable::harness! {
            { test = __policy_corpus_wrapper, root = $crate::CEDAR_CORPUS, pattern = r".*[.]cedar$" },
            { test = __policy_corpus_wrapper, root = $crate::CEDAR_INTEGRATION_TESTS, pattern = r".*[.]cedar$" },
            { test = __policy_corpus_wrapper, root = $crate::CEDAR_POLICY_PARSER_TESTS, pattern = r".*[.]cedar$" },
        }
    };

    (schema = $schema_fn:ident) => {
        fn __schema_corpus_wrapper(path: &::std::path::Path) -> $crate::datatest_stable::Result<()> {
            let source = ::std::fs::read_to_string(path)?;
            $crate::IntoTestResult::into_test_result($schema_fn(path, &source))
        }

        $crate::datatest_stable::harness! {
            { test = __schema_corpus_wrapper, root = $crate::CEDAR_CORPUS, pattern = r".*[.]cedarschema$" },
            { test = __schema_corpus_wrapper, root = $crate::CEDAR_SAMPLE_DATA, pattern = r".*[.]cedarschema$" },
            { test = __schema_corpus_wrapper, root = $crate::CEDAR_SCHEMA_PARSER_TESTS, pattern = r".*[.]cedarschema$" },
        }
    };

    (policy = $policy_fn:ident, schema = $schema_fn:ident) => {
        fn __policy_corpus_wrapper(path: &::std::path::Path) -> $crate::datatest_stable::Result<()> {
            let source = ::std::fs::read_to_string(path)?;
            $crate::IntoTestResult::into_test_result($policy_fn(path, &source))
        }

        fn __schema_corpus_wrapper(path: &::std::path::Path) -> $crate::datatest_stable::Result<()> {
            let source = ::std::fs::read_to_string(path)?;
            $crate::IntoTestResult::into_test_result($schema_fn(path, &source))
        }

        $crate::datatest_stable::harness! {
            { test = __policy_corpus_wrapper, root = $crate::CEDAR_CORPUS, pattern = r".*[.]cedar$" },
            { test = __policy_corpus_wrapper, root = $crate::CEDAR_INTEGRATION_TESTS, pattern = r".*[.]cedar$" },
            { test = __policy_corpus_wrapper, root = $crate::CEDAR_POLICY_PARSER_TESTS, pattern = r".*[.]cedar$" },
            { test = __schema_corpus_wrapper, root = $crate::CEDAR_CORPUS, pattern = r".*[.]cedarschema$" },
            { test = __schema_corpus_wrapper, root = $crate::CEDAR_SAMPLE_DATA, pattern = r".*[.]cedarschema$" },
            { test = __schema_corpus_wrapper, root = $crate::CEDAR_SCHEMA_PARSER_TESTS, pattern = r".*[.]cedarschema$" },
        }
    };
}

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

pub static POLICIES: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
    walk(
        &[
            CEDAR_CORPUS,
            CEDAR_INTEGRATION_TESTS,
            CEDAR_POLICY_PARSER_TESTS,
        ],
        "cedar",
    )
});

pub static SCHEMAS: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
    walk(
        &[CEDAR_CORPUS, CEDAR_SAMPLE_DATA, CEDAR_SCHEMA_PARSER_TESTS],
        "cedarschema",
    )
});

fn walk(directories: &[&str], extension: &str) -> Vec<PathBuf> {
    directories
        .iter()
        .filter(|path| Path::new(path).exists())
        .flat_map(WalkDir::new)
        .filter_map(core::result::Result::ok)
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == extension))
        .map(|entry| entry.path().to_path_buf())
        .collect()
}
