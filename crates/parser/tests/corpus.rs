use std::path::Path;

use duramen_parser::{PolicyParser, SchemaParser};
use duramen_test::assert_eq;

duramen_test::corpus!(policy = policy, schema = schema);

fn policy(_path: &Path, source: &str) {
    let result = PolicyParser::new(source).parse();
    let printed = result.print(source);
    assert_eq!(printed, source, "Roundtrip failed");
}

fn schema(_path: &Path, source: &str) {
    let result = SchemaParser::new(source).parse();
    let printed = result.print(source);
    assert_eq!(printed, source, "Roundtrip failed");
}
