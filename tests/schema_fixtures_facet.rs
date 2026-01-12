use std::path::Path;

use cedar_policy::SchemaFragment as CedarSchema;
use duramen::schema::Schema;
use serde_json::Value;
use similar_asserts::assert_eq;

datatest_stable::harness! {
    {
        test = compare_schema,
        root = "cedar-integration-tests/sample-data",
        pattern = r".*[.]cedarschema$"
    },
    {
        test = compare_schema,
        root = "cedar-integration-tests/cedar",
        pattern = r".*[.]cedarschema$"
    },
    {
        test = compare_schema,
        root = "cedar-integration-tests/corpus-tests",
        pattern = r".*[.]cedarschema$"
    },
}

fn compare_schema(path: &Path) -> datatest_stable::Result<()> {
    let source = std::fs::read_to_string(path)?;

    let duramen = Schema::parse(&source);
    let cedar = CedarSchema::from_cedarschema_str(&source);

    match (duramen.has_errors(), cedar) {
        (false, Ok((cedar, _warnings))) => {
            let cedar: Value = cedar.to_json_value()?;

            let duramen = duramen.to_facet_json()?;
            let duramen: Value = serde_json::from_str(&duramen)?;

            assert_eq!(cedar, duramen);
        }
        (false, Err(err)) => {
            let path = path.display();
            let err = format!("Duramen succeeded but Cedar failed for {path}: {err:?}");
            return Err(err.into());
        }
        (true, Ok(_)) => {
            let path = path.display().to_string();
            let rendered: Vec<_> = duramen
                .diagnostics()
                .iter()
                .map(|diagnostic| diagnostic.render(&path, &source))
                .collect();

            let error = rendered.join("\n");
            return Err(error.into());
        }
        (true, Err(_)) => {
            // Both failed
        }
    }

    Ok(())
}
