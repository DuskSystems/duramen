#![cfg_attr(doc, doc = include_str!("../README.md"))]

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

pub use duramen_diagnostic::{Diagnostic, Diagnostics};
pub use rand;
pub use similar_asserts::*;
use walkdir::WalkDir;
#[doc(hidden)]
pub use {anstream, datatest_stable, insta};

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
                    super::$inner(path, &source);
                    Ok(())
                }
            )+
        }
    };
}

/// Asserts a single diagnostic snapshot.
#[macro_export]
macro_rules! assert_diagnostic_snapshot {
    ($source:expr, $diagnostic:expr, @$snapshot:literal) => {{
        let diagnostic: $crate::Diagnostic = $diagnostic.into();
        let rendered = diagnostic.render("test", $source);
        let value = $crate::anstream::adapter::strip_str(&rendered).to_string();
        $crate::insta::assert_snapshot!(value, @$snapshot)
    }};
}

/// Asserts multiple diagnostics snapshot.
#[macro_export]
macro_rules! assert_diagnostics_snapshot {
    ($source:expr, $diagnostics:expr) => {{
        let rendered = $diagnostics
            .iter()
            .map(|diagnostic| diagnostic.render("test", $source))
            .collect::<Vec<_>>()
            .join("\n");

        let value = $crate::anstream::adapter::strip_str(&rendered).to_string();
        $crate::insta::assert_snapshot!(value)
    }};

    ($source:expr, $diagnostics:expr, @$snapshot:literal) => {{
        let rendered = $diagnostics
            .iter()
            .map(|diagnostic| diagnostic.render("test", $source))
            .collect::<Vec<_>>()
            .join("\n");

        let value = $crate::anstream::adapter::strip_str(&rendered).to_string();
        $crate::insta::assert_snapshot!(value, @$snapshot)
    }};
}

/// Loads a fixture file from the top-level `fixtures/` directory.
#[macro_export]
macro_rules! fixture {
    ($name:ident) => {
        include_str!(concat!(
            env!("CARGO_WORKSPACE_DIR"),
            "/fixtures/",
            stringify!($name),
            ".cedar"
        ))
    };
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_WORKSPACE_DIR"),
            "/fixtures/",
            $name,
            ".cedar"
        ))
    };
}

/// Generates `#[test]` functions that each load a fixture and call a test function.
///
/// # Function variant
///
/// Calls a function with the fixture source:
///
/// ```ignore
/// fixtures!(parse: expr1, expr2, expr3);
/// ```
///
/// # Body variant
///
/// Inlines a block into each test, binding the source to a variable. Use this
/// when the test body contains snapshot assertions that need the test function
/// name for naming (e.g. `insta::assert_snapshot!`):
///
/// ```ignore
/// fixtures!(|source| {
///     assert_snapshot!(parse(source));
/// }: expr1, expr2, expr3);
/// ```
#[macro_export]
macro_rules! fixtures {
    ($test_fn:ident: $($name:ident),+ $(,)?) => {
        $(
            #[test]
            fn $name() {
                $test_fn($crate::fixture!($name));
            }
        )+
    };
    (|$source:ident| $body:block : $($name:ident),+ $(,)?) => {
        $(
            #[test]
            fn $name() {
                let $source = $crate::fixture!($name);
                $body
            }
        )+
    };
}

pub const CEDAR_CORPUS: &str = concat!(
    env!("CARGO_WORKSPACE_DIR"),
    "/cedar-integration-tests/corpus-tests"
);

pub const CEDAR_INTEGRATION_TESTS: &str = concat!(
    env!("CARGO_WORKSPACE_DIR"),
    "/cedar-integration-tests/tests"
);

pub const CEDAR_SAMPLE_DATA: &str = concat!(
    env!("CARGO_WORKSPACE_DIR"),
    "/cedar-integration-tests/sample-data"
);

pub const CEDAR_POLICY_PARSER_TESTS: &str = concat!(
    env!("CARGO_WORKSPACE_DIR"),
    "/cedar/cedar-policy-core/src/parser/testfiles"
);

pub const CEDAR_SCHEMA_PARSER_TESTS: &str = concat!(
    env!("CARGO_WORKSPACE_DIR"),
    "/cedar/cedar-policy-core/src/validator/cedar_schema/testfiles"
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
