#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl From<Point> for egui_plot::PlotPoint {
    fn from(value: Point) -> Self {
        Self::new(value.x, value.y)
    }
}
