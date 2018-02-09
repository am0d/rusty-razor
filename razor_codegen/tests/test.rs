mod hello;

#[test]
fn test_hello() {
    let expected = r###"

<span>Hello from rusty-razor</span>"###;
    let model = ();
    let view = hello::Hello::new(model);
    let view_result = format!("{}", view);
    assert_eq!(view_result, expected);
}
