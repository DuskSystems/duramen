//! Source: <https://github.com/cedar-policy/cedar/blob/v4.9.0/cedar-policy-core/src/parser/text_to_cst.rs>.

use duramen_cst::{CstNode as _, Policies};
use duramen_diagnostic::Diagnostics;
use duramen_lower::PolicyLowerer;
use duramen_parser::PolicyParser;
use duramen_test::{assert_diagnostics_snapshot, source};

#[test]
#[ignore = "TODO: implement in lowerer"]
fn variable6() {
    let source = source! {r"
        permit(var : in 6, action, resource);
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `)`
      ╭▸ test:1:17
      │
    1 │ permit(var : in 6, action, resource);
      ╰╴                ━ expected `)`
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn member7() {
    let source = source! {r#"
        permit(principal, action, resource)
        when {
            one{num:true,trivia:"first!"}
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: expected `}`
      ╭▸ test:3:8
      │
    3 │     one{num:true,trivia:"first!"}
      ╰╴       ━ expected `}`
    "#);
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn ident3_1() {
    let source = source! {r"
        permit(principal, action, resource)
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
      ╭▸ test:2:12
      │
    2 │ when { if };
      ╰╴           ━ expected `}`
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn ident3_4() {
    let source = source! {r"
        permit(principal, action, resource)
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
      ╭▸ test:2:24
      │
    2 │ when { if::then::else };
      ╰╴                       ━ expected `}`
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn ident3_5() {
    let source = source! {r"
        permit(principal, action, resource)
        when { if::true::then::false::else::true };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `}`
      ╭▸ test:2:12
      │
    2 │ when { if::true::then::false::else::true };
      ╰╴           ━━━━ expected `}`
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn comments_policy_3() {
    let source = source! {r"
        permit(principal, action, resource)
        when { 1 /* multi-line
        comment */d };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `}`
      ╭▸ test:2:13
      │
    2 │ when { 1 /* multi-line
      ╰╴            ━━━━━ expected `}`
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn no_comments_policy4() {
    let source = source! {r#"
        permit(principal,action,resource,context)
        when {
            context.contains(3,"four",five(6,7))
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `)`
      ╭▸ test:1:34
      │
    1 │ permit(principal,action,resource,context)
      ╰╴                                 ━━━━━━━ expected `)`
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn policies2() {
    let source = source! {r#"
        permit(
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
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `)`
      ╭▸ test:5:5
      │
    5 │     context:Group
      ╰╴    ━━━━━━━ expected `)`
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn policy_annotations_bad_val_1() {
    let source = source! {r#"
        @bad_annotation("bad","annotation")
        permit (principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: expected `)`
      ╭▸ test:1:22
      │
    1 │ @bad_annotation("bad","annotation")
      ╰╴                     ━ expected `)`
    "#);
}

#[test]
#[ignore = "TODO: implement in lowerer"]
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
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `)`
      ╭▸ test:1:17
      │
    1 │ @bad_annotation(bad_annotation)
      ╰╴                ━━━━━━━━━━━━━━ expected `)`
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn policy_annotation_bad_position() {
    let source = source! {r#"
        permit (@comment("your name here") principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: expected `)`
      ╭▸ test:1:9
      │
    1 │ permit (@comment("your name here") principal, action, resource);
      ╰╴        ━ expected `)`
    "#);
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn error_recovery_1() {
    let source = source! {r"
        permit(principal, action, !)
        when { principal.foo == resource.bar};

        permit(principal, action, resource);
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `)`
      ╭▸ test:1:27
      │
    1 │ permit(principal, action, !)
      ╰╴                          ━ expected `)`
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn error_recovery_2() {
    let source = source! {r"
        permit(principal, action, !)
        when { principal.foo == resource.bar};

        permit(principal, action, +);
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 2);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `)`
      ╭▸ test:1:27
      │
    1 │ permit(principal, action, !)
      ╰╴                          ━ expected `)`
    error: expected `)`
      ╭▸ test:4:27
      │
    4 │ permit(principal, action, +);
      ╰╴                          ━ expected `)`
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn error_recovery_3() {
    let source = source! {r"
        permit(principal, action, !)
        when { principal.foo == resource.bar}
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `)`
      ╭▸ test:1:27
      │
    1 │ permit(principal, action, !)
      ╰╴                          ━ expected `)`
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn extended_has_21() {
    let source = source! {r"
        permit(principal, action, resource)
        when {
          principal has a.1
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `}`
      ╭▸ test:3:19
      │
    3 │   principal has a.1
      ╰╴                  ━ expected `}`
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn invalid_token_1() {
    let source = source! {r"
        permit(principal, action, resource)
        when { ~ };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: unrecognized character
      ╭▸ test:2:8
      │
    2 │ when { ~ };
      ╰╴       ━ not valid in Cedar
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn invalid_token_2() {
    let source = source! {"
        permit(principal, action, resource)
        when { \u{1F680} };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 1);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: unrecognized character
      ╭▸ test:2:8
      │
    2 │ when { 🚀 };
      ╰╴       ━━ not valid in Cedar
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn unclosed_strings_1() {
    let source = source! {r#"
        permit(principal, action, resource)
        when {
            principal.foo = "bar
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 3);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
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
    error: unterminated string literal
      ╭▸ test:3:21
      │
    3 │       principal.foo = "bar
      │ ┏━━━━━━━━━━━━━━━━━━━━━┛
    4 │ ┃ };
      ╰╴┗━━━┛ missing closing `"`
    error: expected `}`
      ╭▸ test:4:4
      │
    4 │ };
      ╰╴  ━ expected `}`
    "#);
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn unclosed_strings_2() {
    let source = source! {r#"
        permit(principal, action, resource == Photo::"mine.jpg);
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 2);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: expected `)`
      ╭▸ test:1:46
      │
    1 │ permit(principal, action, resource == Photo::"mine.jpg);
      ╰╴                                             ━━━━━━━━━━━ expected `)`
    error: unterminated string literal
      ╭▸ test:1:46
      │
    1 │ permit(principal, action, resource == Photo::"mine.jpg);
      ╰╴                                             ━━━━━━━━━━━ missing closing `"`
    "#);
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn unclosed_strings_3() {
    let source = source! {r#"
        @id("0)permit(principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 2);

    assert_diagnostics_snapshot!(source, &diagnostics, @r#"
    error: expected `)`
      ╭▸ test:1:5
      │
    1 │ @id("0)permit(principal, action, resource);
      ╰╴    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ expected `)`
    error: unterminated string literal
      ╭▸ test:1:5
      │
    1 │ @id("0)permit(principal, action, resource);
      ╰╴    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ missing closing `"`
    "#);
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn unclosed_strings_4() {
    let source = source! {r#"
        @id("0)
        permit(principal, action, resource)
        when {
            principal.foo = "bar"
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
      ╭▸ test:4:22
      │
    4 │     principal.foo = "bar"
      ╰╴                     ━━━ expected `)`
    error: unterminated string literal
      ╭▸ test:4:25
      │
    4 │       principal.foo = "bar"
      │ ┏━━━━━━━━━━━━━━━━━━━━━━━━━┛
    5 │ ┃ };
      ╰╴┗━━━┛ missing closing `"`
    "#);
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn single_quote_string_1() {
    let source = source! {r"
        permit(principal, action, resource)
        when {
            principal.foo = 'bar'
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 4);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
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
    error: unrecognized character
      ╭▸ test:3:21
      │
    3 │     principal.foo = 'bar'
      ╰╴                    ━ not valid in Cedar
    error: expected `}`
      ╭▸ test:3:22
      │
    3 │     principal.foo = 'bar'
      ╰╴                     ━━━ expected `}`
    error: unrecognized character
      ╭▸ test:3:25
      │
    3 │     principal.foo = 'bar'
      ╰╴                        ━ not valid in Cedar
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn single_quote_string_2() {
    let source = source! {r"
        permit(principal, action, resource == Photo::'mine.jpg');
    "};

    let mut diagnostics = Diagnostics::new();

    let tree = PolicyParser::new(source, &mut diagnostics).parse();
    let root = tree.root().unwrap();

    let cst = Policies::cast(root).unwrap();
    let _ast = PolicyLowerer::new(source, &mut diagnostics).lower(cst);
    assert_eq!(diagnostics.len(), 3);

    assert_diagnostics_snapshot!(source, &diagnostics, @"
    error: expected `)`
      ╭▸ test:1:46
      │
    1 │ permit(principal, action, resource == Photo::'mine.jpg');
      ╰╴                                             ━ expected `)`
    error: unrecognized character
      ╭▸ test:1:46
      │
    1 │ permit(principal, action, resource == Photo::'mine.jpg');
      ╰╴                                             ━ not valid in Cedar
    error: unrecognized character
      ╭▸ test:1:55
      │
    1 │ permit(principal, action, resource == Photo::'mine.jpg');
      ╰╴                                                      ━ not valid in Cedar
    ");
}

#[test]
#[ignore = "TODO: implement in lowerer"]
fn single_quote_string_3() {
    let source = source! {r"
        @id('0')permit(principal, action, resource);
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
    1 │ @id('0')permit(principal, action, resource);
      ╰╴    ━ expected `)`
    error: unrecognized character
      ╭▸ test:1:5
      │
    1 │ @id('0')permit(principal, action, resource);
      ╰╴    ━ not valid in Cedar
    error: unrecognized character
      ╭▸ test:1:7
      │
    1 │ @id('0')permit(principal, action, resource);
      ╰╴      ━ not valid in Cedar
    ");
}
