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

    match (duramen, cedar) {
        (Ok(duramen), Ok(cedar)) => {
            let cedar = cedar.encode();
            let duramen = duramen.to_prost_bytes()?;
            assert_eq!(cedar.as_slice(), duramen.as_ref());
        }
        (Ok(_), Err(err)) => {
            let path = path.display();
            let err = format!("Duramen succeeded but Cedar failed for {path}: {err:?}");
            return Err(err.into());
        }
        (Err(err), Ok(_)) => {
            let path = path.display();
            let err = format!("Cedar succeeded but Duramen failed for {path}: {err:?}");
            return Err(err.into());
        }
        (Err(_), Err(_)) => {
            // Both failed
        }
    }

    Ok(())
}
