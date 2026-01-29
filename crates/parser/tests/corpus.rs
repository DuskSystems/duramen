use std::path::Path;

use duramen_parser::{PolicyParser, SchemaParser};
use duramen_test::{
    CEDAR_CORPUS, CEDAR_INTEGRATION_TESTS, CEDAR_POLICY_PARSER_TESTS, CEDAR_SAMPLE_DATA,
    CEDAR_SCHEMA_PARSER_TESTS, assert_eq,
};

datatest_stable::harness! {
    {
        test = policy,
        root = CEDAR_CORPUS,
        pattern = r".*[.]cedar$"
    },
    {
        test = policy,
        root = CEDAR_INTEGRATION_TESTS,
        pattern = r".*[.]cedar$"
    },
    {
        test = policy,
        root = CEDAR_POLICY_PARSER_TESTS,
        pattern = r".*[.]cedar$"
    },

    {
        test = schema,
        root = CEDAR_CORPUS,
        pattern = r".*[.]cedarschema$"
    },
    {
        test = schema,
        root = CEDAR_SAMPLE_DATA,
        pattern = r".*[.]cedarschema$"
    },
    {
        test = schema,
        root = CEDAR_SCHEMA_PARSER_TESTS,
        pattern = r".*[.]cedarschema$"
    },
}

fn policy(path: &Path) -> datatest_stable::Result<()> {
    let source = std::fs::read_to_string(path)?;
    let result = PolicyParser::new(&source).parse();

    let printed = result.print(&source);
    assert_eq!(printed, source, "Roundtrip failed");

    Ok(())
}

fn schema(path: &Path) -> datatest_stable::Result<()> {
    let source = std::fs::read_to_string(path)?;
    let result = SchemaParser::new(&source).parse();

    let printed = result.print(&source);
    assert_eq!(printed, source, "Roundtrip failed");

    Ok(())
}
