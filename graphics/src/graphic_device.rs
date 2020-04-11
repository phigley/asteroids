use nalgebra::Matrix4;
use std::sync::{Arc, Mutex, Weak};
use std::vec::Vec;
use wgpu;
use wgpu::{
    BindGroup, Buffer, BufferAddress, BufferUsage, Device, Queue, RenderPipeline, Surface,
    SwapChain, SwapChainDescriptor,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::color::Color;
use crate::errors::ScreenCreateError;
use crate::model_transform::ModelTransform;
use crate::shape::{Shape, ShapeData};
use crate::uniforms::ViewUniforms;
use crate::vertex::Vertex;

pub struct GraphicDevice {
    surface: Surface,
    device: Device,
    queue: Queue,
    sc_desc: SwapChainDescriptor,
    swap_chain: SwapChain,
    render_pipeline: RenderPipeline,

    view_uniform_buffer: Buffer,
    view_uniform_bind_group: BindGroup,

    shapes: Vec<Weak<Mutex<ShapeData>>>,
}

impl GraphicDevice {
    pub fn create(
        window: &Window,
    ) -> Result<(GraphicDevice, PhysicalSize<u32>, f64), ScreenCreateError> {
        let physical_size = window.inner_size();

        let surface = wgpu::Surface::create(window);
        let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
            ..Default::default()
        })
        .ok_or(ScreenCreateError::AdapterCreateFailure)?;

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: Default::default(),
        });

        let sc_desc = SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            // We should query for format.
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::Vsync,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let dpi_factor = window.scale_factor();

        let view_uniforms = ViewUniforms::from(physical_size);
        let view_uniform_buffer = device
            .create_buffer_mapped(1, BufferUsage::UNIFORM | BufferUsage::COPY_DST)
            .fill_from_slice(&[view_uniforms]);

        let view_uniform_bind_group_layout =
            device.create_bind_group_layout(&ViewUniforms::layout_desc());

        let view_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &view_uniform_bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &view_uniform_buffer,
                    range: 0..std::mem::size_of_val(&view_uniform_buffer) as BufferAddress,
                },
            }],
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&view_uniform_bind_group_layout],
            });

        let render_pipeline = {
            let vs_spirv: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/simple.vert.spv"));
            let fs_spirv: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/simple.frag.spv"));
            let vs_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv)).map_err(|err| {
                ScreenCreateError::PipelineFailure {
                    source: err,
                    file_name: "simple.vert",
                }
            })?;
            let fs_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv)).map_err(|err| {
                ScreenCreateError::PipelineFailure {
                    source: err,
                    file_name: "simple.frag",
                }
            })?;
            let vs_module = device.create_shader_module(&vs_data);
            let fs_module = device.create_shader_module(&fs_data);

            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                layout: &render_pipeline_layout,
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fs_module,
                    entry_point: "main",
                }),

                rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::Back,
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0,
                }),

                color_states: &[wgpu::ColorStateDescriptor {
                    format: sc_desc.format,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                depth_stencil_state: None,
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[Vertex::desc(), Color::desc(), ModelTransform::desc()],
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            })
        };

        let device = GraphicDevice {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            render_pipeline,

            view_uniform_buffer,
            view_uniform_bind_group,

            shapes: Vec::new(),
        };

        Ok((device, physical_size, dpi_factor))
    }

    pub fn set_window_size(&mut self, new_size: PhysicalSize<u32>) {
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);

        let view_uniforms = ViewUniforms::from(new_size);

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        let staging_buffer = self
            .device
            .create_buffer_mapped(1, BufferUsage::COPY_SRC)
            .fill_from_slice(&[view_uniforms]);

        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &self.view_uniform_buffer,
            0,
            std::mem::size_of::<ViewUniforms>() as BufferAddress,
        );

        self.queue.submit(&[encoder.finish()]);
    }

    pub fn create_shape(
        &mut self,
        vertex_data: &[Vertex],
        indices: &[u16],
        name: &'static str,
    ) -> Shape {
        let data = Arc::new(Mutex::new(ShapeData::new(
            &mut self.device,
            vertex_data,
            indices,
        )));

        self.shapes.push(Arc::downgrade(&data));

        Shape { data, name }
    }

    pub fn draw_shape(&mut self, transform: Matrix4<f32>, color: Color, shape: &Shape) {
        let mut shape_data = shape.data.lock().unwrap();

        // Add this draw request to our instances.
        shape_data
            .instance_transforms
            .push(ModelTransform::new(transform));
        shape_data.instance_colors.push(color);
    }

    pub fn render_frame(&mut self, clear_color: wgpu::Color) {
        self.shapes
            .retain(|shape_data| shape_data.strong_count() > 0);

        let frame = self.swap_chain.get_next_texture();

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color,
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.view_uniform_bind_group, &[]);

            for shapes_entry in &self.shapes {
                if let Some(shape_data_cell) = shapes_entry.upgrade() {
                    let mut shape_data = shape_data_cell.lock().unwrap();

                    let instance_transforms_buffer = self
                        .device
                        .create_buffer_mapped(
                            shape_data.instance_transforms.len(),
                            BufferUsage::VERTEX,
                        )
                        .fill_from_slice(&shape_data.instance_transforms);

                    let instance_colors_buffer = self
                        .device
                        .create_buffer_mapped(shape_data.instance_colors.len(), BufferUsage::VERTEX)
                        .fill_from_slice(&shape_data.instance_colors);

                    render_pass.set_vertex_buffers(
                        0,
                        &[
                            (&shape_data.vertex_buffer, 0),
                            (&instance_colors_buffer, 0),
                            (&instance_transforms_buffer, 0),
                        ],
                    );
                    render_pass.set_index_buffer(&shape_data.index_buffer, 0);

                    let num_instances = shape_data.instance_transforms.len() as u32;
                    render_pass.draw_indexed(0..shape_data.num_indices, 0, 0..num_instances);

                    shape_data.instance_transforms.clear();
                    shape_data.instance_colors.clear();
                }
            }
        }
        self.queue.submit(&[encoder.finish()]);
    }
}
