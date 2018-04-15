use gfx;
use gfx_device_gl;
use gfx_window_glutin;
use glutin;

use glutin::GlContext;

use gfx::traits::FactoryExt;
use gfx::Device;

use quick_error::ResultExt;

use cgmath::{Matrix4, Ortho};

use super::color;
use super::errors;
use super::shape;

pub struct GraphicDevice {
    window: glutin::GlWindow,
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    device: gfx_device_gl::Device,
    data: super::pipe::Data<gfx_device_gl::Resources>,
    depth_format: gfx::handle::DepthStencilView<gfx_device_gl::Resources, super::DepthFormat>,
    pso: gfx::PipelineState<gfx_device_gl::Resources, super::pipe::Meta>,

    factory: gfx_device_gl::Factory,
}

impl GraphicDevice {
    pub fn new(
        width: u32,
        height: u32,
        title: &str,
        events_loop: &glutin::EventsLoop,
    ) -> Result<GraphicDevice, errors::ScreenCreateError> {
        let builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);

        let (window, device, mut factory, main_color, main_depth) =
            gfx_window_glutin::init::<super::ColorFormat, super::DepthFormat>(
                builder,
                glutin::ContextBuilder::new(),
                &events_loop,
            );

        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        let pso = try!(
            factory
                .create_pipeline_simple(
                    include_bytes!("simple.vert"),
                    include_bytes!("simple.frag"),
                    super::pipe::new(),
                )
                .context("simple")
        );

        let empty_vertex = [];
        let vbuf = factory.create_vertex_buffer(&empty_vertex);

        let data = super::pipe::Data {
            vbuf,
            view_uniforms: factory.create_constant_buffer(1),
            model_uniforms: factory.create_constant_buffer(1),
            out_color: main_color,
        };

        let mut device = GraphicDevice {
            window,
            encoder,
            device,
            data,
            depth_format: main_depth,
            pso,

            factory,
        };

        device.update_projection(width, height);

        Ok(device)
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        self.update_projection(width, height);
        gfx_window_glutin::update_views(
            &self.window,
            &mut self.data.out_color,
            &mut self.depth_format,
        );
    }

    fn update_projection(&mut self, width: u32, height: u32) {
        let initial_projection: Ortho<f32> = if width >= height {
            let view_ratio = width as f32 / height as f32;
            Ortho {
                left: -view_ratio,
                right: view_ratio,
                top: 1.0,
                bottom: -1.0,
                near: -1.0,
                far: 1.0,
            }
        } else {
            let view_ratio = height as f32 / width as f32;
            Ortho {
                left: -1.0,
                right: 1.0,
                top: view_ratio,
                bottom: -view_ratio,
                near: -1.0,
                far: 1.0,
            }
        };

        let initial_projection_matrix: Matrix4<f32> = initial_projection.into();

        let initial_view_uniforms = super::ViewUniforms {
            projection: initial_projection_matrix.into(),
        };

        self.encoder
            .update_constant_buffer(&self.data.view_uniforms, &initial_view_uniforms);

        self.window.resize(width, height);
    }

    pub fn create_shape(&mut self, vertex_data: &[super::Vertex], indices: &[u16]) -> shape::Shape {
        shape::Shape::new(vertex_data, indices, &mut self.factory)
    }

    pub fn draw_shape(
        &mut self,
        transform: &Matrix4<f32>,
        color: color::Color,
        shape: &shape::Shape,
    ) {
        let locals = super::ModelUniforms {
            translation: (*transform).into(),
            color: color.into(),
        };
        self.encoder
            .update_constant_buffer(&self.data.model_uniforms, &locals);

        self.data.vbuf = shape.vbuf.clone();
        self.encoder.draw(&shape.slice, &self.pso, &self.data);
    }

    pub fn clear(&mut self, clear_color: color::Color) {
        self.encoder.clear(&self.data.out_color, clear_color.into());
    }

    pub fn flush(&mut self) {
        self.encoder.flush(&mut self.device);

        self.window.swap_buffers().unwrap();
        self.device.cleanup();
    }
}
