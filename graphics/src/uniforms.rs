use nalgebra::{Matrix4, Orthographic3};
use winit::dpi::PhysicalSize;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ViewUniforms {
    pub projection: Matrix4<f32>,
}

impl From<PhysicalSize<u32>> for ViewUniforms {
    fn from(size: PhysicalSize<u32>) -> Self {
        let width = size.width as f32;
        let height = size.height as f32;

        let initial_projection: Orthographic3<f32> = if width >= height {
            let view_ratio = (width / height) as f32;
            Orthographic3::new(-view_ratio, view_ratio, -1.0, 1.0, -1.0, 1.0)
        } else {
            let view_ratio = (height / width) as f32;
            Orthographic3::new(-1.0, 1.0, -view_ratio, view_ratio, -1.0, 1.0)
        };

        let initial_projection_matrix: Matrix4<f32> = initial_projection.to_homogeneous();

        Self {
            projection: initial_projection_matrix,
        }
    }
}
