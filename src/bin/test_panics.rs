#[test]
#[should_panic]
fn test2() {
    bitrange! {
        Test: u8,
        [1111_111a],
        a: _first
    }
    // Because the pattern is 1111_111
    // This means that all bits, except the last, should always be 1
    // Because `0` does not match this criteria, this function panics
    Test::from(0);
}
