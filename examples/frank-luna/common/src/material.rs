use glam::{Mat4, Vec3, Vec4};

#[derive(Debug, Default)]
pub struct Material {
    pub name: String,
    pub cb_index: usize,
    pub diffuse_srv_heap_index: Option<usize>,
    pub num_frames_dirty: usize,
    pub diffuse_albedo: Vec4,
    pub fresnel_r0: Vec3,
    pub roughness: f32,
    pub transform: Mat4,
}
