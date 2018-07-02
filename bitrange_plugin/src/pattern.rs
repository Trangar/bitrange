use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use syn;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Pattern {
    pub struct_name: String,
    pub size: String,
    pub trimmed_pattern: String,
    pub original_pattern: String,
    pub tokens: HashSet<char>,
}

#[derive(Default)]
struct Parsed {
    struct_name: String,
    original_pattern: String,
    trimmed_pattern: String,
    size: String,
}

impl Pattern {
    pub fn from_stream(stream: TokenStream) -> Result<Pattern, String> {
        let parsed = Pattern::parse(stream)?;
        let tokens = parsed.trimmed_pattern.chars().collect::<HashSet<_>>();
        Ok(Pattern {
            struct_name: parsed.struct_name,
            original_pattern: parsed.original_pattern,
            trimmed_pattern: parsed.trimmed_pattern,
            size: parsed.size,
            tokens,
        })
    }
    fn get_literal(iter: &mut ::proc_macro2::token_stream::IntoIter) -> Result<String, String> {
        match iter.next() {
            Some(TokenTree::Literal(lit)) => Ok(format!("{}", lit)),
            Some(TokenTree::Ident(ident)) => Ok(format!("{}", ident)),
            Some(TokenTree::Group(group)) => {
                let mut iter = group.stream().into_iter();
                Pattern::get_literal(&mut iter)
            }
            x => Err(format!("Expected literal, got {:?}", x)),
        }
    }
    fn parse(stream: TokenStream) -> Result<Parsed, String> {
        let ast: syn::DeriveInput = syn::parse(stream).unwrap();
        let mut parsed = Parsed::default();
        parsed.struct_name = format!("{}", ast.ident);
        for attr in ast.attrs {
            let ident = attr.path.segments.iter().map(|s| format!("{}", s.ident)).collect::<Vec<String>>().join("::");
            if ident == "BitrangeMask" {
                let mut iter = attr.tts.into_iter();
                match iter.next() {
                    Some(TokenTree::Punct(ref p)) if p.as_char() == '=' => {},
                    x => return Err(format!("Expected '#', got {:?}", x)),
                }
                let original_pattern = Pattern::get_literal(&mut iter)?;
                let original_pattern = original_pattern.trim_matches('"').trim().to_string();
                let trimmed_pattern = original_pattern.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
                parsed.original_pattern = original_pattern;
                parsed.trimmed_pattern = trimmed_pattern;
            } else if ident == "BitrangeSize" {
                let mut iter = attr.tts.into_iter();
                match iter.next() {
                    Some(TokenTree::Punct(ref p)) if p.as_char() == '=' => {},
                    x => return Err(format!("Expected '#', got {:?}", x)),
                }
                let size = Pattern::get_literal(&mut iter)?;
                parsed.size = size.trim_matches('"').to_string();
            }
        }
        if parsed.trimmed_pattern.is_empty() || parsed.original_pattern.is_empty() {
            Err("Missing attribute #[BitrangeMask = \"...\"]".to_string())
        } else if parsed.size.is_empty() {
            Err("Missing attribute #[BitrangeSize = \"...\"]".to_string())
        } else {
            Ok(parsed)
        }
    }

    pub fn get_token_mask(&self, token: char) -> String {
        let mut str = String::with_capacity(self.original_pattern.len() + 2);
        str += "0b";
        for c in self.original_pattern.chars() {
            str += if c == '_' {
                "_"
            } else if c == token {
                "1"
            } else {
                "0"
            };
        }
        str
    }

    pub fn get_token_offset(&self, token: char) -> usize {
        self.trimmed_pattern.chars().rev().take_while(|c| *c != token).count()
    }

    pub fn get_default_mask(&self) -> String {
        let mut str = String::with_capacity(self.original_pattern.len() + 2);
        str += "0b";
        for c in self.original_pattern.chars() {
            str += if c == '_' {
                "_"
            } else if c == '0' || c == '1' {
                "1"
            } else {
                "0"
            }
        }
        str
    }

    pub fn get_default_value(&self) -> String {
        let mut str = String::with_capacity(self.original_pattern.len() + 2);
        str += "0b";
        for c in self.original_pattern.chars() {
            str += if c == '_' {
                "_"
            } else if c == '1' {
                "1"
            } else {
                "0"
            }
        }
        str
    }
}
