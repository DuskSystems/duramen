#![expect(clippy::unwrap_used, reason = "Fuzzing")]
#![no_main]

use duramen::policy::est::json::PolicySetJson;
use libfuzzer_sys::fuzz_target;
use similar_asserts::assert_eq;

fuzz_target!(|policies: PolicySetJson| {
    let duramen_json = serde_json::to_value(&policies).unwrap();
    let Ok(cedar) = cedar_policy::PolicySet::from_json_value(duramen_json.clone()) else {
        return;
    };

    let cedar_json = cedar.to_json().unwrap();
    assert_eq!(duramen_json, cedar_json);
});
