//! Source: <https://github.com/cedar-policy/cedar/blob/v4.9.0/cedar-policy-core/src/parser/text_to_cst.rs>.

use duramen_cst::{CstNode as _, Policies};
use duramen_diagnostic::Diagnostics;
use duramen_lower::PolicyLowerer;
use duramen_parser::PolicyParser;
use duramen_test::{assert_diagnostics_snapshot, source};

#[test]
fn variable6() {
    let source = source! {r"
        permit (var : in 6, action, resource);
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 2);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `)`
      ╭▸ test:1:9
      │
    1 │ permit (var : in 6, action, resource);
      ╰╴        ━━━ expected `)`
    error: missing policy effect
      ╭▸ test:1:18
      │
    1 │ permit (var : in 6, action, resource);
      ╰╴                 ━━━━━━━━━━━━━━━━━━━━━ expected `permit` or `forbid`
    ");
}

#[test]
fn member7() {
    let source = source! {r#"
        permit (principal, action, resource)
        when {
            one{num: true, trivia: "first!"}
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 3);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: expected `}`
      ╭▸ test:3:5
      │
    3 │     one{num: true, trivia: "first!"}
      ╰╴    ━━━ expected `}`
    error: unknown variable `one`
      ╭▸ test:3:5
      │
    3 │     one{num: true, trivia: "first!"}
      │     ━━━ not a valid variable
      ╰╴
    note: `principal`, `action`, `resource`, and `context` are the only variables
    error: missing policy effect
      ╭▸ test:3:8
      │
    3 │       one{num: true, trivia: "first!"}
      │ ┏━━━━━━━━┛
    4 │ ┃ };
      ╰╴┗━━━┛ expected `permit` or `forbid`
    "#);
}

#[test]
fn ident3_1() {
    let source = source! {r"
        permit (principal, action, resource)
        when { if };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `}`
      ╭▸ test:2:8
      │
    2 │ when { if };
      ╰╴       ━━ expected `}`
    ");
}

#[test]
fn ident3_4() {
    let source = source! {r"
        permit (principal, action, resource)
        when { if::then::else };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `}`
      ╭▸ test:2:8
      │
    2 │ when { if::then::else };
      ╰╴       ━━ expected `}`
    ");
}

#[test]
fn ident3_5() {
    let source = source! {r"
        permit (principal, action, resource)
        when { if::true::then::false::else::true };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 2);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `}`
      ╭▸ test:2:8
      │
    2 │ when { if::true::then::false::else::true };
      ╰╴       ━━ expected `}`
    error: missing policy effect
      ╭▸ test:2:12
      │
    2 │ when { if::true::then::false::else::true };
      ╰╴           ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ expected `permit` or `forbid`
    ");
}

#[test]
fn comments_policy_3() {
    let source = source! {r"
        permit (principal, action, resource)
        when { 1 /* multi-line
        comment */d };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 3);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `}`
      ╭▸ test:2:8
      │
    2 │ when { 1 /* multi-line
      ╰╴       ━ expected `}`
    error: division and remainder are not supported
      ╭▸ test:2:8
      │
    2 │ when { 1 /* multi-line
      │        ━━━━━ not supported
      ╰╴
    note: only `*` with an integer literal is allowed
    error: missing policy effect
      ╭▸ test:2:13
      │
    2 │   when { 1 /* multi-line
      │ ┏━━━━━━━━━━━━━┛
    3 │ ┃ comment */d };
      ╰╴┗━━━━━━━━━━━━━━━┛ expected `permit` or `forbid`
    ");
}

#[test]
fn no_comments_policy4() {
    let source = source! {r#"
        permit (principal, action, resource, context)
        when {
            context.contains(3, "four", five(6, 7))
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 2);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: expected `)`
      ╭▸ test:1:9
      │
    1 │ permit (principal, action, resource, context)
      ╰╴        ━━━━━━━━━ expected `)`
    error: missing policy effect
      ╭▸ test:1:38
      │
    1 │   permit (principal, action, resource, context)
      │ ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
    2 │ ┃ when {
    3 │ ┃     context.contains(3, "four", five(6, 7))
    4 │ ┃ };
      ╰╴┗━━━┛ expected `permit` or `forbid`
    "#);
}

#[test]
fn policies2() {
    let source = source! {r#"
        permit (
            principal in Group::"jane_friends",  // Policy c1
            action in [PhotoOp::"view", PhotoOp::"comment"],
            resource in Album::"jane_trips",
            context:Group
        );
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 2);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: expected `)`
      ╭▸ test:2:5
      │
    2 │     principal in Group::"jane_friends",  // Policy c1
      ╰╴    ━━━━━━━━━ expected `)`
    error: missing policy effect
      ╭▸ test:5:5
      │
    5 │ ┏     context:Group
    6 │ ┃ );
      ╰╴┗━━━┛ expected `permit` or `forbid`
    "#);
}

#[test]
fn policy_annotations_bad_val_1() {
    let source = source! {r#"
        @bad_annotation("bad", "annotation")
        permit (principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 3);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: expected `)`
      ╭▸ test:1:17
      │
    1 │ @bad_annotation("bad", "annotation")
      ╰╴                ━━━━━ expected `)`
    error: missing policy effect
      ╭▸ test:1:1
      │
    1 │ @bad_annotation("bad", "annotation")
      ╰╴━━━━━━━━━━━━━━━━━━━━━ expected `permit` or `forbid`
    error: unexpected token
      ╭▸ test:1:22
      │
    1 │   @bad_annotation("bad", "annotation")
      │ ┏━━━━━━━━━━━━━━━━━━━━━━┛
    2 │ ┃ permit (principal, action, resource);
      ╰╴┗━┛ unexpected token
    "#);
}

#[test]
fn policy_annotations_bad_val_3() {
    let source = source! {r"
        @bad_annotation(bad_annotation)
        permit (principal, action, resource);
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 3);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `)`
      ╭▸ test:1:17
      │
    1 │ @bad_annotation(bad_annotation)
      ╰╴                ━ expected `)`
    error: missing policy effect
      ╭▸ test:1:1
      │
    1 │ @bad_annotation(bad_annotation)
      ╰╴━━━━━━━━━━━━━━━━ expected `permit` or `forbid`
    error: unexpected token
      ╭▸ test:1:17
      │
    1 │   @bad_annotation(bad_annotation)
      │ ┏━━━━━━━━━━━━━━━━━┛
    2 │ ┃ permit (principal, action, resource);
      ╰╴┗━┛ unexpected token
    ");
}

#[test]
fn policy_annotation_bad_position() {
    let source = source! {r#"
        permit (@comment("your name here") principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 3);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: expected `)`
      ╭▸ test:1:9
      │
    1 │ permit (@comment("your name here") principal, action, resource);
      ╰╴        ━ expected `)`
    error: missing policy effect
      ╭▸ test:1:9
      │
    1 │ permit (@comment("your name here") principal, action, resource);
      ╰╴        ━━━━━━━━━━━━━━━━━━━━━━━━━━━ expected `permit` or `forbid`
    error: missing policy effect
      ╭▸ test:1:36
      │
    1 │ permit (@comment("your name here") principal, action, resource);
      ╰╴                                   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ expected `permit` or `forbid`
    "#);
}

#[test]
fn error_recovery_1() {
    let source = source! {r"
        permit (principal, action, !)
        when { principal.foo == resource.bar};

        permit (principal, action, resource);
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 2);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `)`
      ╭▸ test:1:9
      │
    1 │ permit (principal, action, !)
      ╰╴        ━━━━━━━━━ expected `)`
    error: unexpected token
      ╭▸ test:1:28
      │
    1 │   permit (principal, action, !)
      │ ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
    2 │ ┃ when { principal.foo == resource.bar};
    3 │ ┃
    4 │ ┃ permit (principal, action, resource);
      ╰╴┗━┛ unexpected token
    ");
}

#[test]
fn error_recovery_2() {
    let source = source! {r"
        permit (principal, action, !)
        when { principal.foo == resource.bar};

        permit (principal, action, +);
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 4);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `)`
      ╭▸ test:1:9
      │
    1 │ permit (principal, action, !)
      ╰╴        ━━━━━━━━━ expected `)`
    error: unexpected token
      ╭▸ test:1:28
      │
    1 │   permit (principal, action, !)
      │ ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
    2 │ ┃ when { principal.foo == resource.bar};
    3 │ ┃
    4 │ ┃ permit (principal, action, +);
      ╰╴┗━┛ unexpected token
    error: expected `)`
      ╭▸ test:4:9
      │
    4 │ permit (principal, action, +);
      ╰╴        ━━━━━━━━━ expected `)`
    error: missing policy effect
      ╭▸ test:4:28
      │
    4 │ permit (principal, action, +);
      ╰╴                           ━━━ expected `permit` or `forbid`
    ");
}

#[test]
fn error_recovery_3() {
    let source = source! {r"
        permit (principal, action, !)
        when { principal.foo == resource.bar}
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 2);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `)`
      ╭▸ test:1:9
      │
    1 │ permit (principal, action, !)
      ╰╴        ━━━━━━━━━ expected `)`
    error: missing policy effect
      ╭▸ test:1:28
      │
    1 │   permit (principal, action, !)
      │ ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
    2 │ ┃ when { principal.foo == resource.bar}
      ╰╴┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ expected `permit` or `forbid`
    ");
}

#[test]
fn extended_has_21() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has a.1
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 3);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `}`
      ╭▸ test:3:5
      │
    3 │     principal has a.1
      ╰╴    ━━━━━━━━━ expected `}`
    error: missing expression
      ╭▸ test:3:19
      │
    3 │     principal has a.1
      ╰╴                  ━━ expected an attribute name
    error: missing policy effect
      ╭▸ test:3:21
      │
    3 │       principal has a.1
      │ ┏━━━━━━━━━━━━━━━━━━━━━┛
    4 │ ┃ };
      ╰╴┗━━━┛ expected `permit` or `forbid`
    ");
}

#[test]
fn invalid_token_1() {
    let source = source! {r"
        permit (principal, action, resource)
        when { ~ };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: unexpected token
      ╭▸ test:2:8
      │
    2 │ when { ~ };
      ╰╴       ━━ unexpected token
    ");
}

#[test]
fn invalid_token_2() {
    let source = source! {"
        permit (principal, action, resource)
        when { \u{1F680} };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: unexpected token
      ╭▸ test:2:8
      │
    2 │ when { 🚀 };
      ╰╴       ━━━ unexpected token
    ");
}

#[test]
fn unclosed_strings_1() {
    let source = source! {r#"
        permit (principal, action, resource)
        when {
            principal.foo = "bar
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 2);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: expected `}`
      ╭▸ test:3:5
      │
    3 │     principal.foo = "bar
      ╰╴    ━━━━━━━━━ expected `}`
    error: invalid operator `=`
      ╭▸ test:3:19
      │
    3 │     principal.foo = "bar
      │                   ━ not a valid operator
      ╰╴
    help: use `==` for equality
      ╭╴
    3 │     principal.foo == "bar
      ╰╴                   +
    "#);
}

#[test]
fn unclosed_strings_2() {
    let source = source! {r#"
        permit (principal, action, resource == Photo::"mine.jpg);
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 3);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: expected `)`
      ╭▸ test:1:9
      │
    1 │ permit (principal, action, resource == Photo::"mine.jpg);
      ╰╴        ━━━━━━━━━ expected `)`
    error: missing expression
      ╭▸ test:1:40
      │
    1 │ permit (principal, action, resource == Photo::"mine.jpg);
      ╰╴                                       ━━━━━━━ expected an entity reference or slot
    error: missing policy effect
      ╭▸ test:1:47
      │
    1 │ permit (principal, action, resource == Photo::"mine.jpg);
      ╰╴                                              ━━━━━━━━━━━ expected `permit` or `forbid`
    "#);
}

#[test]
fn unclosed_strings_3() {
    let source = source! {r#"
        @id("0)permit (principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 3);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: expected `)`
      ╭▸ test:1:5
      │
    1 │ @id("0)permit (principal, action, resource);
      ╰╴    ━ expected `)`
    error: missing policy effect
      ╭▸ test:1:1
      │
    1 │ @id("0)permit (principal, action, resource);
      ╰╴━━━━ expected `permit` or `forbid`
    error: missing policy effect
      ╭▸ test:1:5
      │
    1 │ @id("0)permit (principal, action, resource);
      ╰╴    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ expected `permit` or `forbid`
    "#);
}

#[test]
fn unclosed_strings_4() {
    let source = source! {r#"
        @id("0)
        permit (principal, action, resource)
        when {
            principal.foo = "bar"
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 3);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: expected `)`
      ╭▸ test:1:5
      │
    1 │   @id("0)
      │ ┏━━━━━┛
    2 │ ┃ permit (principal, action, resource)
    3 │ ┃ when {
    4 │ ┃     principal.foo = "bar"
      ╰╴┗━━━━━━━━━━━━━━━━━━━━━┛ expected `)`
    error: missing policy effect
      ╭▸ test:1:1
      │
    1 │ ┏ @id("0)
    2 │ ┃ permit (principal, action, resource)
    3 │ ┃ when {
    4 │ ┃     principal.foo = "bar"
      ╰╴┗━━━━━━━━━━━━━━━━━━━━━┛ expected `permit` or `forbid`
    error: missing policy effect
      ╭▸ test:4:22
      │
    4 │       principal.foo = "bar"
      │ ┏━━━━━━━━━━━━━━━━━━━━━━┛
    5 │ ┃ };
      ╰╴┗━━━┛ expected `permit` or `forbid`
    "#);
}

#[test]
fn single_quote_string_1() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal.foo = 'bar'
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 3);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `}`
      ╭▸ test:3:5
      │
    3 │     principal.foo = 'bar'
      ╰╴    ━━━━━━━━━ expected `}`
    error: invalid operator `=`
      ╭▸ test:3:19
      │
    3 │     principal.foo = 'bar'
      │                   ━ not a valid operator
      ╰╴
    help: use `==` for equality
      ╭╴
    3 │     principal.foo == 'bar'
      ╰╴                   +
    error: missing policy effect
      ╭▸ test:3:22
      │
    3 │       principal.foo = 'bar'
      │ ┏━━━━━━━━━━━━━━━━━━━━━━┛
    4 │ ┃ };
      ╰╴┗━━━┛ expected `permit` or `forbid`
    ");
}

#[test]
fn single_quote_string_2() {
    let source = source! {r"
        permit (principal, action, resource == Photo::'mine.jpg');
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 3);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `)`
      ╭▸ test:1:9
      │
    1 │ permit (principal, action, resource == Photo::'mine.jpg');
      ╰╴        ━━━━━━━━━ expected `)`
    error: missing expression
      ╭▸ test:1:40
      │
    1 │ permit (principal, action, resource == Photo::'mine.jpg');
      ╰╴                                       ━━━━━━━ expected an entity reference or slot
    error: missing policy effect
      ╭▸ test:1:47
      │
    1 │ permit (principal, action, resource == Photo::'mine.jpg');
      ╰╴                                              ━━━━━━━━━━━━ expected `permit` or `forbid`
    ");
}

#[test]
fn single_quote_string_3() {
    let source = source! {r"
        @id('0')permit (principal, action, resource);
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 3);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `)`
      ╭▸ test:1:5
      │
    1 │ @id('0')permit (principal, action, resource);
      ╰╴    ━ expected `)`
    error: missing policy effect
      ╭▸ test:1:1
      │
    1 │ @id('0')permit (principal, action, resource);
      ╰╴━━━━ expected `permit` or `forbid`
    error: unexpected token
      ╭▸ test:1:5
      │
    1 │ @id('0')permit (principal, action, resource);
      ╰╴    ━━━━ unexpected token
    ");
}

#[test]
fn expr_overflow_1() {
    let source = source! {r"
        permit (principal, action, resource)
        when { principal == -5555555555555555555555 };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: integer literal `-5555555555555555555555` is out of range
      ╭▸ test
      │
    ");
}

#[test]
fn expr_overflow_2() {
    let source = source! {r"
        permit (principal, action, resource)
        when { principal == 5555555555555555555555 };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: integer literal `5555555555555555555555` is out of range
      ╭▸ test
      │
    ");
}

#[test]
fn ident4_2() {
    let source = source! {r"
        permit (principal, action, resource)
        when { if(true) };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `then`
      ╭▸ test:2:17
      │
    2 │ when { if(true) };
      ╰╴                ━ expected `then`
    ");
}

#[test]
fn comments_policy_2() {
    let source = source! {r"
        /* multi-line
        comment */
        permit (principal, action, resource)
        when {
            one.two
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 2);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: unexpected token
      ╭▸ test:1:1
      │
    1 │ ┏ /* multi-line
    2 │ ┃ comment */
    3 │ ┃ permit (principal, action, resource)
      ╰╴┗━┛ unexpected token
    error: unknown variable `one`
      ╭▸ test:5:5
      │
    5 │     one.two
      │     ━━━ not a valid variable
      ╰╴
    note: `principal`, `action`, `resource`, and `context` are the only variables
    ");
}

#[test]
fn policy_annotations_bad_id_1() {
    let source = source! {r#"
        @bad-annotation("bad")
        permit (principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 2);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: missing policy effect
      ╭▸ test:1:1
      │
    1 │ @bad-annotation("bad")
      ╰╴━━━━ expected `permit` or `forbid`
    error: unexpected token
      ╭▸ test:1:5
      │
    1 │   @bad-annotation("bad")
      │ ┏━━━━━┛
    2 │ ┃ permit (principal, action, resource);
      ╰╴┗━┛ unexpected token
    "#);
}

#[test]
fn policy_annotations_bad_id_2() {
    let source = source! {r#"
        @hi mom("this should be invalid")
        permit (principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 2);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: missing policy effect
      ╭▸ test:1:1
      │
    1 │ @hi mom("this should be invalid")
      ╰╴━━━━ expected `permit` or `forbid`
    error: unexpected token
      ╭▸ test:1:5
      │
    1 │   @hi mom("this should be invalid")
      │ ┏━━━━━┛
    2 │ ┃ permit (principal, action, resource);
      ╰╴┗━┛ unexpected token
    "#);
}

#[test]
fn policy_annotations_bad_id_3() {
    let source = source! {r#"
        @hi+mom("this should be invalid")
        permit (principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 2);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: missing policy effect
      ╭▸ test:1:1
      │
    1 │ @hi+mom("this should be invalid")
      ╰╴━━━ expected `permit` or `forbid`
    error: unexpected token
      ╭▸ test:1:4
      │
    1 │   @hi+mom("this should be invalid")
      │ ┏━━━━┛
    2 │ ┃ permit (principal, action, resource);
      ╰╴┗━┛ unexpected token
    "#);
}

#[test]
fn policy_annotations_bad_val_2() {
    let source = source! {r"
        @bad_annotation()
        permit (principal, action, resource);
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected a string literal
      ╭▸ test:1:17
      │
    1 │ @bad_annotation()
      ╰╴                ━ expected a string literal
    ");
}

#[test]
fn extended_has_20() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has a.(b)
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: missing expression
      ╭▸ test:3:19
      │
    3 │       principal has a.(b)
      │ ┏━━━━━━━━━━━━━━━━━━━┛
    4 │ ┃ };
      ╰╴┗━┛ expected an attribute name
    ");
}

#[test]
fn policies6() {
    let source = source! {r#"
        3(principal: p, action: a, resource: r)
        when { w }
        unless { u }
        advice { "doit" };

        permit (principal: p, action: a, resource: r)
        when { w }
        unless { u }
        advice { "doit" };

        permit (principal: p, action: a, resource: r)
        when { w }
        unless { u }
        advice { "doit" };
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 7);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: unexpected token
      ╭▸ test:1:1
      │
    1 │ ┏ 3(principal: p, action: a, resource: r)
    2 │ ┃ when { w }
    3 │ ┃ unless { u }
    4 │ ┃ advice { "doit" };
    5 │ ┃
    6 │ ┃ permit (principal: p, action: a, resource: r)
      ╰╴┗━┛ unexpected token
    error: unknown variable `w `
      ╭▸ test:7:8
      │
    7 │ when { w }
      │        ━━ not a valid variable
      ╰╴
    note: `principal`, `action`, `resource`, and `context` are the only variables
    error: unknown variable `u `
      ╭▸ test:8:10
      │
    8 │ unless { u }
      │          ━━ not a valid variable
      ╰╴
    note: `principal`, `action`, `resource`, and `context` are the only variables
    error: unexpected token
       ╭▸ test:9:1
       │
     9 │ ┏ advice { "doit" };
    10 │ ┃
    11 │ ┃ permit (principal: p, action: a, resource: r)
       ╰╴┗━┛ unexpected token
    error: unknown variable `w `
       ╭▸ test:12:8
       │
    12 │ when { w }
       │        ━━ not a valid variable
       ╰╴
    note: `principal`, `action`, `resource`, and `context` are the only variables
    error: unknown variable `u `
       ╭▸ test:13:10
       │
    13 │ unless { u }
       │          ━━ not a valid variable
       ╰╴
    note: `principal`, `action`, `resource`, and `context` are the only variables
    error: missing policy effect
       ╭▸ test:14:1
       │
    14 │ advice { "doit" };
       ╰╴━━━━━━━━━━━━━━━━━━ expected `permit` or `forbid`
    "#);
}
