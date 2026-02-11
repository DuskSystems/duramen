#![no_main]

use duramen::escape::Escaper;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|src: &str| {
    let escaper = Escaper::new(src);
    drop(escaper.unescape_str());

    let escaper = Escaper::new(src);
    drop(escaper.unescape_pattern());
});
