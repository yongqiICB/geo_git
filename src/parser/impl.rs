use std::ops::RangeBounds;

use bytes::Bytes;

use crate::lexer::Cursor;

pub struct StringParser<'src> {
    /// Initial position, read-only.
    pub start_pos: u32,
    /// Source text to tokenize.
    pub src: &'src Bytes,
    /// Cursor to get lexer tokens.
    pub cursor: Cursor<'src>,
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
}
