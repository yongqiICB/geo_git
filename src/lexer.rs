use std::str::Chars;

#[derive(Debug)]
pub struct Cursor<'a> {
    len_remaining: usize,
    chars: Chars<'a>,
    prev: char,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Str,
    Literal { kind: LiteralKind },
    Eof,
}

impl Token {
    fn new(kind: TokenKind, len: u32) -> Token {
        Token { kind, len }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Int,
    Float,
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub(crate) fn advance_token(&mut self) -> Token {
        self.eat_while(|x| x.is_whitespace());
        self.reset_pos_within_token();
        let first_char = self.first();
        let token_kind = match first_char {
            _c @ '0'..='9' => {
                let literal_kind = self.number();
                TokenKind::Literal { kind: literal_kind }
            }
            EOF_CHAR => {
                return Token::new(TokenKind::Eof, 0);
            }
            _ => {
                self.eat_identifier();
                TokenKind::Str
            }
        };
        let res = Token {
            kind: token_kind,
            len: self.pos_within_token(),
        };
        self.reset_pos_within_token();
        res
    }

    pub(crate) fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            len_remaining: input.len(),
            chars: input.chars(),
            prev: EOF_CHAR,
        }
    }
    pub(crate) fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.prev = c;
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

    pub(crate) fn eat_demical_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.first() {
                '0'..='9' => {
                    has_digits = true;
                    self.bump();
                }
                _ => break,
            }
        }
        has_digits
    }

    fn eat_identifier(&mut self) {
        self.bump();
        self.eat_while(|x| !x.is_whitespace());
    }

    pub(crate) fn number(&mut self) -> LiteralKind {
        self.eat_demical_digits();
        match self.first() {
            '.' => {
                self.bump();
                self.eat_demical_digits();
                return LiteralKind::Float;
            }
            _ => return LiteralKind::Int,
        };
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
