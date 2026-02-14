use duramen_lowerer::{PolicyLowerer, SchemaLowerer};
use duramen_parser::{PolicyParser, SchemaParser};
use duramen_test::TestContext;

duramen_test::corpus!(policy = lower_policy, schema = lower_schema);

fn lower_policy(corpus: &TestContext<'_>) {
    let (tree, diagnostics) = PolicyParser::parse(corpus.source);
    let _policies = PolicyLowerer::lower(&tree, diagnostics);
}

fn lower_schema(corpus: &TestContext<'_>) {
    let (tree, diagnostics) = SchemaParser::parse(corpus.source);
    let _schema = SchemaLowerer::lower(&tree, diagnostics);
}
