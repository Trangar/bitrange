#![deny(warnings)]
#![feature(proc_macro)]

extern crate proc_macro;

mod pattern;

use proc_macro::TokenStream;
use pattern::Pattern;

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