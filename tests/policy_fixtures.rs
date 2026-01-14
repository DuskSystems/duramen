use std::path::Path;

use cedar_policy::PolicySet as CedarPolicySet;
use duramen::policy::PolicySet;
use similar_asserts::assert_eq;

datatest_stable::harness! {
    {
        test = roundtrip_policy,
        root = "cedar-integration-tests/tests",
        pattern = r".*[.]cedar$"
    },
    {
        test = roundtrip_policy,
        root = "cedar",
        pattern = r".*[.]cedar$"
    },
    {
        test = roundtrip_policy,
        root = "cedar-integration-tests/corpus-tests",
        pattern = r".*[.]cedar$"
    },
}

fn roundtrip_policy(path: &Path) -> datatest_stable::Result<()> {
    let source = std::fs::read_to_string(path)?;

    let duramen = PolicySet::parse(&source);
    let cedar = source.parse::<CedarPolicySet>();

    match (duramen.has_errors(), cedar) {
        (false, Ok(_)) => {
            assert_eq!(source, duramen.to_string());
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
