#![expect(clippy::unwrap_used, reason = "Benches")]

use core::hint::black_box;

use divan::counter::ItemsCount;
use divan::{AllocProfiler, Bencher};
use duramen_lowerer::{PolicyLowerer, SchemaLowerer};
use duramen_parser::{PolicyParser, SchemaParser};
use duramen_test::{CORPUS_POLICIES, CORPUS_SCHEMAS};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn lower_policy(bencher: Bencher<'_, '_>) {
    let sources: Vec<_> = CORPUS_POLICIES
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect();

    bencher
        .counter(ItemsCount::new(sources.len()))
        .with_inputs(|| {
            sources
                .iter()
                .map(|source| PolicyParser::parse(source))
                .collect::<Vec<_>>()
        })
        .bench_values(|parsed| {
            for (tree, diagnostics) in parsed {
                black_box(PolicyLowerer::lower(&tree, diagnostics));
            }
        });
}

#[divan::bench]
fn lower_schema(bencher: Bencher<'_, '_>) {
    let sources: Vec<_> = CORPUS_SCHEMAS
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect();

    bencher
        .counter(ItemsCount::new(sources.len()))
        .with_inputs(|| {
            sources
                .iter()
                .map(|source| SchemaParser::parse(source))
                .collect::<Vec<_>>()
        })
        .bench_values(|parsed| {
            for (tree, diagnostics) in parsed {
                black_box(SchemaLowerer::lower(&tree, diagnostics));
            }
        });
}
