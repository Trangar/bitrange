#![feature(proc_macro)]
#[macro_use]
extern crate bitrange;

pub mod test_ip;
pub mod test_panics;

#[test]
fn test_default(){
    bitrange! {
        Test: u8,
        [aaa1_0bbb],
        a: first,
        b: second
    }

    let test = Test::default();
    assert_eq!(test.bits, 0b0001_0000);
}


#[test]
fn test_default_2(){
    bitrange! {
        Test: u8,
        [aaa1_0bbb],
        a: first
    }

    let test = Test::default();
    assert_eq!(test.bits, 0b0001_0000);
    println!("{:?}", test.first());
}

fn main(){

}
