#![no_main]

use duramen_lower::PolicyLowerer;
use duramen_parser::PolicyParser;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|source: &str| {
    let result = PolicyParser::new(source).parse();
    drop(PolicyLowerer::new(source).lower(result.tree()));
});
