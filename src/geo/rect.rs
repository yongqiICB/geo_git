use super::point::Point;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Rect {
    pub ll: Point,
    pub ur: Point,
}

impl From<Rect> for eframe::egui::widgets::plot::PlotPoints {
    fn from(value: Rect) -> Self {
        Self::Owned(vec![value.ll.into(), value.ur.into()])
    }
}
