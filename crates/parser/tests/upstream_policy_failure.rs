use duramen_parser::PolicyParser;
use duramen_test::TestContext;

duramen_test::fixtures!(policy::failure = parse);

fn parse(fixture: &TestContext<'_>) {
    let (_tree, _diagnostics) = PolicyParser::parse(fixture.source);
}
