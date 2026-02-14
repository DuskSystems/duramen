use duramen_parser::{PolicyParser, SchemaParser};
use duramen_test::{TestContext, assert_eq};

duramen_test::corpus!(policy = parse_policy, schema = parse_schema);

fn parse_policy(corpus: &TestContext<'_>) {
    let (tree, _diagnostics) = PolicyParser::parse(corpus.source);
    let roundtrip = tree.to_string();
    assert_eq!(roundtrip, corpus.source, "Roundtrip failed");
}

fn parse_schema(corpus: &TestContext<'_>) {
    let (tree, _diagnostics) = SchemaParser::parse(corpus.source);
    let roundtrip = tree.to_string();
    assert_eq!(roundtrip, corpus.source, "Roundtrip failed");
}
