use gfx;
use gfx_device_gl;
use gfx::traits::FactoryExt;

#[derive(Debug, Clone)]
pub struct Shape {
    pub vbuf: gfx::handle::Buffer<gfx_device_gl::Resources, super::Vertex>,
    pub slice: gfx::Slice<gfx_device_gl::Resources>,
}

impl Shape {
    pub fn new(
        vertex_data: &[super::Vertex],
        indices: &[u16],
        factory: &mut gfx_device_gl::Factory,
    ) -> Shape {
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertex_data, indices);

        Shape {
            vbuf: vbuf,
            slice: slice,
        }
    }
}
