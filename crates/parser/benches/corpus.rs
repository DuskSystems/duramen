#![expect(clippy::unwrap_used, reason = "Benches")]

use core::hint::black_box;

use divan::counter::ItemsCount;
use divan::{AllocProfiler, Bencher};
use duramen_diagnostic::Diagnostics;
use duramen_parser::{PolicyParser, SchemaParser};
use duramen_test::{CORPUS_POLICIES, CORPUS_SCHEMAS};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn parser_policy(bencher: Bencher<'_, '_>) {
    let sources: Vec<_> = CORPUS_POLICIES
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect();

    bencher.counter(ItemsCount::new(sources.len())).bench(|| {
        for source in &sources {
            let mut diagnostics = Diagnostics::new();
            black_box(PolicyParser::new(source, &mut diagnostics).parse());
        }
    });
}

#[divan::bench]
fn parser_schema(bencher: Bencher<'_, '_>) {
    let sources: Vec<_> = CORPUS_SCHEMAS
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect();

    bencher.counter(ItemsCount::new(sources.len())).bench(|| {
        for source in &sources {
            let mut diagnostics = Diagnostics::new();
            black_box(SchemaParser::new(source, &mut diagnostics).parse());
        }
    });
}
