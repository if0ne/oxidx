use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use common::geometry_mesh::MeshGeometry;
use glam::Mat4;
use oxidx::dx::PrimitiveTopology;

#[derive(Debug)]
pub struct RenderItem {
    pub world: Mat4,
    pub num_frames_dirty: Cell<usize>,
    pub obj_cb_index: usize,
    pub geo: Rc<RefCell<MeshGeometry>>,
    pub primitive_type: PrimitiveTopology,
    pub index_count: u32,
    pub start_index_location: u32,
    pub base_vertex_location: u32,
}
