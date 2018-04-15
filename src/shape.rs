use std::f32;

use cgmath::prelude::*;
use cgmath::Point2;

use rand::distributions::{IndependentSample, Range};
use rand::Rng;

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

    pub fn create_asteroid<R: Rng>(mut rng: &mut R) -> Self {
        let scale = 0.08;

        let num_points: u16 = 8;

        let mut verts = Vec::with_capacity(num_points as usize);

        let noise = Noise::new(scale, 0.2, 0.25, num_points);

        noise.apply(&mut verts, &mut rng);
        assert_eq!(verts.len(), num_points as usize);

        let num_indices = (3 * (num_points - 2)) as usize;
        let mut indices = Vec::with_capacity(num_indices);

        for i in 1..(num_points - 1) {
            indices.push(0);
            indices.push(i);
            indices.push(i + 1);
        }

        assert_eq!(indices.len(), num_indices);

        Shape::new(verts, indices)
    }
}

struct Noise {
    base_radius: f32,
    radius_var: Range<f32>,
    angle_delta: f32,
    angle_var: Range<f32>,
    num_points: u16,
}

impl Noise {
    fn new(max_radius: f32, max_radial_var: f32, max_angle_var: f32, num_points: u16) -> Self {
        let base_radius = max_radius * (1.0 - max_radial_var);
        let radius_var = Range::new(0.0, max_radius * max_radial_var);

        let angle_delta = 2.0 * f32::consts::PI / f32::from(num_points);

        let angle_var = Range::new(0.0, max_angle_var * angle_delta);

        Noise {
            base_radius,
            radius_var,
            angle_delta,
            angle_var,
            num_points,
        }
    }

    fn apply<R: Rng>(&self, verts: &mut Vec<Point2<f32>>, mut rng: &mut R) {
        for i in 0..self.num_points {
            let angle_delta = self.angle_var.ind_sample(&mut rng);
            let angle = f32::from(i) * self.angle_delta + angle_delta;

            let radius_delta = self.radius_var.ind_sample(&mut rng);
            let radius = self.base_radius + radius_delta;

            let x = radius * angle.cos();
            let y = radius * angle.sin();

            verts.push(Point2::new(x, y));
        }
    }
}
