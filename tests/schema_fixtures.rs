use std::path::Path;

use duramen::schema::Schema;
use similar_asserts::assert_eq;

datatest_stable::harness! {
    {
        test = roundtrip_schema,
        root = "cedar-integration-tests/sample-data",
        pattern = r".*[.]cedarschema$"
    },
    {
        test = roundtrip_schema,
        root = "cedar",
        pattern = r".*[.]cedarschema$"
    },
    {
        test = roundtrip_schema,
        root = "cedar-integration-tests/corpus-tests",
        pattern = r".*[.]cedarschema$"
    },
}

fn roundtrip_schema(path: &Path) -> datatest_stable::Result<()> {
    let source = std::fs::read_to_string(path)?;
    let parsed = Schema::parse(&source);

    if parsed.has_errors() {
        let path = path.display().to_string();
        let rendered: Vec<_> = parsed
            .diagnostics()
            .iter()
            .map(|diagnostic| diagnostic.render(&path, &source))
            .collect();

        let error = rendered.join("\n");
        return Err(error.into());
    }

    assert_eq!(source, parsed.to_string());
    Ok(())
}
