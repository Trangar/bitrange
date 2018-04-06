use proc_macro::{TokenStream, TokenTree, Delimiter};
use std::collections::HashMap;

pub struct Pattern {
    pub pattern: String,
    pub current: Vec<usize>,
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
                    Some(TokenTree::Literal(lit)) => {
                        let mut str = format!("{}", lit);
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
