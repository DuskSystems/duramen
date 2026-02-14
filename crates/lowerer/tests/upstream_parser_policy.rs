#![expect(clippy::unwrap_used, reason = "Tests")]

use duramen_lowerer::PolicyLowerer;
use duramen_parser::PolicyParser;
use duramen_test::{TestContext, assert_diagnostics_snapshot, assert_fixture_snapshot};

duramen_test::fixtures!(policy::failure = lower_corpus);

fn lower_corpus(fixture: &TestContext<'_>) {
    let (tree, diagnostics) = PolicyParser::parse(fixture.source);

    let root = tree.root().unwrap();
    assert_fixture_snapshot!("tree", fixture, root);

    let (_policies, diagnostics) = PolicyLowerer::lower(&tree, diagnostics);
    assert_diagnostics_snapshot!("diagnostics", fixture, &diagnostics);
}
