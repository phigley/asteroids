use nalgebra::Matrix4;
use std::mem;
use wgpu::{BufferAddress, InputStepMode, VertexAttribute, VertexBufferLayout, VertexFormat};
use zerocopy::AsBytes;

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct ModelTransform {
    pub transform: [[f32; 4]; 4],
}

const FLOAT_SIZE: BufferAddress = std::mem::size_of::<f32>() as BufferAddress;

impl ModelTransform {
    pub fn new(transform_matrix: Matrix4<f32>) -> Self {
        Self {
            transform: transform_matrix.into(),
        }
    }

    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: mem::size_of::<ModelTransform>() as BufferAddress,
            step_mode: InputStepMode::Instance,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    format: VertexFormat::Float4,
                    shader_location: 2,
                },
                VertexAttribute {
                    offset: FLOAT_SIZE * 4,
                    format: VertexFormat::Float4,
                    shader_location: 3,
                },
                VertexAttribute {
                    offset: FLOAT_SIZE * 4 * 2,
                    format: VertexFormat::Float4,
                    shader_location: 4,
                },
                VertexAttribute {
                    offset: FLOAT_SIZE * 4 * 3,
                    format: VertexFormat::Float4,
                    shader_location: 5,
                },
            ],
        }
    }
}
