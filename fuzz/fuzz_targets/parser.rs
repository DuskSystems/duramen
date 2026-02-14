#![no_main]

use duramen::parser::{PolicyParser, SchemaParser};
use duramen_test::assert_eq;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|src: &str| {
    let (tree, _diagnostics) = PolicyParser::parse(src);
    let roundtrip = tree.to_string();
    assert_eq!(roundtrip, src, "Roundtrip failed");

    let (tree, _diagnostics) = SchemaParser::parse(src);
    let roundtrip = tree.to_string();
    assert_eq!(roundtrip, src, "Roundtrip failed");
});
