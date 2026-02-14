#![no_main]

use duramen::lowerer::{PolicyLowerer, SchemaLowerer};
use duramen::parser::{PolicyParser, SchemaParser};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|src: &str| {
    let (tree, diagnostics) = PolicyParser::parse(src);
    let _policies = PolicyLowerer::lower(&tree, diagnostics);

    let (tree, diagnostics) = SchemaParser::parse(src);
    let _schema = SchemaLowerer::lower(&tree, diagnostics);
});
