use duramen_diagnostic::Diagnostics;
use duramen_parser::PolicyParser;
use duramen_test::TestContext;

duramen_test::fixtures!(policy = parse);

fn parse(fixture: &TestContext<'_>) {
    let mut diagnostics = Diagnostics::new();
    let _tree = PolicyParser::new(fixture.source, &mut diagnostics).parse();
}
