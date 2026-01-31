use std::path::Path;

use duramen_lower::{PolicyLowerer, SchemaLowerer};
use duramen_parser::{PolicyParser, SchemaParser};

duramen_test::corpus!(policy = policy, schema = schema);

fn policy(_path: &Path, source: &str) {
    let result = PolicyParser::new(source).parse();
    let lowerer = PolicyLowerer::new(source);
    drop(lowerer.lower(result.tree()));
}

fn schema(_path: &Path, source: &str) {
    let result = SchemaParser::new(source).parse();
    let lowerer = SchemaLowerer::new(source);
    drop(lowerer.lower(result.tree()));
}
