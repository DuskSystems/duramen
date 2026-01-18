#![expect(clippy::unwrap_used, reason = "Benchmarks")]
#![expect(
    unsafe_code,
    reason = "Gungraun: https://github.com/gungraun/gungraun/issues/490"
)]

mod data;

use core::hint::black_box;

use data::{POLICIES, SCHEMAS};
use gungraun::{library_benchmark, library_benchmark_group, main};

library_benchmark_group!(
    name = parse_group;
    benchmarks =
        parse_policyset,
        parse_schema,
);

library_benchmark_group!(
    name = serde_group;
    benchmarks =
        serde_policyset,
        serde_schema,
);

library_benchmark_group!(
    name = facet_group;
    benchmarks =
        facet_policyset,
        facet_schema,
);

library_benchmark_group!(
    name = prost_group;
    benchmarks =
        prost_policyset,
);

main!(
    library_benchmark_groups = parse_group,
    serde_group,
    facet_group,
    prost_group
);

#[library_benchmark]
fn parse_policyset() {
    for input in black_box(POLICIES) {
        let _output = black_box(duramen::policy::PolicySet::parse(black_box(input.content)));
    }
}

#[library_benchmark]
fn parse_schema() {
    for input in black_box(SCHEMAS) {
        let _output = black_box(duramen::schema::Schema::parse(black_box(input.content)));
    }
}

#[library_benchmark]
fn serde_policyset() {
    for input in black_box(POLICIES) {
        let pset = duramen::policy::PolicySet::parse(black_box(input.content));
        if pset.has_errors() {
            continue;
        }
        let _json = black_box(pset.to_serde_json_value().unwrap());
    }
}

#[library_benchmark]
fn serde_schema() {
    for input in black_box(SCHEMAS) {
        let schema = duramen::schema::Schema::parse(black_box(input.content));
        let _json = black_box(schema.to_serde_json_value());
    }
}

#[library_benchmark]
fn facet_policyset() {
    for input in black_box(POLICIES) {
        let pset = duramen::policy::PolicySet::parse(black_box(input.content));
        if pset.has_errors() {
            continue;
        }
        let _json = black_box(pset.to_facet_json().unwrap());
    }
}

#[library_benchmark]
fn facet_schema() {
    for input in black_box(SCHEMAS) {
        let schema = duramen::schema::Schema::parse(black_box(input.content));
        let _json = black_box(schema.to_facet_json().unwrap());
    }
}

#[library_benchmark]
fn prost_policyset() {
    for input in black_box(POLICIES) {
        let pset = duramen::policy::PolicySet::parse(black_box(input.content));
        if pset.has_errors() {
            continue;
        }
        let _bytes = black_box(pset.to_prost_bytes().unwrap());
    }
}
