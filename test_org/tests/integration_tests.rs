use test_org::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    let result = add_two(3);
    assert_eq!(result, 5)
}