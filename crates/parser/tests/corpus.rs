use duramen_diagnostic::Diagnostics;
use duramen_parser::{PolicyParser, SchemaParser};
use duramen_test::{TestContext, assert_eq};

duramen_test::corpus!(policy = parse_policy, schema = parse_schema);

fn parse_policy(corpus: &TestContext<'_>) {
    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(corpus.source, &mut diagnostics).parse();
    let roundtrip = tree.to_string();
    assert_eq!(roundtrip, corpus.source, "Roundtrip failed");
}

fn parse_schema(corpus: &TestContext<'_>) {
    let mut diagnostics = Diagnostics::new();

    let tree = SchemaParser::new(corpus.source, &mut diagnostics).parse();
    let roundtrip = tree.to_string();
    assert_eq!(roundtrip, corpus.source, "Roundtrip failed");
}
