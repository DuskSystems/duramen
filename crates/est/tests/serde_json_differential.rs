use std::path::Path;

use cedar_policy::{PolicySet as CedarPolicySet, SchemaFragment as CedarSchema};
use duramen_ast as ast;
use duramen_est::json::{PolicySet, SchemaFragment};
use duramen_lower::{PolicyLowerer, SchemaLowerer};
use duramen_parser::{PolicyParser, SchemaParser};
use duramen_test::{
    CEDAR_CORPUS, CEDAR_INTEGRATION_TESTS, CEDAR_POLICY_PARSER_TESTS, CEDAR_SAMPLE_DATA,
    CEDAR_SCHEMA_PARSER_TESTS, assert_eq,
};

datatest_stable::harness! {
    {
        test = policy_differential,
        root = CEDAR_CORPUS,
        pattern = r".*[.]cedar$"
    },
    {
        test = policy_differential,
        root = CEDAR_INTEGRATION_TESTS,
        pattern = r".*[.]cedar$"
    },
    {
        test = policy_differential,
        root = CEDAR_POLICY_PARSER_TESTS,
        pattern = r".*[.]cedar$"
    },

    {
        test = schema_differential,
        root = CEDAR_CORPUS,
        pattern = r".*[.]cedarschema$"
    },
    {
        test = schema_differential,
        root = CEDAR_SAMPLE_DATA,
        pattern = r".*[.]cedarschema$"
    },
    {
        test = schema_differential,
        root = CEDAR_SCHEMA_PARSER_TESTS,
        pattern = r".*[.]cedarschema$"
    },
}

fn policy_differential(path: &Path) -> datatest_stable::Result<()> {
    let source = std::fs::read_to_string(path)?;

    let cedar = source.parse::<CedarPolicySet>();

    let result = PolicyParser::new(&source).parse();
    let duramen = PolicyLowerer::new(&source).lower(result.tree());

    match (cedar, duramen) {
        (Ok(cedar), Ok((templates, _))) => {
            let cedar_json: serde_json::Value = cedar.to_json()?;
            let policy_set = PolicySet::from(templates.as_slice());
            let serialized = serde_json::to_string(&policy_set)?;
            let duramen_json: serde_json::Value = serde_json::from_str(&serialized)?;
            assert_eq!(cedar_json, duramen_json);

            let est_roundtrip: PolicySet = serde_json::from_str(&serialized)?;
            assert_eq!(policy_set, est_roundtrip);

            let ast_roundtrip: Vec<ast::policy::Template> = Vec::from(&est_roundtrip);
            let policy_set_roundtrip = PolicySet::from(ast_roundtrip.as_slice());
            let json_roundtrip = serde_json::to_string(&policy_set_roundtrip)?;
            let json_roundtrip: serde_json::Value = serde_json::from_str(&json_roundtrip)?;
            assert_eq!(duramen_json, json_roundtrip);
        }
        (Err(err), Ok(_)) => {
            let path = path.display();
            return Err(format!("Duramen succeeded but Cedar failed for {path}: {err:?}").into());
        }
        (Ok(_), Err(err)) => {
            let path = path.display();
            return Err(format!("Cedar succeeded but Duramen failed for {path}: {err:?}").into());
        }
        (Err(_), Err(_)) => {}
    }

    Ok(())
}

fn schema_differential(path: &Path) -> datatest_stable::Result<()> {
    let source = std::fs::read_to_string(path)?;

    let cedar = CedarSchema::from_cedarschema_str(&source);

    let result = SchemaParser::new(&source).parse();
    let duramen = SchemaLowerer::new(&source).lower(result.tree());

    match (cedar, duramen) {
        (Ok((cedar, _warnings)), Ok((schema, _))) => {
            let cedar_json: serde_json::Value = cedar.to_json_value()?;
            let fragment = SchemaFragment::from(&schema);
            let serialized = serde_json::to_string(&fragment)?;
            let duramen_json: serde_json::Value = serde_json::from_str(&serialized)?;
            assert_eq!(cedar_json, duramen_json);

            let est_roundtrip: SchemaFragment = serde_json::from_str(&serialized)?;
            assert_eq!(fragment, est_roundtrip);

            let ast_roundtrip: ast::schema::Schema = ast::schema::Schema::from(&est_roundtrip);
            let fragment_roundtrip = SchemaFragment::from(&ast_roundtrip);
            let json_roundtrip = serde_json::to_string(&fragment_roundtrip)?;
            let json_roundtrip: serde_json::Value = serde_json::from_str(&json_roundtrip)?;
            assert_eq!(duramen_json, json_roundtrip);
        }
        (Err(err), Ok(_)) => {
            let path = path.display();
            return Err(format!("Duramen succeeded but Cedar failed for {path}: {err:?}").into());
        }
        (Ok(_), Err(err)) => {
            let path = path.display();
            return Err(format!("Cedar succeeded but Duramen failed for {path}: {err:?}").into());
        }
        (Err(_), Err(_)) => {}
    }

    Ok(())
}
