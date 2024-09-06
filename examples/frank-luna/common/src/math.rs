use glam::{vec3, Vec3};

pub fn spherical_to_cartesian(r: f32, theta: f32, phi: f32) -> Vec3 {
    vec3(
        r * phi.sin() * theta.cos(),
        r * phi.cos(),
        r * phi.sin() * theta.sin(),
    )
}
