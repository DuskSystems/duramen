use duramen_suggest::suggest;

#[test]
fn exact() {
    assert_eq!(
        suggest("principal", &["principal", "action", "resource", "context"]),
        Some("principal")
    );
}

#[test]
fn typo() {
    assert_eq!(
        suggest("princpal", &["principal", "action", "resource", "context"]),
        Some("principal")
    );
}

#[test]
fn none() {
    assert_eq!(
        suggest("xyz", &["principal", "action", "resource", "context"]),
        None
    );
}

#[test]
fn empty() {
    assert_eq!(suggest("", &["abc", "def"]), None);
}

#[test]
fn empty_candidates() {
    assert_eq!(suggest("foo", &[]), None);
}

#[test]
fn best() {
    assert_eq!(
        suggest("resouce", &["resource", "result", "rescue"]),
        Some("resource")
    );
}

#[test]
fn tie() {
    assert_eq!(suggest("ab", &["aa", "bb"]), Some("aa"));
}
