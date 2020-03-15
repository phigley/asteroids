use crate::color::Color;
use crate::shape::Shape;

use nalgebra::Similarity2;

#[derive(Debug, Clone)]
pub struct Model {
    pub shape: Shape,
    pub color: Color,
    pub transform: Similarity2<f32>,
}

impl Model {
    pub fn new(shape: Shape, color: Color, transform: Similarity2<f32>) -> Self {
        Self {
            shape,
            color,
            transform,
        }
    }
}
