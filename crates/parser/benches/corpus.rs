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
    let files = &*POLICIES;
    bencher.counter(ItemsCount::new(files.len())).bench(|| {
        for source in files {
            drop(black_box(PolicyParser::new(source).parse()));
        }
    });
}

#[divan::bench]
fn schema(bencher: Bencher<'_, '_>) {
    let files = &*SCHEMAS;
    bencher.counter(ItemsCount::new(files.len())).bench(|| {
        for source in files {
            drop(black_box(SchemaParser::new(source).parse()));
        }
    });
}
