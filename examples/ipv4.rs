//! See https://tools.ietf.org/html/rfc791#section-3.1 for more information
//! 
//! The IPv4 protocol uses the following diagram:
//! 
//!     0                   1                   2                   3
//!     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//!    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!    |Version|  IHL  |Type of Service|          Total Length         |
//!    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!    |         Identification        |Flags|      Fragment Offset    |
//!    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!    |  Time to Live |    Protocol   |         Header Checksum       |
//!    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!    |                       Source Address                          |
//!    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!    |                    Destination Address                        |
//!    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!    |                    Options                    |    Padding    |
//!    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! 
//! We will split this into 6 different bitfields

#![feature(proc_macro)]
#[macro_use]
extern crate bitrange;

/// First bitrange: 
///     0                   1                   2                   3
///     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |Version|  IHL  |Type of Service|          Total Length         |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

bitrange! {
    Ipv4First: u32,
    [aaaa_bbbb_cccccccc_dddddddddddddddd],
    a: version,
    b: ihl,
    c: type_of_service,
    d: total_length
}

/// Second bitrange:
///     0                   1                   2                   3
///     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |         Identification        |Flags|      Fragment Offset    |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

bitrange! {
    Ipv4Second: u32,
    [aaaaaaaaaaaaaaaa_bbb_ccccccccccccc],
    a: identification,
    b: flags,
    c: fragment_offset
}

/// Third bitrange
///     0                   1                   2                   3
///     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |  Time to Live |    Protocol   |         Header Checksum       |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

bitrange! {
    Ipv4Third: u32,
    [aaaaaaaa_bbbbbbbb_cccccccccccccccc],
    a: time_to_live,
    b: protocol,
    c: header_checksum
}

/// We'll just use a [u8;4] for the fourth part
///     0                   1                   2                   3
///     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                       Source Address                          |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+


/// We'll just use a [u8;4] for the fifth part
///     0                   1                   2                   3
///     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                    Destination Address                        |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

/// The sixth bitrange
///     0                   1                   2                   3
///     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///    |                    Options                    |    Padding    |
///    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

bitrange! {
    Ipv4Sixth: u32,
    [aaaaaaaaaaaaaaaaaaaaaaaa_bbbbbbbb],
    a: options,
    b: padding
}

/// The final ipv4 package
pub struct Ipv4Header {
    first: Ipv4First,
    second: Ipv4Second,
    third: Ipv4Third,
    // fourth and fifth are the source and destination addresses
    fourth: [u8;4],
    fifth: [u8;4],
    sixth: Ipv4Sixth,
}

impl Ipv4Header {
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//    |Version|  IHL  |Type of Service|          Total Length         |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    pub fn version(&self) -> u8 {
        self.first.version() as u8
    }

    pub fn ihl(&self) -> u8 {
        self.first.ihl() as u8
    }

    pub fn type_of_service(&self) -> u8 {
        self.first.type_of_service() as u8
    }

    pub fn total_length(&self) -> u16 {
        self.first.total_length() as u16
    }
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//    |         Identification        |Flags|      Fragment Offset    |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    pub fn identification(&self) -> u16 {
        self.second.identification() as u16
    }

    pub fn flags(&self) -> u8 {
        self.second.flags() as u8
    }

    pub fn fragment_offset(&self) -> u16 {
        self.second.fragment_offset() as u16
    }
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//    |  Time to Live |    Protocol   |         Header Checksum       |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    pub fn time_to_live(&self) -> u8 {
        self.third.time_to_live() as u8
    }

    pub fn protocol(&self) -> u8 {
        self.third.protocol() as u8
    }

    pub fn header_checksum(&self) -> u16 {
        self.third.header_checksum() as u16
    }
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//    |                       Source Address                          |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    pub fn source_address(&self) -> [u8;4] {
        self.fourth
    }
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//    |                    Destination Address                        |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    pub fn destination_address(&self) -> [u8; 4] {
        self.fifth
    }
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//    |                    Options                    |    Padding    |
//    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    pub fn options(&self) -> u32 {
        self.sixth.options()
    }

    pub fn padding(&self) -> u8 {
        self.sixth.padding() as u8
    }
}

// DON'T USE THIS IN PRODUCTION
// Use the excellent byteorder crate instead:
// https://crates.io/crates/byteorder
fn to_u32(b: &[u8]) -> u32 {
    (b[0] as u32) | ((b[1] as u32) << 8) | ((b[2] as u32) << 16) | ((b[3] as u32) << 24)
}


impl<'a> From<&'a [u8]> for Ipv4Header {
    fn from(u: &[u8]) -> Ipv4Header {
        // This should ideally return an error type if it failed
        let mut chunks = u.chunks(4);
        let first = to_u32(chunks.next().unwrap());
        let second = to_u32(chunks.next().unwrap());
        let third = to_u32(chunks.next().unwrap());
        // DON'T USE THIS IN PRODUCTION
        // Use the excellent byteorder crate instead:
        // https://crates.io/crates/byteorder
        let fourth = { let mut v = [0u8; 4]; v.copy_from_slice(chunks.next().unwrap()); v };
        let fifth = { let mut v = [0u8; 4]; v.copy_from_slice(chunks.next().unwrap()); v };
        let sixth = to_u32(chunks.next().unwrap());

        Ipv4Header {
            first: Ipv4First::from(first).unwrap(),
            second: Ipv4Second::from(second).unwrap(),
            third: Ipv4Third::from(third).unwrap(),
            fourth: fourth,
            fifth: fifth,
            sixth: Ipv4Sixth::from(sixth).unwrap(),
        }
    }
}

fn main() {
    let bytes: [u8; 24] = [
        0x02, 0x5c, 0x44, 0xd8,
        0x00, 0x00, 0x80, 0x11,
        0x6e, 0x62, 0xc0, 0xa8,
        0x02, 0x02, 0xc0, 0xa8,
        0x02, 0x04, 0xe8, 0xa8,
        0x13, 0xc4, 0x02, 0x48
    ];

    let header = Ipv4Header::from(&bytes[..]);

    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    println!("|{:^7}|{:^7}|{:^15}|{:^31}|", header.version(), header.ihl(), header.type_of_service(), header.total_length());
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    println!("|{:^31}|{:^5}|{:^25}|", header.identification(), header.flags(), header.fragment_offset());
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    println!("|{:^15}|{:^15}|{:^31}|", header.time_to_live(), header.protocol(), header.header_checksum());
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    println!("|{:^63}|", format!("{:?}", header.source_address()));
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    println!("|{:^63}|", format!("{:?}", header.destination_address()));
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    println!("|{:^47}|{:^15}|", header.options(), header.padding());
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
}
