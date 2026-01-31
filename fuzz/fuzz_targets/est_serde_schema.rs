#![expect(clippy::panic, clippy::unwrap_used, reason = "Fuzzing")]
#![no_main]

use cedar_policy::SchemaFragment as CedarSchema;
use duramen_ast as ast;
use duramen_est::json::SchemaFragment;
use duramen_lower::SchemaLowerer;
use duramen_parser::SchemaParser;
use duramen_test::assert_eq;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|source: String| {
    let cedar = CedarSchema::from_cedarschema_str(&source);

    let result = SchemaParser::new(&source).parse();
    let duramen = SchemaLowerer::new(&source).lower(result.tree());

    match (cedar, duramen) {
        (Ok((cedar, _warnings)), Ok((schema, _))) => {
            let cedar_json: serde_json::Value = cedar.to_json_value().unwrap();
            let fragment = SchemaFragment::from(&schema);
            let serialized = serde_json::to_string(&fragment).unwrap();
            let duramen_json: serde_json::Value = serde_json::from_str(&serialized).unwrap();
            assert_eq!(cedar_json, duramen_json);

            let est_roundtrip: SchemaFragment = serde_json::from_str(&serialized).unwrap();
            assert_eq!(fragment, est_roundtrip);

            let ast_roundtrip: ast::schema::Schema = ast::schema::Schema::from(&est_roundtrip);
            let fragment_roundtrip = SchemaFragment::from(&ast_roundtrip);
            let json_roundtrip: serde_json::Value =
                serde_json::from_str(&serde_json::to_string(&fragment_roundtrip).unwrap()).unwrap();
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
