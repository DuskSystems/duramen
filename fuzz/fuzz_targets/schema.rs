#![no_main]

use duramen::schema::Schema;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(source) = core::str::from_utf8(data) {
        drop(Schema::parse(source));
    }
});
