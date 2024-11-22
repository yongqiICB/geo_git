use std::{
    f32,
    ops::{Div, Sub},
};

use colorgrad::Gradient;

use super::color::Color;

/** Generate the color of a number,
which is based on it's position under a range of the minimum and the maximum value of a number sequence.
*/
pub struct ColorGenerator<V> {
    min: V,
    max: V,
    grad: Box<dyn Gradient>,
}

impl<V> ColorGenerator<V>
where
    V: Ord,
    V: Sub<Output = V>,
    V: Div<Output = V>,
    V: Copy,
    f32: From<V>,
{
    pub fn build(iter: impl Iterator<Item = V> + Clone, grad: Box<dyn Gradient>) -> Self {
        let max = iter.clone().max().unwrap();
        let min = iter.clone().min().unwrap();
        Self { max, min, grad }
    }
    pub fn get_color(&self, value: V) -> Color
    where
        V: Sub<Output = V>,
        V: Div<Output = V>,
        V: Copy,
        f32: From<V>,
    {
        let v = Criticality(value);
        v.color(self.min, self.max, &self.grad)
    }
}

pub struct Criticality<V>(pub V);

impl<V> Criticality<V>
where
    V: Sub<Output = V>,
    V: Div<Output = V>,
    V: Copy,
    f32: From<V>,
{
    pub fn color(&self, min: V, max: V, colorgrad: &Box<dyn colorgrad::Gradient>) -> Color {
        let uniformed_criticality = (self.0 - min) / (max - min);
        let color = colorgrad.at(f32::from(uniformed_criticality));
        (&color).into()
    }
}
