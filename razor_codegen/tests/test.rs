mod views;
use views::*;

macro_rules! assert_view_result {
    ($view: expr, $expected: expr) => {
        let view = $view;
        let view_result = format!("{}", view);
        assert_eq!(view_result, $expected);
    };
}

#[test]
fn test_empty() {
    let expected = r###""###;
    assert_view_result!(Empty::new(), expected);
}

#[test]
fn test_hello() {
    let expected = r###"<span>Hello from rusty-razor</span>"###;
    assert_view_result!(Hello::new(), expected);
}

#[test]
fn test_email() {
    let expected = r###"test@example.com"###;
    assert_view_result!(Email::new(), expected);
}

#[test]
fn test_literal_at() {
    let expected = r###"Meet @ 4pm."###;
    assert_view_result!(LiteralAt::new(), expected);
}

#[test]
fn test_code_string() {
    let expected = r###"

Hello, Razor!"###;
    assert_view_result!(CodeString::new(String::from("Razor!")), expected);
}
