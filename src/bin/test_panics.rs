#[test]
#[cfg(not(feature = "panic"))]
fn test_error() {
    bitrange! {
        Test: u8,
        [1111_111a],
        a: _first
    }
    // Because the pattern is 1111_111
    // This means that all bits, except the last, should always be 1
    // Because `0` does not match this criteria, this function panics
    if let Err(e) = Test::from(0) {
        assert_eq!(0b1111_1110, e.expected);
        assert_eq!(0, e.provided);
    } else {
        panic!("Test should have failed but didn't");
    }
}

#[test]
#[cfg(feature = "panic")]
#[should_panic]
fn test_panic() {
    bitrange! {
        Test: u8,
        [1111_111a],
        a: _first
    }
    // Because the pattern is 1111_111
    // This means that all bits, except the last, should always be 1
    // Because `0` does not match this criteria, this function panics
    let _ = Test::from(0);
}
