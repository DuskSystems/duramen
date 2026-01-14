#![expect(clippy::unwrap_used, clippy::panic, reason = "Fuzzing")]
#![no_main]

use core::str::FromStr as _;

use cedar_policy::PolicySet as CedarPolicySet;
use cedar_policy::proto::traits::Protobuf as _;
use duramen::policy::PolicySet;
use libfuzzer_sys::fuzz_target;
use similar_asserts::assert_eq;

fuzz_target!(|src: String| {
    let cedar = CedarPolicySet::from_str(&src);
    let duramen = PolicySet::parse(&src);

    match (duramen.has_errors(), cedar) {
        (false, Ok(cedar)) => {
            assert_eq!(src, duramen.to_string());

            let duramen_bytes = duramen.to_prost_bytes().unwrap();
            let duramen_decoded = CedarPolicySet::decode(duramen_bytes.as_ref()).unwrap();
            let duramen_json = duramen_decoded.to_json().unwrap();

            let cedar_bytes = cedar.encode();
            let cedar_decoded = CedarPolicySet::decode(cedar_bytes.as_slice()).unwrap();
            let cedar_json = cedar_decoded.to_json().unwrap();

            assert_eq!(duramen_json, cedar_json);
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
