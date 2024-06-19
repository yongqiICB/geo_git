use egui_plot::{Arrows, PlotPoints};

use super::point::Point;

pub struct Vector {
    pub init: Point,
    pub term: Point,
}

impl From<Vector> for egui_plot::Arrows {
    fn from(value: Vector) -> Self {
        Arrows::new(
            PlotPoints::Owned(vec![value.init.into()]),
            PlotPoints::Owned(vec![value.term.into()]),
        )
    }
}
