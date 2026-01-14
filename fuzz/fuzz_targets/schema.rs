#![expect(clippy::unwrap_used, clippy::panic, reason = "Fuzzing")]
#![no_main]

use cedar_policy::SchemaFragment as CedarSchema;
use duramen::schema::Schema;
use libfuzzer_sys::fuzz_target;
use similar_asserts::assert_eq;

fuzz_target!(|src: String| {
    let cedar = CedarSchema::from_cedarschema_str(&src);
    let duramen = Schema::parse(&src);

    match (duramen.has_errors(), cedar) {
        (false, Ok((cedar, _warnings))) => {
            assert_eq!(src, duramen.to_string());

            let cedar_json = cedar.to_json_value().unwrap();
            let duramen_json = duramen.to_serde_json_value();
            assert_eq!(cedar_json, duramen_json);
        }
        (false, Err(err)) => {
            panic!("Duramen succeeded but Cedar failed: {err:?}");
        }
        (true, Ok(_)) => {
            panic!("Cedar succeeded but Duramen failed");
        }
        (true, Err(_)) => {
            // Both failed
        }
    }
});
