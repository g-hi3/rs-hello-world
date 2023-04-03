mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

pub mod adder {
    pub fn add_two(n: i32) -> i32 {
        n + 2
    }
}