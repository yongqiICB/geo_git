use super::point::Point;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Line {
    pub ll: Point,
    pub ur: Point,
}

impl From<Line> for egui_plot::PlotPoints {
    fn from(value: Line) -> Self {
        Self::Owned(vec![value.ll.into(), value.ur.into()])
    }
}
