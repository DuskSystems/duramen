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

#[divan::bench_group(name = "serde")]
mod serde {
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

#[divan::bench_group(name = "facet")]
mod facet {
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

#[divan::bench_group(name = "prost")]
mod prost {
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
