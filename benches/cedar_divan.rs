#![expect(clippy::unwrap_used, reason = "Benchmarks")]

mod data;

use data::{BenchInput, POLICIES, SCHEMAS};
use divan::AllocProfiler;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench_group(name = "parse")]
mod parse {
    use super::{BenchInput, POLICIES, SCHEMAS};

    #[divan::bench(args = POLICIES)]
    fn policyset(input: &BenchInput) {
        let _output = cedar_policy_core::parser::parse_policyset(input.content);
    }

    #[divan::bench(args = SCHEMAS)]
    fn schema(input: &BenchInput) {
        let _output =
            cedar_policy_core::validator::cedar_schema::parser::parse_schema(input.content);
    }
}

#[divan::bench_group(name = "serde")]
mod serde {
    use super::{BenchInput, POLICIES, SCHEMAS};

    #[divan::bench(args = POLICIES)]
    fn policyset(input: &BenchInput) {
        let Ok((ests, _pset)) =
            cedar_policy_core::parser::parse_policyset_to_ests_and_pset(input.content)
        else {
            return;
        };
        let _json = serde_json::to_value(ests).unwrap();
    }

    #[divan::bench(args = SCHEMAS)]
    fn schema(input: &BenchInput) {
        let (schema, _warnings) =
            cedar_policy::SchemaFragment::from_cedarschema_str(input.content).unwrap();
        let _json = schema.to_json_value().unwrap();
    }
}

#[divan::bench_group(name = "prost")]
mod prost {
    use core::str::FromStr as _;

    use cedar_policy::proto::traits::Protobuf as _;

    use super::{BenchInput, POLICIES};

    #[divan::bench(args = POLICIES)]
    fn policyset(input: &BenchInput) {
        let Ok(pset) = cedar_policy::PolicySet::from_str(input.content) else {
            return;
        };
        let _bytes = pset.encode();
    }
}
