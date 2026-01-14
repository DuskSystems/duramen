#![expect(clippy::unwrap_used, reason = "Benchmarks")]
#![expect(
    unsafe_code,
    reason = "Gungraun: https://github.com/gungraun/gungraun/issues/490"
)]

mod data;

use core::hint::black_box;
use core::str::FromStr as _;

use cedar_policy::proto::traits::Protobuf as _;
use data::{POLICIES, SCHEMAS};
use gungraun::{library_benchmark, library_benchmark_group, main};

library_benchmark_group!(
    name = parse_group;
    benchmarks =
        duramen_parse_policyset,
        duramen_parse_schema,
        cedar_parse_policyset,
        cedar_parse_schema,
);

library_benchmark_group!(
    name = serde_group;
    benchmarks =
        duramen_serde_policyset,
        duramen_serde_schema,
        cedar_serde_policyset,
        cedar_serde_schema,
);

library_benchmark_group!(
    name = prost_group;
    benchmarks =
        duramen_prost_policyset,
        cedar_prost_policyset,
);

main!(
    library_benchmark_groups = parse_group,
    serde_group,
    prost_group
);

#[library_benchmark]
fn duramen_parse_policyset() {
    for input in black_box(POLICIES) {
        let _output = black_box(duramen::policy::PolicySet::parse(black_box(input.content)));
    }
}

#[library_benchmark]
fn duramen_parse_schema() {
    for input in black_box(SCHEMAS) {
        let _output = black_box(duramen::schema::Schema::parse(black_box(input.content)));
    }
}

#[library_benchmark]
fn cedar_parse_policyset() {
    for input in black_box(POLICIES) {
        let _output = black_box(cedar_policy_core::parser::parse_policyset(black_box(
            input.content,
        )));
    }
}

#[library_benchmark]
fn cedar_parse_schema() {
    for input in black_box(SCHEMAS) {
        let _output = black_box(
            cedar_policy_core::validator::cedar_schema::parser::parse_schema(black_box(
                input.content,
            )),
        );
    }
}

#[library_benchmark]
fn duramen_serde_policyset() {
    for input in black_box(POLICIES) {
        let pset = duramen::policy::PolicySet::parse(black_box(input.content));
        if pset.has_errors() {
            continue;
        }
        let _json = black_box(pset.to_serde_json_value().unwrap());
    }
}

#[library_benchmark]
fn duramen_serde_schema() {
    for input in black_box(SCHEMAS) {
        let schema = duramen::schema::Schema::parse(black_box(input.content));
        let _json = black_box(schema.to_serde_json_value());
    }
}

#[library_benchmark]
fn cedar_serde_policyset() {
    for input in black_box(POLICIES) {
        let Ok((ests, _pset)) =
            cedar_policy_core::parser::parse_policyset_to_ests_and_pset(black_box(input.content))
        else {
            continue;
        };
        let _json = black_box(serde_json::to_value(ests).unwrap());
    }
}

#[library_benchmark]
fn cedar_serde_schema() {
    for input in black_box(SCHEMAS) {
        let (schema, _warnings) =
            cedar_policy::SchemaFragment::from_cedarschema_str(black_box(input.content)).unwrap();
        let _json = black_box(schema.to_json_value().unwrap());
    }
}

#[library_benchmark]
fn duramen_prost_policyset() {
    for input in black_box(POLICIES) {
        let pset = duramen::policy::PolicySet::parse(black_box(input.content));
        if pset.has_errors() {
            continue;
        }
        let _bytes = black_box(pset.to_prost_bytes().unwrap());
    }
}

#[library_benchmark]
fn cedar_prost_policyset() {
    for input in black_box(POLICIES) {
        let Ok(pset) = cedar_policy::PolicySet::from_str(black_box(input.content)) else {
            continue;
        };
        let _bytes = black_box(pset.encode());
    }
}
