use glam::{Vec2, Vec3};

#[derive(Debug)]
pub struct GeometryGenerator;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vertex {
    pub pos: Vec3,
    pub normal: Vec3,
    pub tangent: Vec3,
    pub uv: Vec2,
}

#[derive(Clone, Debug)]
pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices32: Vec<u32>,
    pub indices16: Vec<u16>,
}

impl GeometryGenerator {
    pub fn create_box(width: f32, height: f32, depth: f32, num_subdivisions: u32) -> MeshData {
        todo!()
    }

    pub fn create_sphere(radius: f32, slice_count: u32, stack_count: u32) -> MeshData {
        todo!()
    }

    pub fn create_geosphere(radius: f32, num_subdivisions: u32) -> MeshData {
        todo!()
    }

    pub fn create_cylinder(bottom_radius: f32, top_radius: f32, height: f32, slice_count: u32, stack_count: u32) -> MeshData {
        todo!()
    }

    pub fn create_grid(width: f32, depth: f32, m: u32, n: u32) -> MeshData {
        todo!()
    }

    pub fn create_quad(x: f32, y: f32, w: f32, h: f32, depth: f32) -> MeshData {
        todo!()
    }
}

impl GeometryGenerator {
    fn subdivide(mesh_data: &mut MeshData) {
        todo!()
    }

    fn mid_point(v0: &Vertex, v1: &Vertex) -> Vertex {
        todo!()
    }

    fn build_cylider_top_cap(bottom_radius: f32, top_radius: f32, height: f32, slice_count: u32, stack_count: u32, mesh_data: &mut MeshData) -> Vertex {
        todo!()
    }

    fn build_cylider_bottom_cap(bottom_radius: f32, top_radius: f32, height: f32, slice_count: u32, stack_count: u32, mesh_data: &mut MeshData) -> Vertex {
        todo!()
    }
}