use std::collections::HashMap;

use oxidx::dx::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct BoundingBox {
    pub min: glam::Vec3,
    pub max: glam::Vec3,
}

#[derive(Clone, Copy, Debug)]
pub struct SubmeshGeometry {
    pub index_count: u32,
    pub start_index_location: u32,
    pub base_vertex_location: u32,
    pub bounds: BoundingBox,
}

#[derive(Clone, Debug)]
pub struct MeshGeometry {
    pub name: String,

    pub vertex_buffer_cpu: Blob,
    pub index_buffer_cpu: Blob,

    pub vertex_buffer_gpu: Option<Resource>,
    pub index_buffer_gpu: Option<Resource>,

    pub vertex_buffer_uploader: Option<Resource>,
    pub index_buffer_uploader: Option<Resource>,

    pub vertex_byte_stride: u32,
    pub vertex_byte_size: u32,
    pub index_format: Format,
    pub index_buffer_byte_size: u32,

    pub draw_args: HashMap<String, SubmeshGeometry>,
}

impl MeshGeometry {
    pub fn vertex_buffer_view(&self) -> VertexBufferView {
        VertexBufferView::new(
            self.vertex_buffer_gpu
                .as_ref()
                .expect("Vertex buffer should be set")
                .get_gpu_virtual_address(),
            self.vertex_byte_stride as usize,
            self.vertex_byte_size as usize,
        )
    }

    pub fn index_buffer_view(&self) -> IndexBufferView {
        IndexBufferView::new(
            self.index_buffer_gpu
                .as_ref()
                .expect("Index buffer should be set")
                .get_gpu_virtual_address(),
            self.index_buffer_byte_size as usize,
            self.index_format,
        )
    }

    pub fn dispose_uploaders(&mut self) {
        self.vertex_buffer_uploader.take();
        self.index_buffer_uploader.take();
    }
}
