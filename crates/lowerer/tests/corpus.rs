#![expect(clippy::unwrap_used, reason = "Tests")]

use duramen_cst::{CstNode as _, Policies, Schema};
use duramen_diagnostic::Diagnostics;
use duramen_lowerer::{PolicyLowerer, SchemaLowerer};
use duramen_parser::{PolicyParser, SchemaParser};
use duramen_test::TestContext;

duramen_test::corpus!(policy = lower_policy, schema = lower_schema);

fn lower_policy(corpus: &TestContext<'_>) {
    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(corpus.source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(&mut diagnostics).lower(cst);
}

fn lower_schema(corpus: &TestContext<'_>) {
    let mut diagnostics = Diagnostics::new();

    let tree = SchemaParser::new(corpus.source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Schema::cast(root).unwrap();
    let _ast = SchemaLowerer::new(&mut diagnostics).lower(cst);
}
