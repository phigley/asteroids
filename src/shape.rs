use cgmath::Point2;
use cgmath::prelude::*;

use specs::VecStorage;

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Shape {
    pub verts: Vec<Point2<f32>>,
    pub indices: Vec<u16>,
    pub radius: f32,
}

impl Shape {
    pub fn new(verts: Vec<Point2<f32>>, indices: Vec<u16>) -> Self {
        let origin = Point2::origin();
        let mut radius = 0.0;

        for v in &verts {
            let dist = v.distance(origin);

            if dist > radius {
                radius = dist;
            }
        }

        Shape {
            verts,
            indices,
            radius,
        }
    }

    pub fn create_ship() -> Self {
        let scale = 0.025;

        let verts = vec![
            Point2::new(0.0, 1.0 * scale),
            Point2::new(1.0 * scale, -1.0 * scale),
            Point2::new(0.0, -0.5 * scale),
            Point2::new(-1.0 * scale, -1.0 * scale),
        ];
        let indices = vec![0, 1, 2, 0, 2, 3];

        Shape::new(verts, indices)
    }
}
