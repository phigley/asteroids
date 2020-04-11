use std::mem;
use wgpu::{
    BufferAddress, InputStepMode, VertexAttributeDescriptor, VertexBufferDescriptor, VertexFormat,
};
use zerocopy::AsBytes;

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Vertex {
    pub position: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self { position: [x, y] }
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
