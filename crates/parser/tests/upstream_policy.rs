use duramen_parser::PolicyParser;
use duramen_test::TestContext;

duramen_test::fixtures!(policy::success = parse);

fn parse(fixture: &TestContext<'_>) {
    let (_tree, diagnostics) = PolicyParser::parse(fixture.source);
    assert_eq!(diagnostics.len(), 0, "Expected no parser diagnostics");
}
