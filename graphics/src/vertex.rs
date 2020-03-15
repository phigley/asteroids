use nalgebra::Vector2;
use std::mem;
use wgpu::{
    BufferAddress, InputStepMode, VertexAttributeDescriptor, VertexBufferDescriptor, VertexFormat,
};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: Vector2<f32>,
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vector2::new(x, y),
        }
    }

    pub fn desc<'a>() -> VertexBufferDescriptor<'a> {
        VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as BufferAddress,
            step_mode: InputStepMode::Vertex,
            attributes: &[VertexAttributeDescriptor {
                offset: 0,
                shader_location: 0,
                format: VertexFormat::Float2,
            }],
        }
    }
}
