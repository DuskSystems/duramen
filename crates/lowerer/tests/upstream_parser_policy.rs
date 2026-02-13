#![expect(clippy::unwrap_used, reason = "Tests")]

use duramen_cst::{CstNode as _, Policies};
use duramen_diagnostic::Diagnostics;
use duramen_lowerer::PolicyLowerer;
use duramen_parser::PolicyParser;
use duramen_test::{TestContext, assert_diagnostics_snapshot, assert_fixture_snapshot};

duramen_test::fixtures!(policy::failure = lower_corpus);

fn lower_corpus(fixture: &TestContext<'_>) {
    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(fixture.source, &mut diagnostics).parse();
    let root = tree.root().unwrap();
    assert_fixture_snapshot!("tree", fixture, root);

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(&mut diagnostics).lower(cst);

    assert!(!diagnostics.is_empty(), "Expect diagnostics");
    assert_diagnostics_snapshot!("diagnostics", fixture, &diagnostics);
}
