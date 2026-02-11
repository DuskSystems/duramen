//! Source: <https://github.com/cedar-policy/cedar/blob/v4.9.0/cedar-policy-core/src/parser/text_to_cst.rs>.

use duramen_diagnostic::Diagnostics;
use duramen_parser::PolicyParser;
use duramen_test::source;

#[test]
fn expr1() {
    let source = source! {r"
        permit (principal, action, resource)
        when { 1 };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn expr2() {
    let source = source! {r#"
        permit (principal, action, resource)
        when { "string" };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn expr3() {
    let source = source! {r#"
        permit (principal, action, resource)
        when { "string".foo == !7 };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn expr4() {
    let source = source! {r"
        permit (principal, action, resource)
        when { 5 < 3 || -7 == 2 && 3 >= 6 };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn expr5() {
    let source = source! {r#"
        permit (principal, action, resource)
        when { if 7 then 6 > 5 else !5 || "thursday" };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn expr6() {
    let source = source! {r#"
        permit (principal, action, resource)
        when { if 7 then 6 > 5 else !5 || "thursday" && ((8) >= "fish") };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn expr_overflow_1() {
    let source = source! {r"
        permit (principal, action, resource)
        when { principal == -5555555555555555555555 };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn expr_overflow_2() {
    let source = source! {r"
        permit (principal, action, resource)
        when { principal == 5555555555555555555555 };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn variable1() {
    let source = source! {r"
        permit (principal, action, var: h in 1);
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn variable2() {
    let source = source! {r"
        permit (principal, action, more in 2);
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn variable3() {
    let source = source! {r"
        permit (principal, action: a_name, resource);
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn variable4() {
    let source = source! {r"
        permit (principalorsomeotherident, action, resource);
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn variable6() {
    let source = source! {r"
        permit (var : in 6, action, resource);
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn member1() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            2._field // oh, look, comments!
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn member2() {
    let source = source! {r#"
        permit (principal, action, resource)
        when {
            "first".some_ident()
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn member3() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            [2, 3, 4].foo[2]
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn member4() {
    let source = source! {r#"
        permit (principal, action, resource)
        when {
            {3<-4:"what?","ok then":-5>4}
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn member5() {
    let source = source! {r#"
        permit (principal, action, resource)
        when {
            [3 < 4, "ok then", 17, ("none")]
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn member6() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            one.two
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn member7() {
    let source = source! {r#"
        permit (principal, action, resource)
        when {
            one{num: true, trivia: "first!"}
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn member8() {
    let source = source! {r#"
        permit (principal, action, resource)
        when {
            {2: true, 4: me}.with["pizza"]
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn member9() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            AllRects({two: 2, four: 3 + 5 / 5})
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn ident3_1() {
    let source = source! {r"
        permit (principal, action, resource)
        when { if };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn ident3_2() {
    let source = source! {r"
        permit (principal, action, resource)
        when { foo };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// NOTE: Same as: ident3_2
// #[test]
// fn ident3_3() {}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn ident3_4() {
    let source = source! {r"
        permit (principal, action, resource)
        when { if::then::else };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn ident3_5() {
    let source = source! {r"
        permit (principal, action, resource)
        when { if::true::then::false::else::true };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn ident4_1() {
    let source = source! {r"
        permit (principal, action, resource)
        when { true(true) };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn ident4_2() {
    let source = source! {r"
        permit (principal, action, resource)
        when { if(true) };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn ident5_1() {
    let source = source! {r"
        permit (principal, action, resource)
        when { {true : false} };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn ident5_2() {
    let source = source! {r"
        permit (principal, action, resource)
        when { { if : true } };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn ident6_1() {
    let source = source! {r"
        permit (principal, action, resource)
        when { {true : false} has false };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn ident6_2() {
    let source = source! {r"
        permit (principal, action, resource)
        when { { if : true } has if };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn comments_has() {
    let source = source! {r"
        permit (principal, action,resource)
        when { principal //comment p
        has //comment has
        age //comment
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn comments_like() {
    let source = source! {r"
        permit (principal, action,resource)
        when { principal //comment p
        like //comment like

        age //comment
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn comments_and() {
    let source = source! {r#"
        permit (principal, action,resource)
        when { 1 //comment p
        &&  //comment &&
            //comment &&
        "hello" //comment
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn comments_or() {
    let source = source! {r#"
        permit (principal, action,resource)
        when { 1 //comment 1
              //  comment 1
        ||  //comment ||
            //comments ||
        "hello" //comment
                //comment hello
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn comments_add() {
    let source = source! {r"
        permit (principal, action,resource)
        when { 1 //comment 1
                //comment 1_2
        + //comment +
           //comment +
         2 //comment 2
            //comment 2
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn comments_paren() {
    let source = source! {r"
        permit (principal, action,resource)
        when {
        ( //comment 1
            ( //comment 2
         1
            ) //comment 3
        ) //comment 4
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn comments_set() {
    let source = source! {r#"
        permit (principal, action,resource)
        when {
        [ // comment 1
        "hello" //comment 2
        , // comment 3
         // comment 3-2
        1 //comment 4
            //comment 5
        ]  //comment 5-0

        .  //comment 5-1

        contains //comment 5-2

        ( //comment 6

        "a"  //comment 7

        ) //comment 20
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn comments_if() {
    let source = source! {r#"
        permit (principal, action,resource)
        when {
        ( //comment open outer
        ( //comment open inner
         if //comment if
          1             //comment
          < //comment <
          2 //comment 2
          then // comment then
          "hello" //comment hello
        else  //comment else
            1 //comment 1
            ) //comment close inner
            ) //comment close outer
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn comments_member_access() {
    let source = source! {r"
        permit (principal, action,resource)
        when { principal. //comment .
        age // comment age
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn comments_principal() {
    let source = source! {r#"
        permit (principal //comment 1
         ==
          User::"alice" //comment 3
          ,  //comment 4
           action,resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn comments_annotation() {
    let source = source! {r#"
        //comment policy
        // comment policy 2
        @anno("good annotation")  // comments after annotation
        // comments after annotation 2
        permit (principal //comment 1
         ==
          User::"alice" //comment 3
          ,  //comment 4
           action,resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn comments_policy_1() {
    let source = source! {r#"
        //comment policy 1
        //comment policy 2
        permit ( //comment 3
           // comment 4
        principal //comment principal
        == //comment == 1
           //comment == 2
        User::"alice" //comment alice
        , //comment comma 1
                    //comment comma 2
        action //comment action 1
        //comment action 2
        , //comment comma action
        resource // comment resource
        )
        //comment 5
        //comment 6
        ;
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
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

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn comments_policy_3() {
    let source = source! {r"
        permit (principal, action, resource)
        when { 1 /* multi-line
        comment */d };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn no_comments_policy() {
    let source = source! {r#"
        permit (
        principal
        ==
        User::"alice"
        ,
        action

        ,
        resource
        )
        ;
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn no_comments_policy2() {
    let source = source! {r#"
        permit (
            principal == IAM::Principal::"arn:aws:iam::12345678901:user/Dave",
            action == S3::Action::"GetAccountPublicAccessBlock",
            resource == Account::"12345678901"
        );
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn no_comments_policy4() {
    let source = source! {r#"
        permit (principal, action, resource, context)
        when {
            context.contains(3, "four", five(6, 7))
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn no_comments_policy5() {
    let source = source! {r#"
        permit (
            principal,
            action,
            resource == Album::{uid: "772358b3-de11-42dc-8681-f0a32e34aab8",
            displayName: "vacation_photos"}
        );
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn policies1() {
    let source = source! {r#"
        permit (principal: p, action: a, resource: r)
        when { w }
        unless { u }
        advice { "doit" };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn policies2() {
    let source = source! {r#"
        permit (
            principal in Group::"jane_friends",  // Policy c1
            action in [PhotoOp::"view", PhotoOp::"comment"],
            resource in Album::"jane_trips",
            context: Group
        );
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn policies3() {
    let source = source! {r#"
        forbid (principal, action, resource)           // Policy c2
        when   { "private" in resource.tags }  // resource.tags is a set of strings
        unless { resource in user.account };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn policies3p() {
    let source = source! {r#"
        forbid (principality, action, resource)           // Policy c2
        when   { "private" in resource.tags }  // resource.tags is a set of strings
        unless { resource in user.account };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn policies4() {
    let source = source! {r#"
        permit (principal: p, action: a, resource: r)
        when { w }
        unless { u }
        advice { "doit" };

        permit (principal in Group::"jane_friends",  // Policy c1
        action in [PhotoOp::"view", PhotoOp::"comment"],
        resource in Album::"jane_trips");

        forbid (principal, action, resource)           // Policy c2
        when   { "private" in resource.tags }  // resource.tags is a set of strings
        unless { resource in user.account };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn policies5() {
    let source = source! {r#"
        permit (
            principal == User::"alice",
            action in PhotoflashRole::"viewer",
            resource in Account::"jane"
        )
        advice {
            "{\"type\":\"PhotoFilterInstruction\", \"anonymize\":true}"
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
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

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn policy_annotations_ok() {
    let source = source! {r#"
        @anno("good annotation")
        permit (principal, action, resource);

        @anno1("good")
        @anno2("annotation")
        permit (principal, action, resource);

        @long6wordphraseisident007("good annotation")
        permit (principal, action, resource);

        @   spacy  (  "  good  annotation  "  )
        permit (principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn policy_annotations_no_value_ok() {
    let source = source! {r"
        @foo
        permit (principal, action, resource);
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn policy_annotations_bad_id_1() {
    let source = source! {r#"
        @bad-annotation("bad")
        permit (principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn policy_annotations_bad_id_2() {
    let source = source! {r#"
        @hi mom("this should be invalid")
        permit (principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn policy_annotations_bad_id_3() {
    let source = source! {r#"
        @hi+mom("this should be invalid")
        permit (principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn policy_annotations_bad_val_1() {
    let source = source! {r#"
        @bad_annotation("bad", "annotation")
        permit (principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn policy_annotations_bad_val_2() {
    let source = source! {r"
        @bad_annotation()
        permit (principal, action, resource);
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn policy_annotations_bad_val_3() {
    let source = source! {r"
        @bad_annotation(bad_annotation)
        permit (principal, action, resource);
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn policy_annotation_bad_position() {
    let source = source! {r#"
        permit (@comment("your name here") principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn error_recovery_1() {
    let source = source! {r"
        permit (principal, action, !)
        when { principal.foo == resource.bar};

        permit (principal, action, resource);
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn error_recovery_2() {
    let source = source! {r"
        permit (principal, action, !)
        when { principal.foo == resource.bar};

        permit (principal, action, +);
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn error_recovery_3() {
    let source = source! {r"
        permit (principal, action, !)
        when { principal.foo == resource.bar}
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_1() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has a.b
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_2() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has a.if
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_3() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has if.a
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_4() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has if.if
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_5() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has true.if
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_6() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has if.true
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_7() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has if.then.else.in.like.has.is.__cedar
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_8() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has 1+1
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_9() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has a - 1
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_10() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has a*3 + 1
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_11() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has 3*a
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_12() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has -a.b
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_13() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has !a.b
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_14() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has a::b.c
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_15() {
    let source = source! {r#"
        permit (principal, action, resource)
        when {
            principal has A::""
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_16() {
    let source = source! {r#"
        permit (principal, action, resource)
        when {
            principal has A::"".a
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_17() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has ?principal
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_18() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has ?principal.a
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn extended_has_19() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has (b).a
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn extended_has_20() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has a.(b)
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn extended_has_21() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal has a.1
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn trailing_comma_1() {
    let source = source! {r"
        permit (principal, action, resource,);
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn trailing_comma_2() {
    let source = source! {r"
        permit (principal, action, resource)
        when { foo(a, b, c,) };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn trailing_comma_3() {
    let source = source! {r"
        permit (principal, action, resource)
        when { [A, B, C,] };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn trailing_comma_4() {
    let source = source! {r"
        permit (principal, action, resource)
        when { { A: B, C: D, } };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn trailing_comma_5() {
    let source = source! {r#"
        permit (principal == Principal::{uid: "123", role: "admin",}, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn invalid_token_1() {
    let source = source! {r"
        permit (principal, action, resource)
        when { ~ };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn invalid_token_2() {
    let source = source! {"
        permit (principal, action, resource)
        when { \u{1F680} };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn unclosed_strings_1() {
    let source = source! {r#"
        permit (principal, action, resource)
        when {
            principal.foo = "bar
        };
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn unclosed_strings_2() {
    let source = source! {r#"
        permit (principal, action, resource == Photo::"mine.jpg);
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn unclosed_strings_3() {
    let source = source! {r#"
        @id("0)permit (principal, action, resource);
    "#};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
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

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn single_quote_string_1() {
    let source = source! {r"
        permit (principal, action, resource)
        when {
            principal.foo = 'bar'
        };
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn single_quote_string_2() {
    let source = source! {r"
        permit (principal, action, resource == Photo::'mine.jpg');
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}

// UPSTREAM
// See: crates/lowerer/tests/upstream_parser_policy.rs
#[test]
fn single_quote_string_3() {
    let source = source! {r"
        @id('0')permit (principal, action, resource);
    "};

    let mut diagnostics = Diagnostics::new();

    let _tree = PolicyParser::new(source, &mut diagnostics).parse();
    assert_eq!(diagnostics.len(), 0);
}
