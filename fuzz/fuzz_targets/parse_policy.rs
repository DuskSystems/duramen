#![no_main]

use duramen_parser::PolicyParser;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|src: &str| {
    drop(PolicyParser::new(src).parse());
});
