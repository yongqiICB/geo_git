use super::{line::Line, rect::Rect};

pub enum Shape {
    Rect(Rect),
    Line(Line),
    None,
}
