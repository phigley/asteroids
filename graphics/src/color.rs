use std::mem;
use wgpu::{BufferAddress, InputStepMode, VertexAttribute, VertexBufferLayout, VertexFormat};
use zerocopy::AsBytes;

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub(crate) fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: mem::size_of::<Color>() as BufferAddress,
            step_mode: InputStepMode::Instance,
            attributes: &[VertexAttribute {
                offset: 0,
                shader_location: 1,
                format: VertexFormat::Float4,
            }],
        }
    }
}

impl From<Color> for [f32; 4] {
    fn from(color: Color) -> Self {
        [color.r, color.g, color.b, color.a]
    }
}
