use std::path::Path;

use duramen_diagnostic::Diagnostics;
use duramen_parser::{PolicyParser, SchemaParser};
use duramen_test::assert_eq;

duramen_test::corpus!(policy = parser_policy, schema = parser_schema);

fn parser_policy(_path: &Path, source: &str) {
    let mut diagnostics = Diagnostics::new();
    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(tree.to_string(), source, "Roundtrip failed");
}

fn parser_schema(_path: &Path, source: &str) {
    let mut diagnostics = Diagnostics::new();
    let tree = SchemaParser::new(source, &mut diagnostics).parse();
    assert_eq!(tree.to_string(), source, "Roundtrip failed");
}
