use super::color;
use super::shape;

use nalgebra::Similarity2;

#[derive(Debug, Clone)]
pub struct Model<'a> {
    pub shape: &'a shape::Shape,
    pub color: color::Color,
    pub transform: Similarity2<f32>,
}

impl<'a> Model<'a> {
    pub fn new(
        shape: &'a shape::Shape,
        color: color::Color,
        transform: Similarity2<f32>,
    ) -> Model<'a> {
        Model {
            shape,
            color,
            transform,
        }
    }
}
