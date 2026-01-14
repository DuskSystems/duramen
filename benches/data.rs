use core::fmt;

pub const POLICIES: &[BenchInput] = &[
    BenchInput {
        name: "decimal_1",
        content: include_str!("../cedar-integration-tests/tests/decimal/policies_1.cedar"),
    },
    BenchInput {
        name: "decimal_2",
        content: include_str!("../cedar-integration-tests/tests/decimal/policies_2.cedar"),
    },
    BenchInput {
        name: "example_1a",
        content: include_str!(
            "../cedar-integration-tests/tests/example_use_cases/policies_1a.cedar"
        ),
    },
    BenchInput {
        name: "example_2a",
        content: include_str!(
            "../cedar-integration-tests/tests/example_use_cases/policies_2a.cedar"
        ),
    },
    BenchInput {
        name: "example_2b",
        content: include_str!(
            "../cedar-integration-tests/tests/example_use_cases/policies_2b.cedar"
        ),
    },
    BenchInput {
        name: "example_2c",
        content: include_str!(
            "../cedar-integration-tests/tests/example_use_cases/policies_2c.cedar"
        ),
    },
    BenchInput {
        name: "example_3a",
        content: include_str!(
            "../cedar-integration-tests/tests/example_use_cases/policies_3a.cedar"
        ),
    },
    BenchInput {
        name: "example_3b",
        content: include_str!(
            "../cedar-integration-tests/tests/example_use_cases/policies_3b.cedar"
        ),
    },
    BenchInput {
        name: "example_3c",
        content: include_str!(
            "../cedar-integration-tests/tests/example_use_cases/policies_3c.cedar"
        ),
    },
    BenchInput {
        name: "example_4a",
        content: include_str!(
            "../cedar-integration-tests/tests/example_use_cases/policies_4a.cedar"
        ),
    },
    BenchInput {
        name: "example_4d",
        content: include_str!(
            "../cedar-integration-tests/tests/example_use_cases/policies_4d.cedar"
        ),
    },
    BenchInput {
        name: "example_4e",
        content: include_str!(
            "../cedar-integration-tests/tests/example_use_cases/policies_4e.cedar"
        ),
    },
    BenchInput {
        name: "example_4f",
        content: include_str!(
            "../cedar-integration-tests/tests/example_use_cases/policies_4f.cedar"
        ),
    },
    BenchInput {
        name: "example_5b",
        content: include_str!(
            "../cedar-integration-tests/tests/example_use_cases/policies_5b.cedar"
        ),
    },
    BenchInput {
        name: "ip_1",
        content: include_str!("../cedar-integration-tests/tests/ip/policies_1.cedar"),
    },
    BenchInput {
        name: "ip_2",
        content: include_str!("../cedar-integration-tests/tests/ip/policies_2.cedar"),
    },
    BenchInput {
        name: "ip_3",
        content: include_str!("../cedar-integration-tests/tests/ip/policies_3.cedar"),
    },
    BenchInput {
        name: "multi_1",
        content: include_str!("../cedar-integration-tests/tests/multi/policies_1.cedar"),
    },
    BenchInput {
        name: "multi_2",
        content: include_str!("../cedar-integration-tests/tests/multi/policies_2.cedar"),
    },
    BenchInput {
        name: "multi_3",
        content: include_str!("../cedar-integration-tests/tests/multi/policies_3.cedar"),
    },
    BenchInput {
        name: "multi_4",
        content: include_str!("../cedar-integration-tests/tests/multi/policies_4.cedar"),
    },
    BenchInput {
        name: "multi_5",
        content: include_str!("../cedar-integration-tests/tests/multi/policies_5.cedar"),
    },
    BenchInput {
        name: "corpus_502da",
        content: include_str!(
            "../cedar-integration-tests/corpus-tests/502dae14ed9b788b742c1480b595b9f29f27612e.cedar"
        ),
    },
    BenchInput {
        name: "corpus_c7e64",
        content: include_str!(
            "../cedar-integration-tests/corpus-tests/c7e64d12c0dd1c53c6211747f0aaafaff18637bd.cedar"
        ),
    },
    BenchInput {
        name: "corpus_f4174",
        content: include_str!(
            "../cedar-integration-tests/corpus-tests/f41742f09093931cfdc536f0a9ffcf6e3a541e91.cedar"
        ),
    },
    BenchInput {
        name: "parser_testfile",
        content: include_str!("../cedar/cedar-policy-core/src/parser/testfiles/policies.cedar"),
    },
];

pub const SCHEMAS: &[BenchInput] = &[
    BenchInput {
        name: "sandbox_a",
        content: include_str!(
            "../cedar-integration-tests/sample-data/sandbox_a/schema.cedarschema"
        ),
    },
    BenchInput {
        name: "sandbox_b",
        content: include_str!(
            "../cedar-integration-tests/sample-data/sandbox_b/schema.cedarschema"
        ),
    },
    BenchInput {
        name: "sandbox_b_exts",
        content: include_str!(
            "../cedar-integration-tests/sample-data/sandbox_b/schema_exts.cedarschema"
        ),
    },
    BenchInput {
        name: "corpus_011ec",
        content: include_str!(
            "../cedar-integration-tests/corpus-tests/011ec2ee8d14b9fcd06ac1bc5776eed797d4381b.cedarschema"
        ),
    },
    BenchInput {
        name: "corpus_37250",
        content: include_str!(
            "../cedar-integration-tests/corpus-tests/37250f77c89308275cdb8b6d9cadc674d05f0c8f.cedarschema"
        ),
    },
    BenchInput {
        name: "corpus_bd2fe",
        content: include_str!(
            "../cedar-integration-tests/corpus-tests/bd2fe0bd21edf81d06093d4311a3561c28fc79a6.cedarschema"
        ),
    },
    BenchInput {
        name: "validator_testfile",
        content: include_str!(
            "../cedar/cedar-policy-core/src/validator/cedar_schema/testfiles/example.cedarschema"
        ),
    },
];

#[derive(Copy, Clone)]
pub struct BenchInput {
    pub name: &'static str,
    pub content: &'static str,
}

impl fmt::Debug for BenchInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
