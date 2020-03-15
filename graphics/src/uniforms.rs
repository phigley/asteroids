use nalgebra::{Matrix4, Orthographic3};
use winit::dpi::LogicalSize;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ViewUniforms {
    projection: Matrix4<f32>,
}

impl ViewUniforms {
    pub fn from_logical(logical_size: &LogicalSize<f64>) -> Self {
        let width = logical_size.width;
        let height = logical_size.height;

        let initial_projection: Orthographic3<f32> = if width >= height {
            let view_ratio = (width / height) as f32;
            Orthographic3::new(-view_ratio, view_ratio, -1.0, 1.0, -1.0, 1.0)
        } else {
            let view_ratio = (height / width) as f32;
            Orthographic3::new(-1.0, 1.0, -view_ratio, view_ratio, -1.0, 1.0)
        };

        let initial_projection_matrix: Matrix4<f32> = initial_projection.to_homogeneous();

        Self {
            projection: initial_projection_matrix.into(),
        }
    }

    pub fn layout_desc<'a>() -> wgpu::BindGroupLayoutDescriptor<'a> {
        wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            }],
        }
    }
}
