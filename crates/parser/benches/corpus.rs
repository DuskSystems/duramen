#![expect(clippy::unwrap_used, reason = "Benchmarks")]

use core::hint::black_box;

use divan::counter::ItemsCount;
use divan::{AllocProfiler, Bencher};
use duramen_parser::{PolicyParser, SchemaParser};
use duramen_test::{POLICIES, SCHEMAS};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn policy(bencher: Bencher<'_, '_>) {
    let sources: Vec<_> = POLICIES
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect();

    bencher.counter(ItemsCount::new(sources.len())).bench(|| {
        for source in &sources {
            black_box(PolicyParser::new(source).parse());
        }
    });
}

#[divan::bench]
fn schema(bencher: Bencher<'_, '_>) {
    let sources: Vec<_> = SCHEMAS
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect();

    bencher.counter(ItemsCount::new(sources.len())).bench(|| {
        for source in &sources {
            black_box(SchemaParser::new(source).parse());
        }
    });
}
