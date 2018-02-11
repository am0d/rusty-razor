mod views;
use views::*;

#[test]
fn test_() {
    let expected = r###""###;
    let view = Empty::new();
    let view_result = format!("{}", view);
    assert_eq!(view_result, expected);
}

#[test]
fn test_hello() {
    let expected = r###"<span>Hello from rusty-razor</span>"###;
    let view = Hello::new();
    let view_result = format!("{}", view);
    assert_eq!(view_result, expected);
}
