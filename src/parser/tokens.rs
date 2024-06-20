use bytes::Bytes;

use crate::lexer::Token;

use super::r#impl::StringParser;

pub fn next_token(parser: &mut StringParser) -> Token {
    parser.cursor.advance_token()
}
pub fn next_ident(parser: &mut StringParser) -> Option<Bytes> {
    let token = parser.cursor.advance_token();
    match token.kind {
        crate::lexer::TokenKind::Eof => {
            return None;
        },
        crate::lexer::TokenKind::Str => {},
        crate::lexer::TokenKind::Literal { val: _ } => unreachable!("Requires a ident. Found a literal instead."),
        crate::lexer::TokenKind::Semicolon => unreachable!("Requires a ident. Found a `;` instead."),
    };
    Some(parser.read_text(token.start..token.end))
}

pub fn next_literal(parser: &mut StringParser) -> Option<f64> {
    let token = parser.cursor.advance_token();
    match token.kind {
        crate::lexer::TokenKind::Eof => {
            return None;
        },
        crate::lexer::TokenKind::Str => unreachable!("Requires a literal. Found a string instead."),
        crate::lexer::TokenKind::Literal { val: _ } => {},
        crate::lexer::TokenKind::Semicolon => unreachable!("Requires a literal. Found a `;` instead."),
    };
    let texted = parser.read_text(token.start..token.end);
    let f64 = String::from_utf8(texted.to_vec()).unwrap().parse::<f64>().unwrap();
    // FIXME:
    // `bytes` to usize
    Some(f64)
}

pub fn next_semicolon(parser: &mut StringParser) {
    let token = parser.cursor.advance_token();
    match token.kind {
        crate::lexer::TokenKind::Eof => (),
        crate::lexer::TokenKind::Str => unreachable!("Requires a `;`. Found a string instead."),
        crate::lexer::TokenKind::Literal { val: _ } => unreachable!("Requires a `;`. Found a literal instead."),
        crate::lexer::TokenKind::Semicolon => (),
    }
}

pub fn peek_token(parser: &StringParser) -> Token {
    let mut cursor = parser.cursor.clone();
    cursor.advance_token()
}

#[test]
fn testf64() {
    use std::str::FromStr;
    let str = String::from_str("1.01").unwrap();
    let a = str.parse::<f64>().unwrap();
    let str = String::from_str("1").unwrap();
    let b = str.parse::<f64>().unwrap();
    println!("{}\n{}",a,b);
}