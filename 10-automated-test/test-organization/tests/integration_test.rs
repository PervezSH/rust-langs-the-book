use test_organization;
mod common;
// we don't need the #[cfg(test)] annotation
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_organization::add_two(2));
}
