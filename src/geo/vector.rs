use eframe::egui::plot::{Arrows, PlotPoints};

use super::point::Point;

pub struct Vector {
    pub init: Point,
    pub term: Point,
}

impl From<Vector> for eframe::egui::widgets::plot::Arrows {
    fn from(value: Vector) -> Self {
        Arrows::new(
            PlotPoints::Owned(vec![value.init.into()]),
            PlotPoints::Owned(vec![value.term.into()]),
        )
    }
}
