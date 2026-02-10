#![expect(clippy::unwrap_used, reason = "Benches")]

use core::hint::black_box;

use divan::counter::ItemsCount;
use divan::{AllocProfiler, Bencher};
use duramen_cst::{CstNode as _, Policies, Schema};
use duramen_diagnostic::Diagnostics;
use duramen_lower::{PolicyLowerer, SchemaLowerer};
use duramen_parser::{PolicyParser, SchemaParser};
use duramen_test::{POLICIES, SCHEMAS};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn lower_policy(bencher: Bencher<'_, '_>) {
    let sources: Vec<_> = POLICIES
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect();

    let trees: Vec<_> = sources
        .iter()
        .map(|source| {
            let mut diagnostics = Diagnostics::new();
            PolicyParser::new(source, &mut diagnostics).parse()
        })
        .collect();

    bencher.counter(ItemsCount::new(sources.len())).bench(|| {
        for (source, tree) in sources.iter().zip(&trees) {
            let mut diagnostics = Diagnostics::new();
            let root = tree.root().unwrap();
            let cst = Policies::cast(root).unwrap();
            black_box(PolicyLowerer::new(source, &mut diagnostics).lower(cst));
        }
    });
}

#[divan::bench]
fn lower_schema(bencher: Bencher<'_, '_>) {
    let sources: Vec<_> = SCHEMAS
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect();

    let trees: Vec<_> = sources
        .iter()
        .map(|source| {
            let mut diagnostics = Diagnostics::new();
            SchemaParser::new(source, &mut diagnostics).parse()
        })
        .collect();

    bencher.counter(ItemsCount::new(sources.len())).bench(|| {
        for (source, tree) in sources.iter().zip(&trees) {
            let mut diagnostics = Diagnostics::new();
            let root = tree.root().unwrap();
            let cst = Schema::cast(root).unwrap();
            black_box(SchemaLowerer::new(source, &mut diagnostics).lower(cst));
        }
    });
}
