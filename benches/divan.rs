#![expect(clippy::unwrap_used, reason = "Benchmarks")]

mod data;

use data::{BenchInput, POLICIES, SCHEMAS};
use divan::AllocProfiler;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench_group(name = "duramen_parse")]
mod duramen_parse {
    use duramen::policy::PolicySet;
    use duramen::schema::Schema;

    use super::{BenchInput, POLICIES, SCHEMAS};

    #[divan::bench(args = POLICIES)]
    fn policyset(input: &BenchInput) {
        let _output = PolicySet::parse(input.content);
    }

    #[divan::bench(args = SCHEMAS)]
    fn schema(input: &BenchInput) {
        let _output = Schema::parse(input.content);
    }
}

#[divan::bench_group(name = "duramen_serde")]
mod duramen_serde {
    use duramen::policy::PolicySet;
    use duramen::schema::Schema;

    use super::{BenchInput, POLICIES, SCHEMAS};

    #[divan::bench(args = POLICIES)]
    fn policyset(input: &BenchInput) {
        let pset = PolicySet::parse(input.content);
        if pset.has_errors() {
            return;
        }
        let _json = pset.to_serde_json_value().unwrap();
    }

    #[divan::bench(args = SCHEMAS)]
    fn schema(input: &BenchInput) {
        let schema = Schema::parse(input.content);
        let _json = schema.to_serde_json_value();
    }
}

#[divan::bench_group(name = "duramen_facet")]
mod duramen_facet {
    use duramen::policy::PolicySet;
    use duramen::schema::Schema;

    use super::{BenchInput, POLICIES, SCHEMAS};

    #[divan::bench(args = POLICIES)]
    fn policyset(input: &BenchInput) {
        let pset = PolicySet::parse(input.content);
        if pset.has_errors() {
            return;
        }
        let _json = pset.to_facet_json().unwrap();
    }

    #[divan::bench(args = SCHEMAS)]
    fn schema(input: &BenchInput) {
        let schema = Schema::parse(input.content);
        let _json = schema.to_facet_json().unwrap();
    }
}

#[divan::bench_group(name = "duramen_prost")]
mod duramen_prost {
    use duramen::policy::PolicySet;

    use super::{BenchInput, POLICIES};

    #[divan::bench(args = POLICIES)]
    fn policyset(input: &BenchInput) {
        let pset = PolicySet::parse(input.content);
        if pset.has_errors() {
            return;
        }
        let _bytes = pset.to_prost_bytes().unwrap();
    }
}

#[divan::bench_group(name = "cedar_parse")]
mod cedar_parse {
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

#[divan::bench_group(name = "cedar_serde")]
mod cedar_serde {
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

#[divan::bench_group(name = "cedar_prost")]
mod cedar_prost {
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
