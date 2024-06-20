use crate::{
    db::version_controller::Action,
    geo::{color::Color, point::Point, rect::Rect},
    lexer::TokenKind,
};

use super::{
    r#impl::StringParser,
    tokens::{next_ident, next_literal, next_semicolon, peek_token},
};

pub fn next_action(parser: &mut StringParser) -> Action {
    let token = next_ident(parser).unwrap();
    let res = match token.to_ascii_uppercase().as_slice() {
        b"ADDRECT" => {
            let name = next_ident(parser).unwrap();
            let llx = next_literal(parser).unwrap();
            let lly = next_literal(parser).unwrap();
            let urx = next_literal(parser).unwrap();
            let ury = next_literal(parser).unwrap();

            let mut res = Action {
                action: crate::db::version_controller::ActionKind::Add,
                name: String::from_utf8(name.to_vec()).unwrap(),
                geo: Some(Rect {
                    ll: Point { x: llx, y: lly },
                    ur: Point { x: urx, y: ury },
                }),
                desc: None,
                color: None,
            };

            if let TokenKind::Semicolon = peek_token(parser).kind {
                next_semicolon(parser);
                return res;
            }

            let r = next_literal(parser).unwrap();
            let g = next_literal(parser).unwrap();
            let b = next_literal(parser).unwrap();
            res.color = Some(Color {
                r: r as u8,
                g: g as u8,
                b: b as u8,
            });
            res
        }

        b"UPDRECT" => {
            let name = next_ident(parser).unwrap();
            let mut res = Action {
                action: crate::db::version_controller::ActionKind::Modify,
                name: String::from_utf8(name.to_vec()).unwrap(),
                geo: None,
                desc: None,
                color: None,
            };

            let mut literals = vec![];
            while peek_token(parser).kind != TokenKind::Semicolon
                && peek_token(parser).kind != TokenKind::Eof
            {
                literals.push(next_literal(parser).unwrap());
            }
            literals.reverse();

            if literals.len() >= 4 {
                let llx = literals.pop().unwrap();
                let lly = literals.pop().unwrap();
                let urx = literals.pop().unwrap();
                let ury = literals.pop().unwrap();
                res.geo = Some(Rect {
                    ll: Point { x: llx, y: lly },
                    ur: Point { x: urx, y: ury },
                });
            }
            if literals.len() >= 3 {
                let r = literals.pop().unwrap() as u8;
                let g = literals.pop().unwrap() as u8;
                let b = literals.pop().unwrap() as u8;
                res.color = Some(Color { r, g, b });
            }
            res
        }
        b"DELRECT" => {
            let name = next_ident(parser).unwrap();
            Action {
                action: crate::db::version_controller::ActionKind::Delete,
                name: String::from_utf8(name.to_vec()).unwrap(),
                geo: None,
                desc: None,
                color: None,
            }
        }
        _ => unreachable!(),
    };
    next_semicolon(parser);
    res
}
