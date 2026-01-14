#![expect(clippy::unwrap_used, reason = "Fuzzing")]
#![no_main]

use duramen::schema::est::json::SchemaFragmentJson;
use libfuzzer_sys::fuzz_target;
use similar_asserts::assert_eq;

fuzz_target!(|schema: SchemaFragmentJson| {
    let duramen_json = serde_json::to_value(&schema).unwrap();
    let Ok(cedar) = cedar_policy::SchemaFragment::from_json_value(duramen_json.clone()) else {
        return;
    };

    let cedar_json = cedar.to_json_value().unwrap();
    assert_eq!(duramen_json, cedar_json);
});
