#[test]
#[should_panic]
fn test2() {
    bitrange! {
        Test: u8,
        [aaa1_0bbb],
        a: first,
        b: second
    }
    // Because the pattern is aaa1_0bbb
    // This means that the 5th bit should always be 1
    // This means that this contructor should always panic
    Test::from(0); 
}