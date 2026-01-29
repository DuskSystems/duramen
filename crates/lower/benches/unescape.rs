#![expect(clippy::arithmetic_side_effects, reason = "Benchmarks")]

use core::hint::black_box;

use divan::counter::BytesCount;
use divan::{AllocProfiler, Bencher};
use duramen_lower::unescape::{PatternUnescaper, StringUnescaper};
use rand::rngs::SmallRng;
use rand::{Rng as _, SeedableRng as _};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

const SEED: u64 = 42;
const COUNT: usize = 100_000;

fn main() {
    divan::main();
}

#[divan::bench]
fn string(bencher: Bencher<'_, '_>) {
    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut inputs = Vec::with_capacity(COUNT);

    let mut bytes = 0;
    for _ in 0..COUNT {
        let mut string = String::new();

        for _ in 0..20 {
            match rng.random_range(0..10) {
                0 => string.push_str(r"\\"),
                1 => string.push_str(r"\n"),
                2 => string.push_str(r"\x41"),
                3 => string.push_str(r"\u{1F600}"),
                _ => {
                    let char = rng.random_range(b'a'..=b'z');
                    string.push(char as char);
                }
            }
        }

        bytes += string.len();
        inputs.push(string);
    }

    bencher.counter(BytesCount::new(bytes)).bench(|| {
        for input in &inputs {
            drop(black_box(StringUnescaper::new(black_box(input)).unescape()));
        }
    });
}

#[divan::bench]
fn pattern(bencher: Bencher<'_, '_>) {
    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut inputs = Vec::with_capacity(COUNT);

    let mut bytes = 0;
    for _ in 0..COUNT {
        let mut string = String::new();

        for _ in 0..20 {
            match rng.random_range(0..10) {
                0 => string.push('*'),
                1 => string.push_str(r"\*"),
                2 => string.push_str(r"\n"),
                _ => {
                    let char = rng.random_range(b'a'..=b'z');
                    string.push(char as char);
                }
            }
        }

        bytes += string.len();
        inputs.push(string);
    }

    bencher.counter(BytesCount::new(bytes)).bench(|| {
        for input in &inputs {
            drop(black_box(
                PatternUnescaper::new(black_box(input)).unescape(),
            ));
        }
    });
}
