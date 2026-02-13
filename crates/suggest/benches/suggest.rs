use core::hint::black_box;

use divan::{AllocProfiler, Bencher};
use duramen_suggest::suggest;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn suggest_few(bencher: Bencher<'_, '_>) {
    let candidates = &["principal", "action", "resource", "context"];

    bencher.bench(|| {
        black_box(suggest(black_box("princpal"), black_box(candidates)));
    });
}

#[divan::bench]
fn suggest_many(bencher: Bencher<'_, '_>) {
    let candidates = &[
        "contains",
        "containsAll",
        "containsAny",
        "getTag",
        "hasTag",
        "ip",
        "isIpv4",
        "isIpv6",
        "isInRange",
        "isLoopback",
        "isMulticast",
        "decimal",
        "lessThan",
        "lessThanOrEqual",
        "greaterThan",
        "greaterThanOrEqual",
        "toDate",
        "toTime",
        "toDays",
        "toHours",
        "toMinutes",
        "toSeconds",
        "toMilliseconds",
        "offset",
        "durationSince",
    ];

    bencher.bench(|| {
        black_box(suggest(black_box("contans"), black_box(candidates)));
    });
}

#[divan::bench]
fn suggest_exact(bencher: Bencher<'_, '_>) {
    let candidates = &["principal", "action", "resource", "context"];

    bencher.bench(|| {
        black_box(suggest(black_box("principal"), black_box(candidates)));
    });
}

#[divan::bench]
fn suggest_none(bencher: Bencher<'_, '_>) {
    let candidates = &["principal", "action", "resource", "context"];

    bencher.bench(|| {
        black_box(suggest(black_box("zzzzzzzzz"), black_box(candidates)));
    });
}
