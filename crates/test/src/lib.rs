#![cfg_attr(doc, doc = include_str!("../README.md"))]

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

pub use duramen_diagnostic::{Diagnostic, Diagnostics};
pub use rand;
pub use similar_asserts::*;
use walkdir::WalkDir;
#[doc(hidden)]
pub use {anstream, datatest_stable, insta};

pub struct TestContext<'a> {
    pub path: &'a Path,
    pub source: &'a str,
    pub name: &'a str,
}

impl<'a> TestContext<'a> {
    /// # Panics
    ///
    /// Panics if `path` has no name.
    #[must_use]
    #[expect(clippy::unwrap_used, reason = "Testing")]
    pub fn new(path: &'a Path, source: &'a str) -> Self {
        let name = path.file_stem().unwrap().to_str().unwrap();
        Self { path, source, name }
    }
}

/// Runs tests across the entire corpus.
#[macro_export]
macro_rules! corpus {
    (policy = $policy_fn:ident) => {
        $crate::corpus!(@wrap $policy_fn);
        $crate::datatest_stable::harness! {
            { test = corpus::$policy_fn, root = $crate::CEDAR_CORPUS, pattern = r".*[.]cedar$" },
            { test = corpus::$policy_fn, root = $crate::CEDAR_INTEGRATION_TESTS, pattern = r".*[.]cedar$" },
            { test = corpus::$policy_fn, root = $crate::CEDAR_POLICY_PARSER_TESTS, pattern = r".*[.]cedar$" },
        }
    };

    (schema = $schema_fn:ident) => {
        $crate::corpus!(@wrap $schema_fn);
        $crate::datatest_stable::harness! {
            { test = corpus::$schema_fn, root = $crate::CEDAR_CORPUS, pattern = r".*[.]cedarschema$" },
            { test = corpus::$schema_fn, root = $crate::CEDAR_SAMPLE_DATA, pattern = r".*[.]cedarschema$" },
            { test = corpus::$schema_fn, root = $crate::CEDAR_SCHEMA_PARSER_TESTS, pattern = r".*[.]cedarschema$" },
        }
    };

    (policy = $policy_fn:ident, schema = $schema_fn:ident) => {
        $crate::corpus!(@wrap $policy_fn, $schema_fn);
        $crate::datatest_stable::harness! {
            { test = corpus::$policy_fn, root = $crate::CEDAR_CORPUS, pattern = r".*[.]cedar$" },
            { test = corpus::$policy_fn, root = $crate::CEDAR_INTEGRATION_TESTS, pattern = r".*[.]cedar$" },
            { test = corpus::$policy_fn, root = $crate::CEDAR_POLICY_PARSER_TESTS, pattern = r".*[.]cedar$" },
            { test = corpus::$schema_fn, root = $crate::CEDAR_CORPUS, pattern = r".*[.]cedarschema$" },
            { test = corpus::$schema_fn, root = $crate::CEDAR_SAMPLE_DATA, pattern = r".*[.]cedarschema$" },
            { test = corpus::$schema_fn, root = $crate::CEDAR_SCHEMA_PARSER_TESTS, pattern = r".*[.]cedarschema$" },
        }
    };

    (@wrap $($inner:ident),+) => {
        mod corpus {
            $(
                pub(super) fn $inner(
                    path: &::std::path::Path,
                    source: String,
                ) -> $crate::datatest_stable::Result<()> {
                    super::$inner(&$crate::TestContext::new(path, &source));
                    Ok(())
                }
            )+
        }
    };
}

#[macro_export]
macro_rules! fixtures {
    (policy = $fn:ident) => {
        $crate::fixtures!(@policy $fn, $crate::POLICY_FIXTURES);
    };

    (policy::success = $fn:ident) => {
        $crate::fixtures!(@policy $fn, $crate::POLICY_FIXTURES_SUCCESS);
    };

    (policy::failure = $fn:ident) => {
        $crate::fixtures!(@policy $fn, $crate::POLICY_FIXTURES_FAILURE);
    };

    (@policy $fn:ident, $root:expr) => {
        $crate::fixtures!(@wrap $fn);
        $crate::datatest_stable::harness! {
            { test = fixtures::$fn, root = $root, pattern = r".*[.]cedar$" },
        }
    };

    (@wrap $($inner:ident),+) => {
        mod fixtures {
            $(
                pub(super) fn $inner(
                    path: &::std::path::Path,
                    source: String,
                ) -> $crate::datatest_stable::Result<()> {
                    super::$inner(&$crate::TestContext::new(path, &source));
                    Ok(())
                }
            )+
        }
    };
}

/// Asserts a snapshot with a name derived from the fixture.
#[macro_export]
macro_rules! assert_fixture_snapshot {
    ($name:literal, $fixture:expr, $value:expr) => {{
        let name = ::std::format!("{}_{}", $fixture.name, $name);
        let value = ::std::string::ToString::to_string(&$value);
        $crate::insta::assert_snapshot!(name, value)
    }};
}

/// Asserts a diagnostics snapshot.
#[macro_export]
macro_rules! assert_diagnostics_snapshot {
    ($name:literal, $fixture:expr, $diagnostics:expr) => {{
        let rendered = $diagnostics
            .iter()
            .map(|diagnostic| diagnostic.render($fixture.name, $fixture.source))
            .collect::<Vec<_>>()
            .join("\n");

        let value = $crate::anstream::adapter::strip_str(&rendered).to_string();
        $crate::assert_fixture_snapshot!($name, $fixture, value)
    }};
}

#[rustfmt::skip]
pub const POLICY_FIXTURES: &str = concat!(env!("CARGO_WORKSPACE_DIR"), "/fixtures/policy");
#[rustfmt::skip]
pub const POLICY_FIXTURES_SUCCESS: &str = concat!(env!("CARGO_WORKSPACE_DIR"), "/fixtures/policy/success");
#[rustfmt::skip]
pub const POLICY_FIXTURES_FAILURE: &str = concat!(env!("CARGO_WORKSPACE_DIR"), "/fixtures/policy/failure");

#[rustfmt::skip]
pub const CEDAR_CORPUS: &str = concat!(env!("CARGO_WORKSPACE_DIR"), "/cedar-integration-tests/corpus-tests");

#[rustfmt::skip]
pub const CEDAR_INTEGRATION_TESTS: &str = concat!(env!("CARGO_WORKSPACE_DIR"), "/cedar-integration-tests/tests");
#[rustfmt::skip]
pub const CEDAR_SAMPLE_DATA: &str = concat!(env!("CARGO_WORKSPACE_DIR"), "/cedar-integration-tests/sample-data");
#[rustfmt::skip]
pub const CEDAR_POLICY_PARSER_TESTS: &str = concat!(env!("CARGO_WORKSPACE_DIR"), "/cedar/cedar-policy-core/src/parser/testfiles");
#[rustfmt::skip]
pub const CEDAR_SCHEMA_PARSER_TESTS: &str = concat!(env!("CARGO_WORKSPACE_DIR"), "/cedar/cedar-policy-core/src/validator/cedar_schema/testfiles");

#[rustfmt::skip]
pub static CORPUS_POLICIES: LazyLock<Vec<PathBuf>> = LazyLock::new(|| walk(&[CEDAR_CORPUS, CEDAR_INTEGRATION_TESTS, CEDAR_POLICY_PARSER_TESTS], "cedar"));
#[rustfmt::skip]
pub static CORPUS_SCHEMAS: LazyLock<Vec<PathBuf>> = LazyLock::new(|| walk(&[CEDAR_CORPUS, CEDAR_SAMPLE_DATA, CEDAR_SCHEMA_PARSER_TESTS], "cedarschema"));

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
