#![doc(hidden)]

#![deny(warnings)]
#![feature(proc_macro)]
#![feature(proc_macro_non_items)]
#[cfg(test)]
#[macro_use]
extern crate bitrange;

pub mod test_ip;
pub mod test_panics;

#[test]
fn test_default() {
    bitrange! {
        Test: u8,
        [aaa1_0bbb],
        a: first,
        b: second
    }

    let test = Test::default();
    assert_eq!(test.bits, 0b0001_0000);
    assert_eq!(test.first(), 0);
    assert_eq!(test.second(), 0);
}

#[test]
fn test_default_2() {
    bitrange! {
        Test: u8,
        [aaa1_0bbb],
        a: first
    }

    let test = Test::default();
    assert_eq!(test.bits, 0b0001_0000);
    println!("{:?}", test.first());
}

fn main() {}
