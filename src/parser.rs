use std::ops::RangeBounds;

use bytes::Bytes;

use crate::{db::version_controller::Commit, lexer::Cursor};

pub struct StringParser<'src> {
    /// Initial position, read-only.
    start_pos: u32,
    /// Source text to tokenize.
    src: &'src Bytes,
    /// Cursor to get lexer tokens.
    cursor: Cursor<'src>,
}

impl<'src> StringParser<'src> {
    pub fn new(src: &'src Bytes, cursor: Cursor<'src>) -> Self {
        Self {
            start_pos: 0,
            src,
            cursor,
        }
    }

    pub fn read_text(&self, rng: impl RangeBounds<usize>) -> bytes::Bytes {
        self.src.slice(rng)
    }
    pub fn next_commit(&mut self) -> Option<Commit> {
        let token = self.cursor.advance_token();
        match token.kind {
            crate::lexer::TokenKind::Eof => {
                return None;
            },
            crate::lexer::TokenKind::Str => {},
            crate::lexer::TokenKind::Literal { kind: _ } => unreachable!("Requires a \"commit\" statement. Found a literal instead."),
        };

        // FIXME: 
        // `u32` cast into usize.
        //  Environment below 32bit may not work.
        let len = token.len as usize;
        let end_pos = self.cursor.pos_within_token() as usize;
        let start_pos = end_pos - len;

        let token = self.read_text(start_pos..end_pos);

        // ENHANCE:
        // `to_ascii_uppercase()` slow.
        assert!(&token.to_ascii_uppercase() == b"COMMIT");

        loop {
            

        }

    }

    pub fn next_action() -> Option<>{

    }
}