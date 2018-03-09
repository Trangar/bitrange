#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

#![allow(unused_variables, unused_imports)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
extern crate rustc_data_structures;

use syntax::parse::{token, token::Token, token::DelimToken};
use syntax::symbol::Ident;
use syntax::ast;
use syntax::codemap::respan;
use syntax::tokenstream::{Delimited, TokenTree, TokenStream, TokenStreamBuilder};
use syntax::ext::base::{ExtCtxt, MacEager, MacResult, DummyResult};
use syntax::ext::quote::rt::Span;
use syntax::ptr::P;
use rustc_plugin::Registry;
use rustc_data_structures::small_vec::SmallVec;
use std::collections::HashMap;

#[cfg(test)]
pub mod test;

#[plugin_registrar]
pub fn plugin_registar(reg: &mut Registry) {
    reg.register_macro("bitrange", bitrange);
}

fn parse(cx: &mut ExtCtxt, args: &[TokenTree]) -> Result<State, Box<MacResult + 'static>> {
    let mut state = State::default();
    for arg in args {
        match arg {
            &TokenTree::Delimited(sp, ref del) => {
                if let Expected::Format = state.expected {
                    match parse_format(&del) {
                        Ok(format) => {
                            state.format = format;
                            state.expected = Expected::Comma;
                        },
                        Err(e) => {
                            cx.span_err(sp, &e.to_string());
                            return Err(DummyResult::any(sp));
                        }
                    }
                } else {
                    cx.span_err(sp, &format!("Unexpected token, expected {:?}", state.expected.to_string()));
                    return Err(DummyResult::any(sp));
                }
            },
            &TokenTree::Token(sp, ref token) => {
                match *token {
                    token::Token::Comma => {
                        if let Expected::Comma = state.expected {
                            state.expected = Expected::VariantKey;
                        } else {
                            cx.span_err(sp, &format!("Unexpected comma, expected {:?}", state.expected.to_string()));
                            return Err(DummyResult::any(sp));
                        }
                    },
                    token::Token::Ident(ref ident) => {
                        if let Expected::StructName = state.expected {
                            let name = match parse_name(ident) {
                                Ok(name) => name,
                                Err(e) => {
                                    cx.span_err(sp, &e.to_string());
                                    return Err(DummyResult::any(sp));
                                }
                            };
                            state.name = name;
                            state.expected = Expected::Format;
                        } else if let Expected::VariantKey = state.expected {
                            let key = match parse_key(ident) {
                                Ok(key) => key,
                                Err(e) => {
                                    cx.span_err(sp, &e.to_string());
                                    return Err(DummyResult::any(sp));
                                }
                            };
                            state.expected = Expected::Colon(key);
                        } else if let Expected::VariantName(key) = state.expected {
                            let name = match parse_name(ident) {
                                Ok(name) => name,
                                Err(e) => {
                                    cx.span_err(sp, &e.to_string());
                                    return Err(DummyResult::any(sp));
                                }
                            };
                            state.methods.insert(key, name);
                            state.expected = Expected::Comma;
                        } else {
                            cx.span_err(sp, &format!("Unexpected text, expected {:?}", state.expected.to_string()));
                            return Err(DummyResult::any(sp));
                        }
                    },
                    token::Token::Colon => {
                        if let Expected::Colon(key) = state.expected {
                            state.expected = Expected::VariantName(key);
                        } else {
                            cx.span_err(sp, &format!("Unexpected colon, expected {:?}", state.expected.to_string()));
                            return Err(DummyResult::any(sp));
                        }
                    }
                    _ => {
                        cx.span_err(sp, &format!("Unexpected token, expected {:?}", state.expected.to_string()));
                        return Err(DummyResult::any(sp));
                    }
                }
            },
        }
    }
    Ok(state)
}

pub fn generate_struct(cx: &mut ExtCtxt, sp: Span, state: &State) -> P<ast::Item> {
    let builder = TokenStreamBuilder::new();
        // .add(Token::Ident(Ident::from_str("Derive")));
    let item = ast::Item {
        ident: Ident::from_str(&state.name),
        attrs: vec![
            ast::Attribute {
                id: ast::AttrId(1),
                style: ast::AttrStyle::Outer,
                path: ast::Path::from_ident(sp, Ident::from_str("derive")),
                tokens: builder.add(Token::OpenDelim(DelimToken::Paren))
                               .add(Token::Ident(Ident::from_str("Debug")))
                               .add(Token::CloseDelim(DelimToken::Paren))
                               .build(),
                is_sugared_doc: false,
                span: sp.clone(),
            }
        ],
        id: ast::DUMMY_NODE_ID,
        node: ast::ItemKind::Struct(
            ast::VariantData::Struct(vec![
                ast::StructField {
                    span: sp,
                    ident: Some(Ident::from_str("bits")),
                    vis: respan(sp, ast::VisibilityKind::Public),
                    id: ast::DUMMY_NODE_ID,
                    ty: P(ast::Ty {
                        id: ast::DUMMY_NODE_ID,
                        node: ast::TyKind::Path(
                            None,
                            ast::Path::from_ident(sp, Ident::from_str("u8"))
                        ),
                        span: sp,
                    }),
                    attrs: Vec::new(),
                }
            ], ast::DUMMY_NODE_ID),
            Default::default()
        ),
        vis: respan(sp, ast::VisibilityKind::Public),
        span: sp.clone(),
        tokens: None,
    };

    P(item)
}

pub fn generate_impl(cx: &mut ExtCtxt, sp: Span, state: &State) -> P<ast::Item> {
    let mut items = vec![
    ];
    for (key, name) in state.methods.iter() {
        items.push(ast::ImplItem {
            id: ast::DUMMY_NODE_ID,
            ident: Ident::from_str(name),
            vis: respan(sp, ast::VisibilityKind::Public),
            defaultness: ast::Defaultness::Final,
            attrs: Vec::new(),
            generics: ast::Generics::default(),
            span: sp,
            tokens: None, 
            node: ast::ImplItemKind::Method(
                ast::MethodSig {
                    unsafety: ast::Unsafety::Normal,
                    constness: respan(sp, ast::Constness::NotConst),
                    abi: syntax::abi::Abi::Rust,
                    decl: P(ast::FnDecl {
                        inputs: vec![
                            ast::Arg {
                                id: ast::DUMMY_NODE_ID,
                                ty: P(ast::Ty {
                                    id: ast::DUMMY_NODE_ID,
                                    node: ast::TyKind::Rptr(
                                        None,
                                        ast::MutTy {
                                            ty: P(ast::Ty {
                                                id: ast::DUMMY_NODE_ID,
                                                node: ast::TyKind::ImplicitSelf,
                                                span: sp
                                            }),
                                            mutbl: ast::Mutability::Immutable
                                        }
                                    ),
                                    span: sp
                                }),
                                pat: P(ast::Pat {
                                    id: ast::DUMMY_NODE_ID,
                                    node: ast::PatKind::Ident(
                                        ast::BindingMode::ByValue(ast::Mutability::Immutable),
                                        respan(sp, ast::Ident::from_str("self")),
                                        None
                                    ),
                                    span: sp,
                                })
                            }
                        ],
                        output: ast::FunctionRetTy::Ty(P(ast::Ty {
                            id: ast::DUMMY_NODE_ID,
                            node: ast::TyKind::Path(
                                None,
                                ast::Path::from_ident(sp, Ident::from_str("u8"))
                            ),
                            span: sp,
                        })),
                        variadic: false,
                    })
                },
                P(ast::Block {
                    stmts: vec![
                        ast::Stmt {
                            id: ast::DUMMY_NODE_ID,
                            node: ast::StmtKind::Expr(P(ast::Expr {
                                id: ast::DUMMY_NODE_ID,
                                node: ast::ExprKind::Lit(P(respan(sp, ast::LitKind::Byte(0)))),
                                span: sp,
                                attrs: syntax::util::ThinVec::new(),
                            })),
                            span: sp,
                        }
                    ],
                    id: ast::DUMMY_NODE_ID,
                    rules: ast::BlockCheckMode::Default,
                    span: sp,
                    recovered: false,
                })
            )
        });
    }
    let struct_impl = ast::ItemKind::Impl(
        ast::Unsafety::Normal,
        ast::ImplPolarity::Positive,
        ast::Defaultness::Final,
        ast::Generics::default(),
        None,
        P(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            node: ast::TyKind::Path(None, ast::Path::from_ident(sp, Ident::from_str(&state.name))),
            span: sp
        }),
        items
    );
    let item = ast::Item {
        ident: Ident::from_str(""),
        attrs: Vec::new(),
        id: ast::DUMMY_NODE_ID,
        node: struct_impl,
        vis: respan(sp, ast::VisibilityKind::Inherited),
        span: sp.clone(),
        tokens: None,
    };
    P(item)
}

pub fn generate(cx: &mut ExtCtxt, sp: Span, state: State) -> Box<MacResult + 'static> {
    let mut it = Vec::with_capacity(2);
    it.push(generate_struct(cx, sp, &state));
    it.push(generate_impl(cx, sp, &state));
    MacEager::items(SmallVec::many(it.into_iter()))
}

pub fn bitrange(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    let state = match parse(cx, args) {
        Ok(state) => state,
        Err(e) => return e,
    };

    generate(cx, sp, state)
}

pub fn parse_format(delim: &Delimited) -> Result<Vec<char>, ParseError> {
    let stream: TokenStream = delim.tts.clone().into();
    let mut cursor = stream.into_trees();
    if let Some(TokenTree::Token(_, token::Token::Ident(ident))) = cursor.next() {
        let name: &str = &ident.name.as_str();
        Ok(name.chars().collect())
    } else {
        Err(ParseError::InvalidFormat)
    }
}

pub fn parse_key(ident: &Ident) -> Result<char, ParseError> {
    let name: &str = &ident.name.as_str();
    if name.len() != 1 {
        Err(ParseError::TokenNotASingleCharacter)
    } else {
        Ok(name.chars().next().unwrap())
    }
}

pub fn parse_name(ident: &Ident) -> Result<String, ParseError> {
    Ok((&ident.name.as_str()).to_string())
}

pub enum ParseError {
    NotImplemented,
    InvalidFormat,
    TokenNotASingleCharacter,
}

impl ParseError {
    pub fn to_string(&self) -> String {
        match *self {
            ParseError::InvalidFormat => "Invalid bitrange format, expected [aaa_bbbb] as a first token".to_string(),
            ParseError::TokenNotASingleCharacter => "Token needs to be a single char, expected 'a: name'".to_string(),
            ParseError::NotImplemented => "Not implemented".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub name: String,
    pub format: Vec<char>,
    pub methods: HashMap<char, String>,
    pub expected: Expected,
}

impl Default for State {
    fn default() -> State {
        State {
            name: String::new(),
            format: Vec::new(),
            methods: HashMap::new(),
            expected: Expected::StructName
        }
    }
}

#[derive(Debug)]
pub enum Expected {
    StructName,
    Format,
    VariantKey,
    Colon(char),
    VariantName(char),
    Comma,
}

impl Expected {
    pub fn to_string(&self) -> String {
        match *self {
            Expected::StructName => "the name of the struct, e.g. 'MappedInt'".to_owned(),
            Expected::Format => "a format, e.g. [aaa_bbbb]".to_owned(),
            Expected::VariantKey => "a key, e.g. 'a:'".to_owned(),
            Expected::Colon(_) => "a colon".to_owned(),
            Expected::VariantName(_) => "a name for the key, e.g. 'a: First'".to_owned(),
            Expected::Comma => "a comma".to_owned(),
        }
    }
}