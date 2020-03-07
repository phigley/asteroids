use gfx;
use gfx_device_gl;
use gfx_window_glutin;
use glutin;

use glutin::dpi::{LogicalSize, PhysicalSize};

use gfx::traits::FactoryExt;
use gfx::Device;

use nalgebra::{Matrix4, Orthographic3};

use super::color;
use super::errors;
use super::shape;

use errors::ScreenCreateError;

pub struct GraphicDevice {
    window: glutin::WindowedContext<glutin::PossiblyCurrent>,
    encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
    device: gfx_device_gl::Device,
    data: super::pipe::Data<gfx_device_gl::Resources>,
    depth_format: gfx::handle::DepthStencilView<gfx_device_gl::Resources, super::DepthFormat>,
    pso: gfx::PipelineState<gfx_device_gl::Resources, super::pipe::Meta>,

    factory: gfx_device_gl::Factory,
}

impl GraphicDevice {
    pub fn create(
        width: f64,
        height: f64,
        title: &str,
        events_loop: &glutin::EventsLoop,
    ) -> Result<(GraphicDevice, LogicalSize, f64), errors::ScreenCreateError> {
        let logical_size = LogicalSize::new(width, height);

        let builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(logical_size);

        let (window, device, mut factory, main_color, main_depth) =
            gfx_window_glutin::init::<super::ColorFormat, super::DepthFormat>(
                builder,
                glutin::ContextBuilder::new(),
                &events_loop,
            )?;

        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        let pso = factory
            .create_pipeline_simple(
                include_bytes!("simple.vert"),
                include_bytes!("simple.frag"),
                super::pipe::new(),
            )
            .map_err(|err| ScreenCreateError::PipelineFailure {
                source: err,
                file_name: "simple",
            })?;

        let empty_vertex = [];
        let vbuf = factory.create_vertex_buffer(&empty_vertex);

        let data = super::pipe::Data {
            vbuf,
            view_uniforms: factory.create_constant_buffer(1),
            model_uniforms: factory.create_constant_buffer(1),
            out_color: main_color,
        };

        let actual_logical_size = match window.window().get_inner_size() {
            Some(logical_size) => logical_size,
            None => LogicalSize::new(width, height),
        };
        let actual_dpi_factor = window.window().get_hidpi_factor();

        let mut device = GraphicDevice {
            window,
            encoder,
            device,
            data,
            depth_format: main_depth,
            pso,

            factory,
        };

        device.update_projection(&actual_logical_size, actual_dpi_factor);

        Ok((device, actual_logical_size, actual_dpi_factor))
    }

    pub fn set_window_size(&mut self, logical_size: &LogicalSize, dpi_factor: f64) {
        self.update_projection(logical_size, dpi_factor);
        gfx_window_glutin::update_views(
            &self.window,
            &mut self.data.out_color,
            &mut self.depth_format,
        );
    }

    fn update_projection(&mut self, logical_size: &LogicalSize, dpi_factor: f64) {
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

        let initial_view_uniforms = super::ViewUniforms {
            projection: initial_projection_matrix.into(),
        };

        self.encoder
            .update_constant_buffer(&self.data.view_uniforms, &initial_view_uniforms);

        let physical_size = PhysicalSize::from_logical(*logical_size, dpi_factor);
        self.window.resize(physical_size);
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
