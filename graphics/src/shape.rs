use std::fmt;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

use crate::color::Color;
use crate::model_transform::ModelTransform;
use crate::vertex::Vertex;
use wgpu::{Buffer, BufferUsage, Device};

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
        let vertex_buffer = device
            .create_buffer_mapped(vertex_data.len(), BufferUsage::VERTEX)
            .fill_from_slice(vertex_data);

        let index_buffer = device
            .create_buffer_mapped(indices.len(), BufferUsage::INDEX)
            .fill_from_slice(indices);
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
