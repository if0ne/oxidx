use glam::{vec3, vec4, Mat4, Vec4};

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
        let dot: u32 = unsafe { std::mem::transmute(p.dot(light_pos)) };
        let p = -p;

        let a = vec4(p.x, p.x, p.x, p.x);
        let b = vec4(p.y, p.y, p.y, p.y);
        let c = vec4(p.z, p.z, p.z, p.z);
        let d = vec4(p.w, p.w, p.w, p.w);

        let dot = unsafe {
            vec4(
                std::mem::transmute(SELECT0001[0] & !SELECT0001[0] | dot & SELECT0001[0]),
                std::mem::transmute(SELECT0001[1] & !SELECT0001[1] | dot & SELECT0001[1]),
                std::mem::transmute(SELECT0001[2] & !SELECT0001[2] | dot & SELECT0001[2]),
                std::mem::transmute(SELECT0001[3] & !SELECT0001[3] | dot & SELECT0001[3]),
            )
        };

        Mat4 {
            x_axis: a.mul_add(light_pos, dot),
            y_axis: b.mul_add(light_pos, dot),
            z_axis: c.mul_add(light_pos, dot),
            w_axis: d.mul_add(light_pos, dot),
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
