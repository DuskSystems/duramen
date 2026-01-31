#![expect(clippy::unwrap_used, reason = "Benchmarks")]

use core::hint::black_box;

use divan::counter::ItemsCount;
use divan::{AllocProfiler, Bencher};
use duramen_ast as ast;
use duramen_est::json::{Policy, SchemaFragment};
use duramen_lower::{PolicyLowerer, SchemaLowerer};
use duramen_parser::{PolicyParser, SchemaParser};
use duramen_test::{POLICIES, SCHEMAS};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn est_serde_policy_serialize(bencher: Bencher<'_, '_>) {
    let templates: Vec<ast::policy::Template> = POLICIES
        .iter()
        .filter_map(|path| {
            let source = std::fs::read_to_string(path).unwrap();
            let result = PolicyParser::new(&source).parse();
            PolicyLowerer::new(&source).lower(result.tree()).ok()
        })
        .flat_map(|(templates, _)| templates)
        .collect();

    bencher.counter(ItemsCount::new(templates.len())).bench(|| {
        for template in &templates {
            let est_policy = Policy::from(template);
            drop(black_box(serde_json::to_string(&est_policy)));
        }
    });
}

#[divan::bench]
fn est_serde_policy_deserialize(bencher: Bencher<'_, '_>) {
    let templates: Vec<ast::policy::Template> = POLICIES
        .iter()
        .filter_map(|path| {
            let source = std::fs::read_to_string(path).unwrap();
            let result = PolicyParser::new(&source).parse();
            PolicyLowerer::new(&source).lower(result.tree()).ok()
        })
        .flat_map(|(templates, _)| templates)
        .collect();

    let json_strings: Vec<String> = templates
        .iter()
        .map(|template| serde_json::to_string(&Policy::from(template)).unwrap())
        .collect();

    bencher
        .counter(ItemsCount::new(json_strings.len()))
        .bench(|| {
            for json in &json_strings {
                let est: Policy = serde_json::from_str(json).unwrap();
                let ast: ast::policy::Template = ast::policy::Template::from(&est);
                drop(black_box(ast));
            }
        });
}

#[divan::bench]
fn est_serde_policy_roundtrip(bencher: Bencher<'_, '_>) {
    let templates: Vec<ast::policy::Template> = POLICIES
        .iter()
        .filter_map(|path| {
            let source = std::fs::read_to_string(path).unwrap();
            let result = PolicyParser::new(&source).parse();
            PolicyLowerer::new(&source).lower(result.tree()).ok()
        })
        .flat_map(|(templates, _)| templates)
        .collect();

    bencher.counter(ItemsCount::new(templates.len())).bench(|| {
        for template in &templates {
            let est = Policy::from(template);
            let json = serde_json::to_string(&est).unwrap();
            let est: Policy = serde_json::from_str(&json).unwrap();
            let ast: ast::policy::Template = ast::policy::Template::from(&est);
            drop(black_box(ast));
        }
    });
}

#[divan::bench]
fn est_serde_schema_serialize(bencher: Bencher<'_, '_>) {
    let schemas: Vec<ast::schema::Schema> = SCHEMAS
        .iter()
        .filter_map(|path| {
            let source = std::fs::read_to_string(path).unwrap();
            let result = SchemaParser::new(&source).parse();
            SchemaLowerer::new(&source).lower(result.tree()).ok()
        })
        .map(|(schema, _)| schema)
        .collect();

    bencher.counter(ItemsCount::new(schemas.len())).bench(|| {
        for schema in &schemas {
            let fragment = SchemaFragment::from(schema);
            drop(black_box(serde_json::to_string(&fragment)));
        }
    });
}

#[divan::bench]
fn est_serde_schema_deserialize(bencher: Bencher<'_, '_>) {
    let schemas: Vec<ast::schema::Schema> = SCHEMAS
        .iter()
        .filter_map(|path| {
            let source = std::fs::read_to_string(path).unwrap();
            let result = SchemaParser::new(&source).parse();
            SchemaLowerer::new(&source).lower(result.tree()).ok()
        })
        .map(|(schema, _)| schema)
        .collect();

    let json_strings: Vec<String> = schemas
        .iter()
        .map(|schema| serde_json::to_string(&SchemaFragment::from(schema)).unwrap())
        .collect();

    bencher
        .counter(ItemsCount::new(json_strings.len()))
        .bench(|| {
            for json in &json_strings {
                let est: SchemaFragment = serde_json::from_str(json).unwrap();
                let ast: ast::schema::Schema = ast::schema::Schema::from(&est);
                drop(black_box(ast));
            }
        });
}

#[divan::bench]
fn est_serde_schema_roundtrip(bencher: Bencher<'_, '_>) {
    let schemas: Vec<ast::schema::Schema> = SCHEMAS
        .iter()
        .filter_map(|path| {
            let source = std::fs::read_to_string(path).unwrap();
            let result = SchemaParser::new(&source).parse();
            SchemaLowerer::new(&source).lower(result.tree()).ok()
        })
        .map(|(schema, _)| schema)
        .collect();

    bencher.counter(ItemsCount::new(schemas.len())).bench(|| {
        for schema in &schemas {
            let est = SchemaFragment::from(schema);
            let json = serde_json::to_string(&est).unwrap();
            let est: SchemaFragment = serde_json::from_str(&json).unwrap();
            let ast: ast::schema::Schema = ast::schema::Schema::from(&est);
            drop(black_box(ast));
        }
    });
}
