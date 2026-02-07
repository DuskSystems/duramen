#![no_main]

use duramen::lexer::Lexer;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|src: &str| {
    if let Ok(lexer) = Lexer::new(src) {
        lexer.count();
    }
});
