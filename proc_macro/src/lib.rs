#![feature(proc_macro)]

extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree, Delimiter};
use std::collections::HashMap;

struct Pattern {
    pattern: String,
    current: Vec<usize>,
}

impl Pattern {
    pub fn from_stream_with_selector(stream: TokenStream) -> Result<Pattern, String> {
        let mut iter = stream.into_iter();
        let possible_pattern = iter.next();
        let pattern = parse_pattern(possible_pattern)?;
        consume_comma(iter.next())?;
        let selector = parse_selector(iter.next())?;
        if let Some(x) = iter.next() {
            return Err(format!("Unexpected trailing token: {:?}", x));
        }

        let mut map = HashMap::<char, Vec<usize>>::default();
        for (index, c) in pattern.chars().enumerate() {
            let entry = map.entry(c).or_insert_with(|| Vec::new());
            entry.push(index);
        }

        let current = match map.get(&selector) {
            Some(vec) => vec.clone(),
            None => {
                return Err(format!("Token {:?} is not found in pattern {:?}", selector, pattern));
            }
        };

        Ok(Pattern {
            pattern,
            current,
        })
    }

    pub fn from_stream(stream: TokenStream) -> Result<Pattern, String> {
        let mut iter = stream.into_iter();
        let possible_pattern = iter.next();
        let pattern = parse_pattern(possible_pattern)?;
        Ok(Pattern {
            pattern,
            current: Vec::new()
        })
    }
}

fn parse_pattern(item: Option<TokenTree>) -> Result<String, String> {
    match item {
        Some(TokenTree::Group(mut group)) => {
            loop {
                match group.delimiter() {
                    Delimiter::Bracket | Delimiter::None => {},
                    _ => return Err(format!("Expected bracket, got {:?}", group))
                }
                let mut inner_iter = group.stream().into_iter();
                match inner_iter.next() {
                    Some(TokenTree::Term(term)) => {
                        let mut str = term.as_str().to_string();
                        str.retain(|c| !c.is_whitespace() && c != '_');
                        return Ok(str)
                    },
                    Some(TokenTree::Group(g)) => {
                        group = g;
                    },

                    x => return Err(format!("Expected TokenTree::Term, got {:?}", x))
                }
            }
        },
        x => {
            Err(format!("Expected pattern, got {:?}", x))
        }
    }
}

fn consume_comma(item: Option<TokenTree>) -> Result<(), String> {
    if let Some(TokenTree::Op(op)) = item {
        if op.op() == ',' {
            Ok(())
        } else {
            Err(format!("Expected comma, got {:?}", op.op()))
        }
    } else {
        Err(format!("Expected TokenTree::Op, got {:?}", item))
    }
}

fn parse_selector(item: Option<TokenTree>) -> Result<char, String> {
    if let Some(TokenTree::Term(term)) = item {
        let str = term.as_str();
        if str.len() == 1 {
            Ok(str.chars().next().unwrap())
        } else {
            Err(format!("Expected a single (ascii) character, got {:?}", str))
        }
    } else if let Some(TokenTree::Group(group)) = item {
        parse_selector(group.stream().into_iter().next())
    } else {
        Err(format!("Not implemented: {:?}", item))
    }
}

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