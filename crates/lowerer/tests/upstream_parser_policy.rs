//! Source: <https://github.com/cedar-policy/cedar/blob/v4.9.0/cedar-policy-core/src/parser/text_to_cst.rs>.

use duramen_cst::{CstNode as _, Policies};
use duramen_diagnostic::Diagnostics;
use duramen_lowerer::PolicyLowerer;
use duramen_parser::PolicyParser;
use duramen_test::insta::assert_snapshot;
use duramen_test::{assert_diagnostics_snapshot, fixtures};

fixtures!(|source| {
    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();
    assert_snapshot!(root);

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_diagnostics_snapshot!(source, &diagnostics);
}:
    variable6, member7,
    ident3_1, ident3_4, ident3_5, ident4_2,
    comments_policy_2, comments_policy_3,
    no_comments_policy4,
    policies2, policies6,
    policy_annotations_bad_id_1, policy_annotations_bad_id_2, policy_annotations_bad_id_3,
    policy_annotations_bad_val_1, policy_annotations_bad_val_2, policy_annotations_bad_val_3,
    policy_annotation_bad_position,
    error_recovery_1, error_recovery_2, error_recovery_3,
    extended_has_20, extended_has_21,
    invalid_token_1, invalid_token_2,
    unclosed_strings_1, unclosed_strings_2, unclosed_strings_3, unclosed_strings_4,
    single_quote_string_1, single_quote_string_2, single_quote_string_3,
    expr_overflow_1, expr_overflow_2,
);
