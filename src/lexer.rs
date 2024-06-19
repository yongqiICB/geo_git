use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Cursor<'a> {
    len_remaining: usize,
    chars: Chars<'a>,
    pub len: usize,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
    pub len: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    Str,
    Literal { val: f64 },
    Semicolon,
    Eof,
}

impl Token {
    fn new(kind: TokenKind, len: u32) -> Token {
        Token { kind, len, start: 0,end: 0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralKind {
    Int,
    Float,
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub(crate) fn advance_token(&mut self) -> Token {
        self.eat_while(|x| x.is_whitespace());
        self.reset_pos_within_token();
        let start = self.len - self.len_remaining;
        let first_char = self.first();
        let token_kind = match first_char {
            _c @ '0'..='9' => {
                let val = self.number();
                TokenKind::Literal { val: val }
            }
            EOF_CHAR => {
                return Token::new(TokenKind::Eof, 0);
            }
            ';' => {
                self.bump();
                TokenKind::Semicolon
            }
            _ => {
                self.eat_identifier();
                TokenKind::Str
            }
        };
        let end = self.len - self.chars.as_str().len();
        let res = Token {
            kind: token_kind,
            len: self.pos_within_token(),
            start: start,
            end: end,
        };
        res
    }

    pub(crate) fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            len_remaining: input.len(),
            chars: input.chars(),
            len: input.len(),
        }
    }
    pub(crate) fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        Some(c)
    }

    pub(crate) fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub(crate) fn pos_within_token(&self) -> u32 {
        (self.len_remaining - self.chars.as_str().len()) as u32
    }

    /// Eats symbols while predicate returns true or until the end of file is reached.
    pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }

    pub(crate) fn eat_demical_digits(&mut self) -> Option<usize> {
        let mut res = None;
        loop {
            let x = self.first();
            match x {
                '0'..='9' => {
                    let p = x as usize - '0' as usize;
                    if res.is_none() {
                        res = Some(p);
                    } else {
                        res = Some(res.unwrap() * 10 + p);
                    }
                    self.bump();
                }
                _ => break,
            }
        }
        res
    }

    fn eat_identifier(&mut self) {
        self.bump();
        self.eat_while(|x| !x.is_whitespace());
    }

    pub(crate) fn number(&mut self) -> f64 {
        let mut res = self.eat_demical_digits().unwrap() as f64;
        match self.first() {
            '.' => {
                self.bump();
                let sub = self.eat_demical_digits().unwrap();
                let mut real_sub = sub as f64;
                while real_sub > 1.0 {
                    real_sub = real_sub / 10.0;
                }
                if sub == 0 {
                    res = res + real_sub;
                }
            }
            _ => {}
        };
        res
    }

    pub(crate) fn reset_pos_within_token(&mut self) {
        self.len_remaining = self.chars.as_str().len()
    }
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != TokenKind::Eof {
            Some(token)
        } else {
            None
        }
    })
}

pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

mod test {
    #[test]
    fn cursor() {
        use crate::lexer::Cursor;
        use crate::lexer::TokenKind;
        use std::{io::Read, path::PathBuf, str::FromStr};

        let prj_rt = project_root::get_project_root().unwrap();
        let mut file =
            std::fs::File::open(prj_rt.join(PathBuf::from_str("test/cursor.txt").unwrap()))
                .unwrap();
        let mut raw_text = String::with_capacity(256);
        file.read_to_string(&mut raw_text).unwrap();
        let oracle_tokens: Vec<_> = raw_text.split_whitespace().collect();
        let mut cursor = Cursor::new(&raw_text);
        let mut id = 0;
        loop {
            let token = cursor.advance_token();
            if token.kind == TokenKind::Eof {
                break;
            }
            assert!(id < oracle_tokens.len());
            assert!(token.len == oracle_tokens[id].len() as u32);

            id += 1;
        }
    }
}
