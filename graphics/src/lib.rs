#[macro_use]
extern crate gfx;
#[macro_use]
extern crate quick_error;

extern crate cgmath;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;

pub mod events;
pub mod errors;
pub mod screen;
pub mod color;
pub mod shape;
pub mod model;

mod cursor;
mod graphic_device;
mod utils;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32;2] = "pos2D",
    }

    constant ViewUniforms {
        projection: [[f32;4];4] = "projection",
    }

    constant ModelUniforms {
        translation: [[f32;4];4] = "translation",
        color: [f32;4] = "color4D",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        view_uniforms: gfx::ConstantBuffer<ViewUniforms> = "ViewUniforms",
        model_uniforms: gfx::ConstantBuffer<ModelUniforms> = "ModelUniforms",
        out_color: gfx::RenderTarget<ColorFormat> = "colorOutput",
    }
}
