#![expect(clippy::panic, clippy::unwrap_used, reason = "Fuzzing")]
#![no_main]

use cedar_policy::PolicySet as CedarPolicySet;
use duramen_ast as ast;
use duramen_est::json::PolicySet;
use duramen_lower::PolicyLowerer;
use duramen_parser::PolicyParser;
use duramen_test::assert_eq;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|source: String| {
    let cedar = source.parse::<CedarPolicySet>();

    let result = PolicyParser::new(&source).parse();
    let duramen = PolicyLowerer::new(&source).lower(result.tree());

    match (cedar, duramen) {
        (Ok(cedar), Ok((templates, _))) => {
            let cedar_json: serde_json::Value = cedar.to_json().unwrap();
            let policy_set = PolicySet::from(templates.as_slice());
            let serialized = serde_json::to_string(&policy_set).unwrap();
            let duramen_json: serde_json::Value = serde_json::from_str(&serialized).unwrap();
            assert_eq!(cedar_json, duramen_json);

            let est_roundtrip: PolicySet = serde_json::from_str(&serialized).unwrap();
            assert_eq!(policy_set, est_roundtrip);

            let ast_roundtrip: Vec<ast::policy::Template> = Vec::from(&est_roundtrip);
            let policy_set_roundtrip = PolicySet::from(ast_roundtrip.as_slice());
            let json_roundtrip: serde_json::Value =
                serde_json::from_str(&serde_json::to_string(&policy_set_roundtrip).unwrap())
                    .unwrap();
            assert_eq!(duramen_json, json_roundtrip);
        }
        (Err(_), Ok(_)) => {
            panic!("Duramen succeeded but Cedar failed");
        }
        (Ok(_), Err(_)) => {
            panic!("Cedar succeeded but Duramen failed");
        }
        (Err(_), Err(_)) => {}
    }
});
