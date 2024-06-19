use std::ops::RangeBounds;

use bytes::Bytes;

use crate::{db::version_controller::Commit, lexer::{Cursor, TokenKind}};

use super::{rect::next_action, tokens::{next_ident, next_token, peek_token}};

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

    pub fn parse(mut self) -> Vec<Commit> {
        let mut res = vec![];        
        loop {
            let c = next_token(&mut self);

            if c.kind == TokenKind::Eof {
                break;
            }

            let mut commit = Commit::new();
            'in_commit: loop {
                let c = peek_token(&mut self);

                if c.kind == TokenKind::Str {
                    let txt = self.read_text(c.start..c.end);
                    match txt.to_ascii_uppercase().as_slice() {
                        b"COMMIT" => break 'in_commit,
                        _ => {},
                    }
                } else if c.kind == TokenKind::Eof {
                    break;
                }

                let action = next_action(&mut self);
                commit.add_action(action);
            }
            res.push(commit);
        }
        res
    }
}
