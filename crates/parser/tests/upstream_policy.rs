//! Source: <https://github.com/cedar-policy/cedar/blob/v4.9.0/cedar-policy-core/src/parser/text_to_cst.rs>.

use duramen_diagnostic::Diagnostics;
use duramen_parser::PolicyParser;
use duramen_test::fixtures;

fn parse(source: &str) {
    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0, "expected no parser diagnostics");
}

fixtures!(parse:
    expr1, expr2, expr3, expr4, expr5, expr6,
    expr_overflow_1, expr_overflow_2,
    variable1, variable2, variable3, variable4, variable6,
    member1, member2, member3, member4, member5, member6, member7, member8, member9,
    ident3_1, ident3_2, ident3_4, ident3_5,
    ident4_1, ident4_2,
    ident5_1, ident5_2,
    ident6_1, ident6_2,
    comments_has, comments_like, comments_and, comments_or, comments_add,
    comments_paren, comments_set, comments_if, comments_member_access,
    comments_principal, comments_annotation,
    comments_policy_1, comments_policy_2, comments_policy_3,
    no_comments_policy, no_comments_policy2, no_comments_policy4, no_comments_policy5,
    policies1, policies2, policies3, policies3p, policies4, policies5, policies6,
    policy_annotations_ok, policy_annotations_no_value_ok,
    policy_annotations_bad_id_1, policy_annotations_bad_id_2, policy_annotations_bad_id_3,
    policy_annotations_bad_val_1, policy_annotations_bad_val_2, policy_annotations_bad_val_3,
    policy_annotation_bad_position,
    error_recovery_1, error_recovery_2, error_recovery_3,
    extended_has_1, extended_has_2, extended_has_3, extended_has_4, extended_has_5,
    extended_has_6, extended_has_7, extended_has_8, extended_has_9, extended_has_10,
    extended_has_11, extended_has_12, extended_has_13, extended_has_14, extended_has_15,
    extended_has_16, extended_has_17, extended_has_18, extended_has_19,
    extended_has_20, extended_has_21,
    trailing_comma_1, trailing_comma_2, trailing_comma_3, trailing_comma_4, trailing_comma_5,
    invalid_token_1, invalid_token_2,
    unclosed_strings_1, unclosed_strings_2, unclosed_strings_3, unclosed_strings_4,
    single_quote_string_1, single_quote_string_2, single_quote_string_3,
);
