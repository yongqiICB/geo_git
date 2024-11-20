use std::u8;

use colorgrad::{self, Gradient};

#[derive(Default)]
pub enum ColorType {
    #[default]
    RGB,
    Gradient {
        generator: Box<dyn colorgrad::Gradient>,
        min: f32,
        max: f32,
    },
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn from_grad(gradient: f32, color_grad: &colorgrad::BasisGradient) -> Self {
        let res = color_grad.at(gradient.clamp(0.0, 1.0));
        (&res).into()
    }
}

impl From<&colorgrad::Color> for Color {
    fn from(value: &colorgrad::Color) -> Self {
        Self {
            r: (value.r * u8::MAX as f32) as u8,
            g: (value.g * u8::MAX as f32) as u8,
            b: (value.b * u8::MAX as f32) as u8,
            a: (value.a * u8::MAX as f32) as u8,
        }
    }
}
