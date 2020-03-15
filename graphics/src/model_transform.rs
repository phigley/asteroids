use nalgebra::Matrix4;
use std::mem;
use wgpu::{
    BufferAddress, InputStepMode, VertexAttributeDescriptor, VertexBufferDescriptor, VertexFormat,
};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ModelTransform {
    pub transform: Matrix4<f32>,
}

const FLOAT_SIZE: BufferAddress = std::mem::size_of::<f32>() as BufferAddress;

impl ModelTransform {
    pub fn new(transform: Matrix4<f32>) -> Self {
        Self { transform }
    }

    pub fn desc<'a>() -> VertexBufferDescriptor<'a> {
        VertexBufferDescriptor {
            stride: mem::size_of::<ModelTransform>() as BufferAddress,
            step_mode: InputStepMode::Instance,
            attributes: &[
                VertexAttributeDescriptor {
                    offset: 0,
                    format: VertexFormat::Float4,
                    shader_location: 2,
                },
                VertexAttributeDescriptor {
                    offset: FLOAT_SIZE * 4,
                    format: VertexFormat::Float4,
                    shader_location: 3,
                },
                VertexAttributeDescriptor {
                    offset: FLOAT_SIZE * 4 * 2,
                    format: VertexFormat::Float4,
                    shader_location: 4,
                },
                VertexAttributeDescriptor {
                    offset: FLOAT_SIZE * 4 * 3,
                    format: VertexFormat::Float4,
                    shader_location: 5,
                },
            ],
        }
    }
}
