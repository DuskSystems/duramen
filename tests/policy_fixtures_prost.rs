use std::path::Path;

use cedar_policy::PolicySet as CedarPolicySet;
use cedar_policy::proto::traits::Protobuf as _;
use duramen::policy::PolicySet;
use similar_asserts::assert_eq;

datatest_stable::harness! {
    {
        test = compare_policy,
        root = "cedar-integration-tests/tests",
        pattern = r".*[.]cedar$"
    },
    {
        test = compare_policy,
        root = "cedar-integration-tests/cedar",
        pattern = r".*[.]cedar$"
    },
    {
        test = compare_policy,
        root = "cedar-integration-tests/corpus-tests",
        pattern = r".*[.]cedar$"
    },
}

fn compare_policy(path: &Path) -> datatest_stable::Result<()> {
    let source = std::fs::read_to_string(path)?;

    let duramen = PolicySet::parse(&source);
    let cedar = source.parse::<CedarPolicySet>();

    match (duramen.has_errors(), cedar) {
        (false, Ok(cedar)) => {
            // Can't compare the raw Protobuf bytes here, since ordering of types isn't guaranteed.

            let duramen_bytes = duramen.to_prost_bytes()?;
            let duramen_decoded = CedarPolicySet::decode(duramen_bytes.as_ref())?;
            let duramen_json = duramen_decoded.to_json()?;

            let cedar_bytes = cedar.encode();
            let cedar_decoded = CedarPolicySet::decode(cedar_bytes.as_slice())?;
            let cedar_json = cedar_decoded.to_json()?;

            assert_eq!(duramen_json, cedar_json);
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
