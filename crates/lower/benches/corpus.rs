#![expect(clippy::unwrap_used, reason = "Benchmarks")]

use core::hint::black_box;

use divan::counter::ItemsCount;
use divan::{AllocProfiler, Bencher};
use duramen_lower::{PolicyLowerer, SchemaLowerer};
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

    let parsed: Vec<_> = sources
        .iter()
        .map(|source| PolicyParser::new(source).parse())
        .collect();

    bencher
        .counter(ItemsCount::new(sources.len()))
        .bench_local(|| {
            for (source, result) in sources.iter().zip(parsed.iter()) {
                let lowerer = PolicyLowerer::new(source);
                drop(black_box(lowerer.lower(result.tree())));
            }
        });
}

#[divan::bench]
fn schema(bencher: Bencher<'_, '_>) {
    let sources: Vec<_> = SCHEMAS
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect();

    let parsed: Vec<_> = sources
        .iter()
        .map(|source| SchemaParser::new(source).parse())
        .collect();

    bencher
        .counter(ItemsCount::new(sources.len()))
        .bench_local(|| {
            for (source, result) in sources.iter().zip(parsed.iter()) {
                let lowerer = SchemaLowerer::new(source);
                drop(black_box(lowerer.lower(result.tree())));
            }
        });
}
