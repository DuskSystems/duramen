#![expect(clippy::indexing_slicing, reason = "Benches")]

use core::hint::black_box;

use divan::counter::BytesCount;
use divan::{AllocProfiler, Bencher};
use duramen_lexer::Lexer;
use rand::rngs::SmallRng;
use rand::{Rng as _, SeedableRng as _};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

const SEED: u64 = 42;
const COUNT: usize = 1_000_000;

fn main() {
    divan::main();
}

#[divan::bench]
fn lexer_integers(bencher: Bencher<'_, '_>) {
    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut input = String::new();

    for _ in 0..COUNT {
        let num: u64 = rng.random();
        input.push_str(&num.to_string());
        input.push(' ');
    }

    bencher
        .counter(BytesCount::of_str(&input))
        .bench(|| black_box(Lexer::new(black_box(&input)).count()));
}

#[divan::bench]
fn lexer_strings(bencher: Bencher<'_, '_>) {
    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut input = String::new();

    for _ in 0..COUNT {
        input.push('"');
        for _ in 0..10 {
            let byte = rng.random_range(b'a'..=b'z');
            input.push(byte as char);
        }

        input.push_str("\" ");
    }

    bencher
        .counter(BytesCount::of_str(&input))
        .bench(|| black_box(Lexer::new(black_box(&input)).count()));
}

#[divan::bench]
fn lexer_identifiers(bencher: Bencher<'_, '_>) {
    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut input = String::new();

    for _ in 0..COUNT {
        let byte = rng.random_range(b'a'..=b'z');
        input.push(byte as char);

        for _ in 0..9 {
            let byte = if rng.random_bool(0.5) {
                rng.random_range(b'a'..=b'z')
            } else {
                rng.random_range(b'0'..=b'9')
            };

            input.push(byte as char);
        }

        input.push(' ');
    }

    bencher
        .counter(BytesCount::of_str(&input))
        .bench(|| black_box(Lexer::new(black_box(&input)).count()));
}

#[divan::bench]
fn lexer_punctuation(bencher: Bencher<'_, '_>) {
    const PUNCTUATIONS: &[u8] = b"(){}[],;:.@?+-*/%=!<>&|";

    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut input = String::new();

    for _ in 0..COUNT {
        let index = rng.random_range(0..PUNCTUATIONS.len());
        let Some(&byte) = PUNCTUATIONS.get(index) else {
            continue;
        };

        input.push(byte as char);
    }

    bencher
        .counter(BytesCount::of_str(&input))
        .bench(|| black_box(Lexer::new(black_box(&input)).count()));
}

#[divan::bench]
fn lexer_comments(bencher: Bencher<'_, '_>) {
    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut input = String::new();

    for _ in 0..COUNT {
        input.push_str("// ");
        for _ in 0..10 {
            let byte = rng.random_range(b'a'..=b'z');
            input.push(byte as char);
        }

        input.push('\n');
    }

    bencher
        .counter(BytesCount::of_str(&input))
        .bench(|| black_box(Lexer::new(black_box(&input)).count()));
}

#[divan::bench]
fn lexer_whitespace_ascii(bencher: Bencher<'_, '_>) {
    const WHITESPACES: &[char] = &[' ', '\t', '\n', '\r', '\x0B', '\x0C'];

    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut input = String::new();

    for _ in 0..COUNT {
        let index = rng.random_range(0..WHITESPACES.len());
        input.push(WHITESPACES[index]);
    }

    bencher
        .counter(BytesCount::of_str(&input))
        .bench(|| black_box(Lexer::new(black_box(&input)).count()));
}

#[divan::bench]
fn lexer_whitespace_unicode(bencher: Bencher<'_, '_>) {
    const WHITESPACES: &[char] = &['\u{00A0}', '\u{2003}', '\u{2009}', '\u{3000}'];

    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut input = String::new();

    for _ in 0..COUNT {
        let index = rng.random_range(0..WHITESPACES.len());
        input.push(WHITESPACES[index]);
    }

    bencher
        .counter(BytesCount::of_str(&input))
        .bench(|| black_box(Lexer::new(black_box(&input)).count()));
}
