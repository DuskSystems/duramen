//! Source: <https://github.com/cedar-policy/cedar/blob/v4.8.2/cedar-policy-core/src/parser/cst_to_ast.rs>.

use duramen_lower::PolicyLowerer;
use duramen_parser::PolicyParser;
use duramen_test::assert_snapshot;

#[test]
fn show_expr1() {
    let source = r#"
        permit(principal, action, resource)
        when { if 7 then 6 > 5 else !5 || "thursday" && ((8) >= "fish") };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn show_expr2() {
    let source = r#"
        permit(principal, action, resource)
        when { [2,3,4].foo["hello"] };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn show_expr3() {
    let source = r#"
        permit(principal, action, resource)
        when { "first".some_ident };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn show_expr4() {
    let source = r"
        permit(principal, action, resource)
        when { 1.some_ident };
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn show_expr5() {
    let source = r#"
        permit(principal, action, resource)
        when { "first"["some string"] };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn show_expr6() {
    let source = r#"
        permit(principal, action, resource)
        when { {"one":1,"two":2} has one };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn show_expr7() {
    let source = r#"
        permit(principal, action, resource)
        when { {"one":1,"two":2}.one };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn show_expr8() {
    let source = r#"
        permit(principal, action, resource)
        when { {"one":1,"two":2}["one"] };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn show_expr9() {
    let source = r#"
        permit(principal, action, resource)
        when { {"this is a valid map key+.-_%()":1,"two":2}["this is a valid map key+.-_%()"] };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn show_expr10() {
    let source = r#"
        permit(principal, action, resource)
        when { {if true then a else b:"b"} };
    "#;

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: `if` is a reserved identifier
      ╭▸ test.cedar:3:17
      │
    3 │         when { {if true then a else b:"b"} };
      ╰╴                ━━━ reserved identifier
    error: missing effect
      ╭▸ test.cedar:3:20
      │
    3 │           when { {if true then a else b:"b"} };
      │ ┏━━━━━━━━━━━━━━━━━━━━┛
    4 │ ┃     
      ╰╴┗━━━━┛ expected `permit` or `forbid`
    "#);
}

#[test]
fn show_expr11() {
    let source = r#"
        permit(principal, action, resource)
        when { {principal:"principal"} };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn show_expr12() {
    let source = r#"
        permit(principal, action, resource)
        when { {"principal":"principal"} };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn reserved_idents1() {
    let source = r#"
        permit(principal, action, resource)
        when { The::true::path::"enlightenment".false };
    "#;

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: `true` is a reserved identifier
      ╭▸ test.cedar:3:21
      │
    3 │         when { The::true::path::"enlightenment".false };
      ╰╴                    ━━━━ reserved identifier
    "#);
}

#[test]
fn reserved_idents2() {
    let source = r"
        permit(principal, action, resource)
        when { {if: true}.if };
    ";

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r"
    error: `if` is a reserved identifier
      ╭▸ test.cedar:3:17
      │
    3 │         when { {if: true}.if };
      ╰╴                ━━━━━━━━ reserved identifier
    error: `if` is a reserved identifier
      ╭▸ test.cedar:3:26
      │
    3 │         when { {if: true}.if };
      ╰╴                         ━━━━ reserved identifier
    ");
}

#[test]
fn show_policy2() {
    let source = r"
        permit(principal, action, resource) when { true };
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn show_policy3() {
    let source = r#"
        permit(principal in User::"jane", action, resource);
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn show_policy4() {
    let source = r#"
        forbid(principal in User::"jane", action, resource) unless {
            context.group != "friends"
        };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn single_annotation() {
    let source = r#"
        @anno("good annotation")
        permit(principal, action, resource);
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn duplicate_annotations_error() {
    let source = r#"
        @anno("good annotation")
        @anno2("good annotation")
        @anno("oops, duplicate")
        permit(principal, action, resource);
    "#;

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: duplicate annotation `@anno`
      ╭▸ test.cedar:4:9
      │
    2 │ ┌         @anno("good annotation")
    3 │ │         @anno2("good annotation")
      │ └────────┘ first defined here
    4 │ ┏         @anno("oops, duplicate")
    5 │ ┃         permit(principal, action, resource);
      ╰╴┗━━━━━━━━┛ duplicate annotation
    "#);
}

#[test]
fn reserved_word_annotations_ok() {
    let source = r#"
        @if("annotation for if")
        @then("annotation for then")
        @else("annotation for else")
        @true("annotation for true")
        @false("annotation for false")
        permit(principal, action, resource);
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn single_annotation_without_value() {
    let source = r"
        @anno
        permit(principal, action, resource);
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn multiple_annotation_without_value() {
    let source = r"
        @foo @bar
        permit(principal, action, resource);
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn fail_scope1() {
    let source = r#"
        permit(
            principal in [User::"jane", Group::"friends"],
            action,
            resource
        );
    "#;

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: expected entity reference or slot
      ╭▸ test.cedar:3:26
      │
    3 │             principal in [User::"jane", Group::"friends"],
      ╰╴                         ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ expected entity like `Type::"id"` or slot like `?principal`
    "#);
}

#[test]
fn fail_scope2() {
    let source = r#"
        permit(
            principal in User::"jane",
            action == if true then Photo::"view" else Photo::"edit",
            resource
        );
    "#;

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: expected entity reference
      ╭▸ test.cedar:4:23
      │
    4 │             action == if true then Photo::"view" else Photo::"edit",
      ╰╴                      ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ expected entity like `Type::"id"`
    "#);
}

#[test]
fn method_call2() {
    let source = r"
        permit(principal, action, resource)
        when { principal.contains(resource) };
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn construct_record_1() {
    let source = r#"
        permit(principal, action, resource)
        when { {one:"one"} };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn construct_record_2() {
    let source = r#"
        permit(principal, action, resource)
        when { {"one":"one"} };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn construct_record_3() {
    let source = r#"
        permit(principal, action, resource)
        when { {"one":"one", two:"two"} };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn construct_record_4() {
    let source = r#"
        permit(principal, action, resource)
        when { {one:"one", "two":"two"} };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn construct_record_5() {
    let source = r#"
        permit(principal, action, resource)
        when { {one:"b\"", "b\"":2} };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn duplicate_keys() {
    let source = r#"
        permit(principal, action, resource)
        when { {"one": 1, "one": 2} };
    "#;

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: duplicate key `one`
      ╭▸ test.cedar:3:27
      │
    3 │         when { {"one": 1, "one": 2} };
      │                 ┬───────  ━━━━━━━━ duplicate key
      │                 │
      ╰╴                first defined here
    "#);
}

#[test]
fn construct_has_1() {
    let source = r#"
        permit(principal, action, resource)
        when { {"one":1,"two":2} has "arbitrary+ _string" };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn construct_has_2() {
    let source = r#"
        permit(principal, action, resource)
        when { {"one":1,"two":2} has 1 };
    "#;

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: missing effect
      ╭▸ test.cedar:3:38
      │
    3 │           when { {"one":1,"two":2} has 1 };
      │ ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
    4 │ ┃     
      ╰╴┗━━━━┛ expected `permit` or `forbid`
    "#);
}

#[test]
fn construct_like_1() {
    let source = r#"
        permit(principal, action, resource)
        when { "354 hams" like "*5*" };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn construct_like_2() {
    let source = r#"
        permit(principal, action, resource)
        when { "354 hams" like 354 };
    "#;

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: missing effect
      ╭▸ test.cedar:3:32
      │
    3 │           when { "354 hams" like 354 };
      │ ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
    4 │ ┃     
      ╰╴┗━━━━┛ expected `permit` or `forbid`
    "#);
}

#[test]
fn construct_like_3() {
    let source = r#"
        permit(principal, action, resource)
        when { "string\\with\\backslashes" like "string\\with\\backslashes" };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn construct_like_6() {
    let source = r#"
        permit(principal, action, resource)
        when { "string*with*stars" like "string\*with\*stars" };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn construct_like_var() {
    let source = r#"
        permit(principal, action, resource)
        when { "principal" like principal };
    "#;

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: missing effect
      ╭▸ test.cedar:3:33
      │
    3 │           when { "principal" like principal };
      │ ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
    4 │ ┃     
      ╰╴┗━━━━┛ expected `permit` or `forbid`
    "#);
}

#[test]
fn entity_access() {
    let source = r#"
        permit(principal, action, resource)
        when { User::"jane" has age };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());

    let source = r#"
        permit(principal, action, resource)
        when { User::"jane" has "arbitrary+ _string" };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());

    let source = r#"
        permit(principal, action, resource)
        when { User::"jane".age };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());

    let source = r#"
        permit(principal, action, resource)
        when { User::"jane"["arbitrary+ _string"] };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn relational_ops1() {
    let source = r"
        permit(principal, action, resource)
        when { 3 >= 2 >= 1 };
    ";

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"
    error: missing effect
      ╭▸ test.cedar:3:23
      │
    3 │           when { 3 >= 2 >= 1 };
      │ ┏━━━━━━━━━━━━━━━━━━━━━━━┛
    4 │ ┃     
      ╰╴┗━━━━┛ expected `permit` or `forbid`
    ");
}

#[test]
fn relational_ops2() {
    let source = r#"
        permit(principal, action, resource)
        when { 3 >= ("dad" in "dad") };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn relational_ops3() {
    let source = r"
        permit(principal, action, resource)
        when { (3 >= 2) == true };
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn arithmetic() {
    let source = r"
        permit(principal, action, resource)
        when { 2 + 4 == 6 };
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());

    let source = r"
        permit(principal, action, resource)
        when { 2 + -5 == -3 };
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());

    let source = r"
        permit(principal, action, resource)
        when { 5 + 10 - 90 * -2 == 195 };
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());

    let source = r"
        permit(principal, action, resource)
        when { context.size * 4 > 100 };
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn template_tests() {
    let source = r#"
        permit(principal == ?principal, action == Action::"action", resource == ?resource);
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());

    let source = r#"
        permit(principal in ?principal, action == Action::"action", resource in ?resource);
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());

    let source = r"
        permit(principal is User in ?principal, action, resource);
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn var_type() {
    let source = r"
        permit(principal, action, resource);
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn unescape_err_positions() {
    let source = r#"
        @foo("\q")
        permit(principal, action, resource);
    "#;

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: invalid escape sequence: invalid escape
      ╭▸ test.cedar:2:9
      │
    2 │ ┏         @foo("\q")
    3 │ ┃         permit(principal, action, resource);
      ╰╴┗━━━━━━━━┛ invalid escape
    "#);

    let source = r#"
        permit(principal, action, resource)
        when { "\q" };
    "#;

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: invalid escape sequence: invalid escape
      ╭▸ test.cedar:3:16
      │
    3 │         when { "\q" };
      ╰╴               ━━━━━ invalid escape
    "#);

    let source = r#"
        permit(principal, action, resource)
        when { "" like "\q" };
    "#;

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: invalid pattern: invalid escape
      ╭▸ test.cedar:3:16
      │
    3 │         when { "" like "\q" };
      ╰╴               ━━━━━━━━━━━━━ invalid pattern
    "#);

    let source = r#"
        permit(principal, action, resource)
        when { User::"\q" };
    "#;

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: invalid escape sequence: invalid escape
      ╭▸ test.cedar:3:16
      │
    3 │         when { User::"\q" };
      ╰╴               ━━━━━━━━━━━ invalid escape
    "#);
}

#[test]
fn method_style() {
    let source = r"
        permit(principal, action, resource)
        when { contains(principal, resource) };
    ";

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"
    error: `contains` is not a valid identifier
      ╭▸ test.cedar:3:16
      │
    3 │         when { contains(principal, resource) };
      ╰╴               ━━━━━━━━ invalid identifier
    ");
}

#[test]
fn neg() {
    let source = r"
        permit(principal, action, resource)
        when { --1 == -(-1) };
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());

    let source = r"
        permit(principal, action, resource)
        when { -9223372036854775808 == -9223372036854775808 };
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());

    let source = r"
        permit(principal, action, resource)
        when { --9223372036854775808 };
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn over_unary() {
    let source = r"
        permit(principal, action, resource)
        when { -9223372036854775809 };
    ";

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"
    error: integer overflow: `9223372036854775809`
      ╭▸ test.cedar:3:17
      │
    3 │         when { -9223372036854775809 };
      ╰╴                ━━━━━━━━━━━━━━━━━━━━ value out of range for i64
    ");

    let source = r"
        permit(principal, action, resource)
        when { -----1 };
    ";

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"
    error: too many `-` operators
      ╭▸ test.cedar:3:16
      │
    3 │         when { -----1 };
      ╰╴               ━━━━━━━ too many consecutive operators
    ");

    let source = r"
        permit(principal, action, resource)
        when { !!!!!true };
    ";

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"
    error: too many `!` operators
      ╭▸ test.cedar:3:16
      │
    3 │         when { !!!!!true };
      ╰╴               ━━━━━━━━━━ too many consecutive operators
    ");
}

#[test]
fn is_condition_ok() {
    let source = r#"
        permit(principal, action, resource)
        when { User::"alice" is User };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());

    let source = r"
        permit(principal, action, resource)
        when { principal is User };
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());

    let source = r#"
        permit(principal, action, resource)
        when { principal is User in Group::"friends" };
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn is_scope() {
    let source = r"
        permit(principal is User, action, resource);
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());

    let source = r#"
        permit(principal is User in Group::"thing", action, resource);
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());

    let source = r"
        permit(principal, action, resource is Folder);
    ";

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());

    let source = r#"
        permit(principal, action, resource is Folder in Folder::"inner");
    "#;

    let tree = PolicyParser::new(source).parse();
    let result = PolicyLowerer::new(source).lower(tree.tree());
    assert!(result.is_ok());
}

#[test]
fn invalid_slot() {
    let source = r"
        permit(principal == ?resource, action, resource);
    ";

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: invalid slot `?resource`
      ╭▸ test.cedar:2:29
      │
    2 │         permit(principal == ?resource, action, resource);
      ╰╴                            ━━━━━━━━━ expected `?principal` or `?resource`
    "#);

    let source = r"
        permit(principal, action, resource in ?principal);
    ";

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r#"
    error: invalid slot `?principal`
      ╭▸ test.cedar:2:47
      │
    2 │         permit(principal, action, resource in ?principal);
      ╰╴                                              ━━━━━━━━━━ expected `?principal` or `?resource`
    "#);
}

#[test]
fn arbitrary_variables() {
    let source = r"
        permit(principal, action, resource)
        when { foo };
    ";

    let tree = PolicyParser::new(source).parse();
    let diagnostics = PolicyLowerer::new(source).lower(tree.tree()).unwrap_err();
    assert!(!diagnostics.is_empty());

    let rendered = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render("test.cedar", source))
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @"
    error: `foo` is not a valid identifier
      ╭▸ test.cedar:3:16
      │
    3 │         when { foo };
      ╰╴               ━━━━ invalid identifier
    ");
}
