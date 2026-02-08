#![no_main]

use duramen::diagnostic::Diagnostics;
use duramen::parser::{PolicyParser, SchemaParser};
use duramen_test::assert_eq;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|src: &str| {
    let mut diagnostics = Diagnostics::new();
    let tree = PolicyParser::new(src, &mut diagnostics).parse();
    assert_eq!(tree.print(src), src, "Roundtrip failed");

    let mut diagnostics = Diagnostics::new();
    let tree = SchemaParser::new(src, &mut diagnostics).parse();
    assert_eq!(tree.print(src), src, "Roundtrip failed");
});
