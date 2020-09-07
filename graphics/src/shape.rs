use crate::color::Color;
use crate::model_transform::ModelTransform;
use crate::vertex::Vertex;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, BufferUsage, Device};
use zerocopy::AsBytes;

#[derive(Clone)]
pub struct Shape {
    pub(crate) data: Arc<Mutex<ShapeData>>,
    pub name: &'static str,
}

impl fmt::Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Shape").field("name", &self.name).finish()
    }
}

pub(crate) struct ShapeData {
    pub(crate) instance_transforms: Vec<ModelTransform>,
    pub(crate) instance_colors: Vec<Color>,

    pub(crate) vertex_buffer: Buffer,
    pub(crate) index_buffer: Buffer,
    pub(crate) num_indices: u32,
}

impl ShapeData {
    pub(crate) fn new(device: &mut Device, vertex_data: &[Vertex], indices: &[u16]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shape_vertex"),
            contents: vertex_data.as_bytes(),
            usage: BufferUsage::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shape_indices"),
            contents: indices.as_bytes(),
            usage: BufferUsage::INDEX,
        });
        let num_indices = indices.len() as u32;

        Self {
            instance_transforms: Vec::new(),
            instance_colors: Vec::new(),

            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }
}
