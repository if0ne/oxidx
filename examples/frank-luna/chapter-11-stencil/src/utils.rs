use glam::{vec3, vec4, Mat4, Vec4, Vec4Swizzles};

pub trait MatrixExt {
    fn reflect(plane: Vec4) -> Mat4 {
        const NEG_TWO: Vec4 = vec4(-2.0, -2.0, -2.0, 0.0);

        let p = plane.normalize();
        let s = p * NEG_TWO;

        let a = vec4(p.x, p.x, p.x, p.x);
        let b = vec4(p.y, p.y, p.y, p.y);
        let c = vec4(p.z, p.z, p.z, p.z);
        let d = vec4(p.w, p.w, p.w, p.w);

        Mat4 {
            x_axis: a.mul_add(s, Vec4::X),
            y_axis: b.mul_add(s, Vec4::X),
            z_axis: c.mul_add(s, Vec4::X),
            w_axis: d.mul_add(s, Vec4::X),
        }
    }

    fn shadow(shadow_plane: Vec4, light_pos: Vec4) -> Mat4 {
        const SELECT0001: [u32; 4] = [0, 0, 0, u32::MAX];

        let p = shadow_plane.normalize_plane();
        let dot: u32 = p.dot(light_pos).to_bits();
        let p = -p;

        let a = vec4(p.x, p.x, p.x, p.x);
        let b = vec4(p.y, p.y, p.y, p.y);
        let c = vec4(p.z, p.z, p.z, p.z);
        let d = vec4(p.w, p.w, p.w, p.w);

        let dot = vec4(
            f32::from_bits(SELECT0001[0] & !SELECT0001[0] | dot & SELECT0001[0]),
            f32::from_bits(SELECT0001[1] & !SELECT0001[1] | dot & SELECT0001[1]),
            f32::from_bits(SELECT0001[2] & !SELECT0001[2] | dot & SELECT0001[2]),
            f32::from_bits(SELECT0001[3] & !SELECT0001[3] | dot & SELECT0001[3]),
        );

        Mat4 {
            x_axis: a.mul_add(light_pos, dot),
            y_axis: b.mul_add(light_pos, dot),
            z_axis: c.mul_add(light_pos, dot),
            w_axis: d.mul_add(light_pos, dot),
        }
    }

    fn create_shadow(shadow_plane: glam::Vec4, l: glam::Vec4) -> Mat4 {
        let d = shadow_plane.w;
        let n = shadow_plane.xyz().normalize();
        let nl = n.dot(l.xyz());

        glam::Mat4 {
            x_axis: glam::Vec4::new(nl + d * l.w - l.x * n.x, -l.x * n.y, -l.x * n.z, -l.x * d),
            y_axis: glam::Vec4::new(-l.y * n.x, nl + d * l.w - l.y * n.y, -l.y * n.z, -l.y * d),
            z_axis: glam::Vec4::new(-l.z * n.x, -l.z * n.y, nl + d * l.w - l.z * n.z, -l.z * d),
            w_axis: glam::Vec4::new(-l.w * n.x, -l.w * n.y, -l.w * n.z, nl),
        }
    }
}

impl MatrixExt for Mat4 {}

pub trait VecExt {
    fn normalize_plane(&self) -> Self;
}

impl VecExt for Vec4 {
    fn normalize_plane(&self) -> Self {
        let plane = vec3(self.x, self.y, self.z).normalize();
        vec4(plane.x, plane.y, plane.z, self.w)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn shadow_test() {
        use glam::{vec4, Mat4};

        use crate::utils::MatrixExt;

        let m = Mat4::shadow(vec4(0.0, 1.0, 0.0, 0.0), vec4(0.0, -0.707, -0.707, 1.0));

        assert_eq!(
            m,
            Mat4 {
                x_axis: vec4(-0.707, 0.0, 0.0, 0.0),
                y_axis: vec4(0.0, 0.0, 0.707, -1.0),
                z_axis: vec4(0.0, 0.0, -0.707, 0.0),
                w_axis: vec4(0.0, 0.0, 0.0, -0.707)
            }
        );
    }

    #[test]
    fn create_shadow_test() {
        use glam::{vec4, Mat4};

        use crate::utils::MatrixExt;

        let m = Mat4::create_shadow(vec4(0.0, 1.0, 0.0, 0.0), vec4(0.0, -0.707, -0.707, 1.0));

        assert_eq!(
            m,
            Mat4 {
                x_axis: vec4(-0.707, 0.0, 0.0, 0.0),
                y_axis: vec4(0.0, 0.0, 0.707, -1.0),
                z_axis: vec4(0.0, 0.0, -0.707, 0.0),
                w_axis: vec4(0.0, 0.0, 0.0, -0.707)
            }
        );
    }
}
