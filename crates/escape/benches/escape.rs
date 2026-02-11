#![expect(clippy::unreachable, reason = "Benches")]

use core::hint::black_box;

use divan::counter::BytesCount;
use divan::{AllocProfiler, Bencher};
use duramen_escape::Escaper;
use duramen_test::rand::rngs::SmallRng;
use duramen_test::rand::{Rng as _, SeedableRng as _};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

const SEED: u64 = 42;
const COUNT: usize = 100_000;

fn main() {
    divan::main();
}

fn generate_inputs(percent: u8) -> (Vec<String>, usize) {
    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut inputs = Vec::with_capacity(COUNT);
    let mut bytes = 0;

    for _ in 0..COUNT {
        let mut string = String::new();
        for _ in 0..20 {
            if rng.random_range(0..100) < percent {
                match rng.random_range(0..4) {
                    0 => string.push_str(r"\\"),
                    1 => string.push_str(r"\n"),
                    2 => string.push_str(r"\x41"),
                    3 => string.push_str(r"\u{1F600}"),
                    _ => unreachable!(),
                }
            } else {
                let ch = rng.random_range(b'a'..=b'z');
                string.push(ch as char);
            }
        }

        bytes += string.len();
        inputs.push(string);
    }

    (inputs, bytes)
}

fn generate_pattern_inputs(escape_percent: u8) -> (Vec<String>, usize) {
    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut inputs = Vec::with_capacity(COUNT);
    let mut bytes = 0;

    for _ in 0..COUNT {
        let mut string = String::new();
        for _ in 0..20 {
            if rng.random_range(0..100) < escape_percent {
                match rng.random_range(0..3) {
                    0 => string.push('*'),
                    1 => string.push_str(r"\*"),
                    2 => string.push_str(r"\n"),
                    _ => unreachable!(),
                }
            } else {
                let ch = rng.random_range(b'a'..=b'z');
                string.push(ch as char);
            }
        }

        bytes += string.len();
        inputs.push(string);
    }

    (inputs, bytes)
}

#[divan::bench]
fn unescape_str_none(bencher: Bencher<'_, '_>) {
    let (inputs, bytes) = generate_inputs(0);

    bencher.counter(BytesCount::new(bytes)).bench(|| {
        for input in &inputs {
            drop(black_box(Escaper::new(black_box(input)).unescape_str()));
        }
    });
}

#[divan::bench]
fn unescape_str(bencher: Bencher<'_, '_>) {
    let (inputs, bytes) = generate_inputs(50);

    bencher.counter(BytesCount::new(bytes)).bench(|| {
        for input in &inputs {
            drop(black_box(Escaper::new(black_box(input)).unescape_str()));
        }
    });
}

#[divan::bench]
fn unescape_str_rare(bencher: Bencher<'_, '_>) {
    let (inputs, bytes) = generate_inputs(2);

    bencher.counter(BytesCount::new(bytes)).bench(|| {
        for input in &inputs {
            drop(black_box(Escaper::new(black_box(input)).unescape_str()));
        }
    });
}

#[divan::bench]
fn unescape_pattern_none(bencher: Bencher<'_, '_>) {
    let (inputs, bytes) = generate_pattern_inputs(0);

    bencher.counter(BytesCount::new(bytes)).bench(|| {
        for input in &inputs {
            drop(black_box(Escaper::new(black_box(input)).unescape_pattern()));
        }
    });
}

#[divan::bench]
fn unescape_pattern(bencher: Bencher<'_, '_>) {
    let (inputs, bytes) = generate_pattern_inputs(50);

    bencher.counter(BytesCount::new(bytes)).bench(|| {
        for input in &inputs {
            drop(black_box(Escaper::new(black_box(input)).unescape_pattern()));
        }
    });
}

#[divan::bench]
fn unescape_pattern_rare(bencher: Bencher<'_, '_>) {
    let (inputs, bytes) = generate_pattern_inputs(2);

    bencher.counter(BytesCount::new(bytes)).bench(|| {
        for input in &inputs {
            drop(black_box(Escaper::new(black_box(input)).unescape_pattern()));
        }
    });
}
