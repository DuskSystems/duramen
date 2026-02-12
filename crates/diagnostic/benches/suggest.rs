use core::hint::black_box;

use divan::{AllocProfiler, Bencher};
use duramen_diagnostic::suggest;

const CANDIDATES: &[&str] = &["principal", "action", "resource", "context"];

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn suggest_exact(bencher: Bencher<'_, '_>) {
    bencher.bench(|| suggest(black_box("principal"), black_box(CANDIDATES)));
}

#[divan::bench]
fn suggest_close(bencher: Bencher<'_, '_>) {
    bencher.bench(|| suggest(black_box("princpal"), black_box(CANDIDATES)));
}

#[divan::bench]
fn suggest_none(bencher: Bencher<'_, '_>) {
    bencher.bench(|| suggest(black_box("foobar"), black_box(CANDIDATES)));
}

#[divan::bench]
fn suggest_transposition(bencher: Bencher<'_, '_>) {
    bencher.bench(|| suggest(black_box("prinicpal"), black_box(CANDIDATES)));
}
