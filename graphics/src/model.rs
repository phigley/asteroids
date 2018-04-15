use super::color;
use super::shape;

use cgmath::Matrix4;

#[derive(Debug, Clone)]
pub struct Model<'a> {
    pub shape: &'a shape::Shape,
    pub color: color::Color,
    pub transform: Matrix4<f32>,
}

impl<'a> Model<'a> {
    pub fn new(shape: &'a shape::Shape, color: color::Color, transform: Matrix4<f32>) -> Model<'a> {
        Model {
            shape,
            color,
            transform,
        }
    }
}
