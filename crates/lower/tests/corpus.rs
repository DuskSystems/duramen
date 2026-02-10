#![expect(clippy::unwrap_used, reason = "Tests")]

use std::path::Path;

use duramen_cst::{CstNode as _, Policies, Schema};
use duramen_diagnostic::Diagnostics;
use duramen_lower::{PolicyLowerer, SchemaLowerer};
use duramen_parser::{PolicyParser, SchemaParser};

duramen_test::corpus!(policy = lower_policy, schema = lower_schema);

fn lower_policy(_path: &Path, source: &str) {
    let mut diagnostics = Diagnostics::new();
    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();
    let cst = Policies::cast(root).unwrap();
    drop(PolicyLowerer::new(source, &mut diagnostics).lower(cst));
}

fn lower_schema(_path: &Path, source: &str) {
    let mut diagnostics = Diagnostics::new();
    let tree = SchemaParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();
    let cst = Schema::cast(root).unwrap();
    drop(SchemaLowerer::new(source, &mut diagnostics).lower(cst));
}
