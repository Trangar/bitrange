#[cfg(test)]
mod test_get {

    bitrange! {
        IpHeader: u32, "u32",                    // struct name
        "aaaa_bbbb_cccccccc_dddddddddddddddd",   // pattern that we're matching against
        a: version,                              // map character 'a' to field 'version'
        b: ihl,                                  // map character 'b' to field 'ihl'
        c: type_of_service,                      // map character 'c' to field 'type_of_service'
        d: total_length                          // map character 'd' to field 'total_length'
    }

    #[test]
    fn test() {
        let header = IpHeader::from(0b0001_0010_00000011_0000000000000100).unwrap();
        assert_eq!(header.version(), 0b0001);
        assert_eq!(header.ihl(), 0b0010);
        assert_eq!(header.type_of_service(), 0b0011);
        assert_eq!(header.total_length(), 0b0100);
    }
}
#[cfg(test)]
mod test_set {
    bitrange! {
        IpHeader: u32, "u32",                    // struct name
        "aaaa_bbbb_cccccccc_dddddddddddddddd",   // pattern that we're matching against
        a: version set_version,                  // map character 'a' to field 'version', and create setter 'set_version'
        b: ihl,                                  // map character 'b' to field 'ihl'
        c: type_of_service,                      // map character 'c' to field 'type_of_service'
        d: total_length                          // map character 'd' to field 'total_length'
    }

    #[test]
    fn test() {
        let mut header = IpHeader::from(0b0001_0010_00000011_0000000000000100).unwrap();
        assert_eq!(header.version(), 0b0001);
        assert_eq!(header.ihl(), 0b0010);
        assert_eq!(header.type_of_service(), 0b0011);
        assert_eq!(header.total_length(), 0b0100);

        header.set_version(0b0100);
        assert_eq!(header.version(), 0b0100);
    }

}
