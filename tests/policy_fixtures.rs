use std::path::Path;

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
        root = "cedar-integration-tests/cedar",
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

    let parsed = PolicySet::parse(&source)?;
    assert_eq!(source, parsed.to_string());

    Ok(())
}
