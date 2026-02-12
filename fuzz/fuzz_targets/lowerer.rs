#![no_main]
#![expect(clippy::unwrap_used, reason = "Fuzz")]

use duramen::cst::{CstNode as _, Policies, Schema};
use duramen::diagnostic::Diagnostics;
use duramen::lowerer::{PolicyLowerer, SchemaLowerer};
use duramen::parser::{PolicyParser, SchemaParser};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|src: &str| {
    let mut diagnostics = Diagnostics::new();
    let tree = PolicyParser::new(src, &mut diagnostics).parse();
    let root = tree.root().unwrap();
    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(&mut diagnostics).lower(cst);

    let mut diagnostics = Diagnostics::new();
    let tree = SchemaParser::new(src, &mut diagnostics).parse();
    let root = tree.root().unwrap();
    let cst = Schema::cast(root).unwrap();
    let _ast = SchemaLowerer::new(&mut diagnostics).lower(cst);
});
