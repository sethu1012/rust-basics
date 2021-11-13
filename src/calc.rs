//! Module Documentation
//! # Examples
//! ```
//! println!("{}", add(2, 3));
//! println!("{}", add(4, 5));
//! ```
#![allow(dead_code)]

/// Function Documentation
pub fn add(a: i32, b: i32) -> i32{
    a + b
}

#[test]
fn add_check1() {
    assert_eq!(5, add(2, 3));
}

#[test]
#[should_panic]
#[ignore]
fn add_check2() {
    assert_eq!(-4, add(-2, -3));
}