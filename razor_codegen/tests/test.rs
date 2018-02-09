mod hello;
mod empty;

#[test]
fn test_() {
    use empty;
    let expected = r###""###;
    let view = empty::Empty::new();
    let view_result = format!("{}", view);
    assert_eq!(view_result, expected);
}

#[test]
fn test_hello() {
    use hello;
    let expected = r###"<span>Hello from rusty-razor</span>"###;
    let view = hello::Hello::new();
    let view_result = format!("{}", view);
    assert_eq!(view_result, expected);
}
