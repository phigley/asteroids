use crate::color::Color;
use crate::errors::ScreenCreateError;
use crate::model_transform::ModelTransform;
use crate::shape::{Shape, ShapeData};
use crate::uniforms::ViewUniforms;
use crate::vertex::Vertex;
use nalgebra::Matrix4;
use std::sync::{Arc, Mutex, MutexGuard, Weak};
use std::vec::Vec;
use wgpu::{util::DeviceExt, IndexFormat};
use wgpu::{
    BindGroup, Buffer, BufferAddress, BufferUsage, Device, Queue, RenderPipeline, Surface,
    SwapChain, SwapChainDescriptor,
};
use winit::{dpi::PhysicalSize, window::Window};
use zerocopy::AsBytes;

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
    pub async fn create(
        window: &Window,
    ) -> Result<(GraphicDevice, PhysicalSize<u32>, f64), ScreenCreateError> {
        let physical_size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or(ScreenCreateError::AdapterCreateFailure)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Main Device"),
                    features: wgpu::Features::empty(),
                    limits: Default::default(),
                },
                None,
            )
            .await
            .map_err(ScreenCreateError::DeviceCreateFailure)?;

        let sc_desc = SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            // We should query for format.
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let dpi_factor = window.scale_factor();

        let view_uniforms = ViewUniforms::from(physical_size);
        let view_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("view_uniforms"),
            contents: view_uniforms.projection.as_slice().as_bytes(),
            usage: BufferUsage::UNIFORM | BufferUsage::COPY_DST,
        });

        let view_uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ViewUniforms"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                }],
            });

        let view_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &view_uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: view_uniform_buffer.as_entire_binding(),
            }],
            label: Some("ViewUniforms"),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render_pipepline_layout"),
                bind_group_layouts: &[&view_uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = {
            let vs_spirv = wgpu::include_spirv!(concat!(env!("OUT_DIR"), "/simple.vert.spv"));
            let fs_spirv = wgpu::include_spirv!(concat!(env!("OUT_DIR"), "/simple.frag.spv"));
            // let vs_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv)).map_err(|err| {
            //     ScreenCreateError::PipelineFailure {
            //         source: err,
            //         file_name: "simple.vert",
            //     }
            // })?;
            // let fs_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv)).map_err(|err| {
            //     ScreenCreateError::PipelineFailure {
            //         source: err,
            //         file_name: "simple.frag",
            //     }
            // })?;
            let vs_module = device.create_shader_module(&vs_spirv);
            let fs_module = device.create_shader_module(&fs_spirv);

            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("MainRenderPipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vs_module,
                    entry_point: "main",
                    buffers: &[Vertex::desc(), Color::desc(), ModelTransform::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &fs_module,
                    entry_point: "main",
                    targets: &[sc_desc.format.into()],
                    // targets: &[wgpu::ColorTargetState {
                    //     format: sc_desc.format,
                    //     color_blend: wgpu::BlendState::REPLACE,
                    //     alpha_blend: wgpu::BlendState::REPLACE,
                    //     write_mask: wgpu::ColorWrite::ALL,
                    // }],
                }),

                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::Back,
                    ..Default::default()
                },

                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
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
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("SetWindowSize"),
            });

        let staging_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("staging_buffer"),
                contents: view_uniforms.projection.as_slice().as_bytes(),
                usage: BufferUsage::COPY_SRC,
            });

        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &self.view_uniform_buffer,
            0,
            std::mem::size_of::<ViewUniforms>() as BufferAddress,
        );

        self.queue.submit(Some(encoder.finish()));
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

    pub fn render_frame(&mut self, clear_color: wgpu::Color) -> Result<(), wgpu::SwapChainError> {
        self.shapes
            .retain(|shape_data| shape_data.strong_count() > 0);

        let frame = self.swap_chain.get_current_frame()?;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("RenderFrame"),
            });

        {
            let mut shape_data_cells: Vec<Arc<Mutex<ShapeData>>> = Vec::new();
            for shape_entry in &self.shapes {
                if let Some(shape_data_cell) = shape_entry.upgrade() {
                    shape_data_cells.push(shape_data_cell);
                }
            }

            let mut shape_data_locks: Vec<MutexGuard<ShapeData>> = Vec::new();
            for shape_data_cell in &shape_data_cells {
                shape_data_locks.push(shape_data_cell.lock().unwrap());
            }

            let mut shape_render_pass_data: Vec<ShapeRenderPassData> = Vec::new();
            for shape_data in &mut shape_data_locks {
                if let Some(shape_render_pass) =
                    ShapeRenderPassData::create(shape_data, &self.device)
                {
                    shape_render_pass_data.push(shape_render_pass);
                }
            }

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.output.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.view_uniform_bind_group, &[]);

            for shape_render_pass in &shape_render_pass_data {
                render_pass.set_vertex_buffer(0, shape_render_pass.vertex_buffer.slice(..));
                render_pass
                    .set_vertex_buffer(1, shape_render_pass.instance_colors_buffer.slice(..));
                render_pass
                    .set_vertex_buffer(2, shape_render_pass.instance_transforms_buffer.slice(..));
                render_pass.set_index_buffer(
                    shape_render_pass.index_buffer.slice(..),
                    IndexFormat::Uint16,
                );

                render_pass.draw_indexed(
                    0..shape_render_pass.num_indices,
                    0,
                    0..shape_render_pass.num_instances,
                );
            }
        }
        self.queue.submit(Some(encoder.finish()));
        Ok(())
    }
}

struct ShapeRenderPassData<'a> {
    vertex_buffer: &'a Buffer,
    index_buffer: &'a Buffer,
    num_indices: u32,

    instance_transforms_buffer: Buffer,
    instance_colors_buffer: Buffer,
    num_instances: u32,
}

impl<'a> ShapeRenderPassData<'a> {
    fn create(shape_data: &'a mut ShapeData, device: &Device) -> Option<Self> {
        if !shape_data.instance_transforms.is_empty() {
            let vertex_buffer = &shape_data.vertex_buffer;
            let index_buffer = &shape_data.index_buffer;
            let num_indices = shape_data.num_indices;

            let instance_transforms_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("instance_transforms"),
                    contents: shape_data.instance_transforms.as_bytes(),
                    usage: BufferUsage::VERTEX,
                });

            let instance_colors_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("instance_colors"),
                    contents: shape_data.instance_colors.as_bytes(),
                    usage: BufferUsage::VERTEX,
                });

            let num_instances = shape_data.instance_transforms.len() as u32;

            shape_data.instance_transforms.clear();
            shape_data.instance_colors.clear();

            Some(Self {
                vertex_buffer,
                index_buffer,
                num_indices,

                instance_transforms_buffer,
                instance_colors_buffer,
                num_instances,
            })
        } else {
            None
        }
    }
}
