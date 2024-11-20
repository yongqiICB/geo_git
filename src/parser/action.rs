use crate::{
    db::version_controller::Action,
    geo::{color::Color, line::Line, point::Point, rect::Rect},
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
                geo: crate::geo::shape::Shape::Rect(Rect {
                    ll: Point { x: llx, y: lly },
                    ur: Point { x: urx, y: ury },
                }),
                desc: None,
                color: None,
                gradient: None,
            };

            let mut numbers = [0.0, 0.0, 0.0];
            let mut cnt = 0;
            loop {
                let next_token = peek_token(parser);
                match next_token.kind {
                    TokenKind::Str | TokenKind::Eof => unimplemented!(),
                    TokenKind::Literal { val } => {
                        numbers[cnt] = val;
                        next_literal(parser);
                        cnt += 1;
                    }
                    TokenKind::Semicolon => {
                        break;
                    }
                }
            }
            match numbers.len() {
                1 => {
                    res.gradient = Some(numbers[0] as f32);
                }
                3 => {
                    let r = numbers[0];
                    let g = numbers[1];
                    let b = numbers[2];
                    res.color = Some(Color {
                        r: r as u8,
                        g: g as u8,
                        b: b as u8,
                        a: 0,
                    });
                }
                _ => unreachable!(),
            }
            res
        }

        b"UPDRECT" => {
            let name = next_ident(parser).unwrap();
            let mut res = Action {
                action: crate::db::version_controller::ActionKind::Modify,
                name: String::from_utf8(name.to_vec()).unwrap(),
                geo: crate::geo::shape::Shape::None,
                desc: None,
                color: None,
                gradient: None,
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
                res.geo = crate::geo::shape::Shape::Rect(Rect {
                    ll: Point { x: llx, y: lly },
                    ur: Point { x: urx, y: ury },
                });
            }
            if literals.len() >= 3 {
                let r = literals.pop().unwrap() as u8;
                let g = literals.pop().unwrap() as u8;
                let b = literals.pop().unwrap() as u8;
                res.color = Some(Color { r, g, b, a: 0 });
            }
            res
        }
        b"DELRECT" => {
            let name = next_ident(parser).unwrap();
            Action {
                action: crate::db::version_controller::ActionKind::Delete,
                name: String::from_utf8(name.to_vec()).unwrap(),
                geo: crate::geo::shape::Shape::None,
                desc: None,
                color: None,
                gradient: None,
            }
        }
        b"ADDLINE" => {
            let name = next_ident(parser).unwrap();
            let llx = next_literal(parser).unwrap();
            let lly = next_literal(parser).unwrap();
            let urx = next_literal(parser).unwrap();
            let ury = next_literal(parser).unwrap();
            Action {
                action: crate::db::version_controller::ActionKind::Add,
                name: String::from_utf8(name.to_vec()).unwrap(),
                geo: crate::geo::shape::Shape::Line(Line {
                    ll: Point { x: llx, y: lly },
                    ur: Point { x: urx, y: ury },
                }),
                desc: None,
                color: None,
                gradient: None,
            }
        }
        _ => unreachable!(),
    };
    next_semicolon(parser);
    res
}
