# bitrange

## getting started

To get started, add this to your `Cargo.toml`:
```
bitrange = "0.1.0"
```

Then add the following code to your `main.rs` or `lib.rs`
```
#![feature(proc_macro)]
#[macro_use]
extern crate bitrange;
```

bitrange needs a nightly version of the compiler because it uses the feature `proc_macro` which is not stabilized yet

<b>The last field may not have a trailing comma at this point in time</b>

## examples

Bitrange helps you map bit fields to proper getters and setters.

Say you're trying to make an IP parser. The [rfc](https://tools.ietf.org/html/rfc791#section-3.1) will give you this:

```
    0                   1                   2                   3
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |Version|  IHL  |Type of Service|          Total Length         |
```

If you wanted to parse this in Rust, you'd have to make the following mapping:
* the first 4 bits are mapped to `version`
* The next 4 bits are mapped to `ihl`
* The next 8 bits are mapped to `type_of_service`
* The last 16 bits are mapped to `total_length`

With bitrange, you can easily map bytes to fields. To parse this part of the protocol, simply write

``` rust
#![feature(proc_macro)]
#[macro_use]
extern crate bitrange;

bitrange! {
    IpHeader: u32,                           // struct name
    [aaaa_bbbb_cccccccc_dddddddddddddddd],   // pattern that we're matching against
    a: version,                              // map character 'a' to field 'version'
    b: ihl,                                  // map character 'b' to field 'ihl'
    c: type_of_service,                      // map character 'c' to field 'type_of_service'
    d: total_length                          // map character 'd' to field 'total_length'
}

fn main() {
    let header = IpHeader::from(0b0001_0010_00000011_0000000000000100);
    assert_eq!(header.version(), 0b0001);
    assert_eq!(header.ihl(), 0b0010);
    assert_eq!(header.type_of_service(), 0b0011);
    assert_eq!(header.total_length(), 0b0100);
}
```

If you wanted to make a field mutable, simply add a second ident to the field mapping, e.g.:

``` rust

bitrange! {
    IpHeader: u32,                           // struct name
    [aaaa_bbbb_cccccccc_dddddddddddddddd],   // pattern that we're matching against
    a: version set_version,                  // map character 'a' to field 'version', and create setter 'set_version'
    b: ihl,                                  // map character 'b' to field 'ihl'
    c: type_of_service,                      // map character 'c' to field 'type_of_service'
    d: total_length                          // map character 'd' to field 'total_length'
}

fn main() {
    let mut header = IpHeader::from(0b0001_0010_00000011_0000000000000100);
    assert_eq!(header.version(), 0b0001);
    assert_eq!(header.ihl(), 0b0010);
    assert_eq!(header.type_of_service(), 0b0011);
    assert_eq!(header.total_length(), 0b0100);

    header.set_version(0b0100);
    assert_eq!(header.version(), 0b0100);
}
```

In addition, you can define constraints to bits that have to always be 0 or 1
``` rust

bitrange! {
    Test: u8,
    // from left (highest) to right (lowest)
    // first 3 bits are mapped to a
    // the next bit is always 1
    // the next bit is always 0
    // the last 3 bits are mapped to b
    [aaa1_0bbb],
    a: first,
    b: second
}

fn main() {
    // This panics at runtime
    // Because the 4th highest bit should always be 1
    // Test::from(0);

    // The enum also implements Default, so you can simply do:
    let _test = Test::default();
}
```

## Compile-time checks

bitrange will also check fields at compile time to see if they exist

``` rust
bitrange! { 
    Test: u8,
    [aaa1_0bbb],
    a: first,
    b: second,
    c: third // this will panic with
             // Token 'c' is not found in pattern "aaa10bbb"
}
```

However, this does not work for unmapped fields

``` rust
bitrange! {
    Test: u8,
    [aaa1_0bbb],
    a: first,
    // b is not mapped
    // Does not give a warning
}
```