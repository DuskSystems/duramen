#![no_main]

use duramen_lexer::Lexer;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|src: &str| {
    Lexer::new(src).count();
});
