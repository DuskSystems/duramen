#![expect(clippy::unwrap_used, reason = "Tests")]

use duramen_cst::{CstNode as _, Policies};
use duramen_diagnostic::Diagnostics;
use duramen_lowerer::PolicyLowerer;
use duramen_parser::PolicyParser;
use duramen_test::{TestContext, assert_diagnostics_snapshot};

duramen_test::fixtures!(policy::failure = lower_corpus);

fn lower_corpus(fixture: &TestContext<'_>) {
    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(fixture.source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(fixture.source, &mut diagnostics).lower(cst);

    assert_diagnostics_snapshot!(fixture.name, fixture.source, &diagnostics);
}
