use core::hint::black_box;

use divan::counter::ItemsCount;
use divan::{AllocProfiler, Bencher};
use duramen::est::json::{PolicySet, SchemaFragment};
use duramen::lower::{PolicyLowerer, SchemaLowerer};
use duramen::parser::{PolicyParser, SchemaParser};
use duramen_test::{POLICIES, SCHEMAS};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn e2e_serde_policy(bencher: Bencher<'_, '_>) {
    let sources: Vec<String> = POLICIES
        .iter()
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .collect();

    bencher.counter(ItemsCount::new(sources.len())).bench(|| {
        for source in &sources {
            let result = PolicyParser::new(source).parse();
            let Ok((templates, _)) = PolicyLowerer::new(source).lower(result.tree()) else {
                continue;
            };

            let policy_set = PolicySet::from(templates.as_slice());
            drop(black_box(serde_json::to_string(&policy_set)));
        }
    });
}

#[divan::bench]
fn e2e_serde_schema(bencher: Bencher<'_, '_>) {
    let sources: Vec<String> = SCHEMAS
        .iter()
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .collect();

    bencher.counter(ItemsCount::new(sources.len())).bench(|| {
        for source in &sources {
            let result = SchemaParser::new(source).parse();
            let Ok((schema, _)) = SchemaLowerer::new(source).lower(result.tree()) else {
                continue;
            };

            let fragment = SchemaFragment::from(&schema);
            drop(black_box(serde_json::to_string(&fragment)));
        }
    });
}
