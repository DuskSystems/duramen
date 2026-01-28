#![no_main]

use duramen_parser::SchemaParser;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|src: &str| {
    drop(SchemaParser::new(src).parse());
});
