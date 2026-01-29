#![no_main]

use duramen_lower::SchemaLowerer;
use duramen_parser::SchemaParser;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|source: &str| {
    let result = SchemaParser::new(source).parse();
    drop(SchemaLowerer::new(source).lower(result.tree()));
});
