#![feature(plugin)]
#![plugin(bitrange)]

fn main(){
    let test = Test { bits: 0 };
    println!("{:?}", test);
    println!("first: {:?}", test.first());
    println!("second: {:?}", test.second());
}

#[test]
fn test_basic() {

    let test = Test { bits: 0b1110_0000 };
    assert_eq!(test.first(), 0b111);
    assert_eq!(test.second(), 0b0000);

    let test = Test { bits: 0b0111_0000 };
    assert_eq!(test.first(), 0b011);
    assert_eq!(test.second(), 0b0000);

    let test = Test { bits: 0b0011_1000 };
    assert_eq!(test.first(), 0b001);
    assert_eq!(test.second(), 0b1000);

    let test = Test { bits: 0b001_1100 };
    assert_eq!(test.first(), 0b000);
    assert_eq!(test.second(), 0b1100);

    let test = Test { bits: 0b0000_1110 };
    assert_eq!(test.first(), 0b000);
    assert_eq!(test.second(), 0b1110);

    let test = Test { bits: 0b0000_0111 };
    assert_eq!(test.first(), 0b000);
    assert_eq!(test.second(), 0b0111);
}

/*
#[test]
fn test_mut() {
    bitrange! {
        Test
        [aaa_bbbb],
        a: first,
        b: second,
    }

    let mut test = Test { bits: 0 };
    assert_eq!(test.first(), 0b000);
    assert_eq!(test.second(), 0b0000);

    test.set_first(0b010);
    assert_eq!(test.first(), 0b010);
    assert_eq!(test.second(), 0b0000);
    assert_eq!(test.bits, 0b0100_0000);
}

#[test]
fn test_gap() {
    bitrange! {
        Test
        [aabb_bbaa],
        a: first,
        b: second
    }

    let test = Test { bits: 0b1111_1111 };
    assert_eq!(test.first(), 0b1100_0011);
    // second is bb_bbxx, so it gets shifted to the right 2 bits
    // 0b1111_1111 & bb_bbxx = 0b0011_1100 -> bbbb = 0b0000_1111
    assert_eq!(test.second(), 0b0000_1111);
}
*/
bitrange! {
    Test
    [aaa_bbbb],
    a: first,
    b: second,
}