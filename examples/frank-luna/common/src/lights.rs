use glam::Vec3;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Light {
    pub strength: Vec3,
    pub falloff_start: f32,
    pub direction: Vec3,
    pub falloff_end: f32,
    pub position: Vec3,
    pub spot_power: f32,
}

pub const MAX_LIGHTS: usize = 16;
