#![deny(warnings)]
#![feature(proc_macro)]

extern crate proc_macro;

mod pattern;

use proc_macro::TokenStream;
use pattern::Pattern;

/// Create a mask based on a given format and a character
/// This will map all the bits that match the given character, to 1
/// All other bits will be set to 0
/// 
/// usage:  `proc_mask!([aaa0_1bbb], a);`
/// output: `0b1110_0000`
#[proc_macro]
pub fn proc_mask(input: TokenStream) -> TokenStream {
    let pattern = match Pattern::from_stream_with_selector(input) {
        Ok(p) => p,
        Err(e) => {
            println!("{}", e);
            println!("Usage: proc_mask!([aaa0_bbbb], a);");
            panic!();
        }
    };

    let total_length = pattern.pattern.len();
    let mut result = String::with_capacity(total_length + 2);
    result.push_str("0b");
    for i in 0..total_length {
        result.push_str(if pattern.current.contains(&i) {
            "1"
        } else {
            "0"
        });
    }

    result.parse().unwrap()
}

/// Return the offset of a given character in a format
/// This is the amount of least-significant bits in the proc_mask that are 0
/// 
/// usage:  `proc_offset([aaa0_1bbb], a);`
/// output: `5` (0b1110_0000 has 5 least-significant bits that are 0)
#[proc_macro]
pub fn proc_offset(input: TokenStream) -> TokenStream {
    let pattern = match Pattern::from_stream_with_selector(input) {
        Ok(p) => p,
        Err(e) => {
            println!("{}", e);
            println!("Usage: proc_offset!([aaa0_bbbb], a);");
            panic!();
        }
    };

    let total_length = pattern.pattern.len();
    let max = pattern.current.into_iter().max().unwrap_or_else(||total_length);
    format!("{}", total_length - max - 1).parse().unwrap()
}

/// Return the default mask of a format
/// This are all the fields that are set to either `0` or `1`
/// 
/// usage:  `proc_default_mask([aaa0_1bbb]);`
/// output: `0b0001_1000`
#[proc_macro]
pub fn proc_default_mask(input: TokenStream) -> TokenStream {
    let pattern = match Pattern::from_stream(input) {
        Ok(p) => p,
        Err(e) => {
            println!("{}", e);
            println!("Usage: proc_default_mask!([aaa0_bbbb]);");
            panic!();
        }
    };

    let mut result = String::with_capacity(pattern.pattern.len() + 2);
    result.push_str("0b");
    for c in pattern.pattern.chars() {
        result.push_str(if c == '1' || c == '0' {
            "1"
        } else {
            "0"
        });
    }
    result.parse().unwrap()
}

/// Returns the default value of a format
/// This is a value with 1 for every `1` in the format
/// 
/// usage:  `proc_default_value([aaa0_1bbb]);`
/// output: `0b0000_1000`
#[proc_macro]
pub fn proc_default_value(input: TokenStream) -> TokenStream {
    let pattern = match Pattern::from_stream(input) {
        Ok(p) => p,
        Err(e) => {
            println!("{}", e);
            println!("Usage: proc_default_value!([aaa0_bbbb]);");
            panic!();
        }
    };

    let mut result = String::with_capacity(pattern.pattern.len() + 2);
    result.push_str("0b");
    for c in pattern.pattern.chars() {
        result.push_str(if c == '1' {
            "1"
        } else {
            "0"
        });
    }
    result.parse().unwrap()
}
