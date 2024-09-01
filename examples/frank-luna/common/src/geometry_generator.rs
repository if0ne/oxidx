use std::{
    cell::{Ref, RefCell},
    f32::consts::PI,
};

use glam::{vec2, vec3, Vec2, Vec3};

#[derive(Debug)]
pub struct GeometryGenerator;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Vertex {
    pub pos: Vec3,
    pub normal: Vec3,
    pub tangent: Vec3,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        xn: f32,
        yn: f32,
        zn: f32,
        xt: f32,
        yt: f32,
        zt: f32,
        u: f32,
        v: f32,
    ) -> Self {
        Self {
            pos: vec3(x, y, z),
            normal: vec3(xn, yn, zn),
            tangent: vec3(xt, yt, zt),
            uv: vec2(u, v),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices32: Vec<u32>,
    pub indices16: RefCell<Vec<u16>>,
}

impl MeshData {
    pub fn indices16<'a>(&self) -> Ref<'_, Vec<u16>> {
        {
            let idx = &mut *self.indices16.borrow_mut();

            if idx.is_empty() {
                let mut vec = Vec::with_capacity(self.indices32.len());
                for i in &self.indices32 {
                    vec.push(*i as u16);
                }

                *idx = vec;
            }
        }

        self.indices16.borrow()
    }
}

impl GeometryGenerator {
    pub fn create_box(width: f32, height: f32, depth: f32, num_subdivisions: u32) -> MeshData {
        let w2 = 0.5 * width;
        let h2 = 0.5 * height;
        let d2 = 0.5 * depth;

        let v = [
            Vertex::new(-w2, -h2, -d2, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0),
            Vertex::new(-w2, h2, -d2, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0),
            Vertex::new(w2, h2, -d2, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0),
            Vertex::new(w2, -h2, -d2, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0),
            Vertex::new(-w2, -h2, d2, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 1.0),
            Vertex::new(w2, -h2, d2, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 1.0),
            Vertex::new(w2, h2, d2, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0),
            Vertex::new(-w2, h2, d2, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0),
            Vertex::new(-w2, h2, -d2, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0),
            Vertex::new(-w2, h2, d2, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0),
            Vertex::new(w2, h2, d2, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0),
            Vertex::new(w2, h2, -d2, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0),
            Vertex::new(-w2, -h2, -d2, 0.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0, 1.0),
            Vertex::new(w2, -h2, -d2, 0.0, -1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0),
            Vertex::new(w2, -h2, d2, 0.0, -1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0),
            Vertex::new(-w2, -h2, d2, 0.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0),
            Vertex::new(-w2, -h2, d2, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0),
            Vertex::new(-w2, h2, d2, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0),
            Vertex::new(-w2, h2, -d2, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0),
            Vertex::new(-w2, -h2, -d2, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 1.0),
            Vertex::new(w2, -h2, -d2, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0),
            Vertex::new(w2, h2, -d2, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            Vertex::new(w2, h2, d2, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0),
            Vertex::new(w2, -h2, d2, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0),
        ];

        let vertices = v.to_vec();

        let mut i = [0u32; 36];
        i[0] = 0;
        i[1] = 1;
        i[2] = 2;
        i[3] = 0;
        i[4] = 2;
        i[5] = 3;

        i[6] = 4;
        i[7] = 5;
        i[8] = 6;
        i[9] = 4;
        i[10] = 6;
        i[11] = 7;

        i[12] = 8;
        i[13] = 9;
        i[14] = 10;
        i[15] = 8;
        i[16] = 10;
        i[17] = 11;

        i[18] = 12;
        i[19] = 13;
        i[20] = 14;
        i[21] = 12;
        i[22] = 14;
        i[23] = 15;

        i[24] = 16;
        i[25] = 17;
        i[26] = 18;
        i[27] = 16;
        i[28] = 18;
        i[29] = 19;

        i[30] = 20;
        i[31] = 21;
        i[32] = 22;
        i[33] = 20;
        i[34] = 22;
        i[35] = 23;

        let indices = i.to_vec();

        let mut mesh_data = MeshData {
            vertices,
            indices32: indices,
            indices16: Default::default(),
        };

        for _ in 0..num_subdivisions.min(6) {
            Self::subdivide(&mut mesh_data)
        }

        mesh_data
    }

    pub fn create_sphere(radius: f32, slice_count: u32, stack_count: u32) -> MeshData {
        let mut vertices = vec![];

        let top_vertex = Vertex::new(0.0, radius, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let bottom_vertex = Vertex::new(0.0, -radius, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);

        vertices.push(top_vertex);

        let phi_step = PI / stack_count as f32;
        let theta_step = 2.0 * PI / slice_count as f32;

        for i in 1..stack_count {
            let phi = i as f32 * phi_step;

            for j in 0..=slice_count {
                let theta = j as f32 * theta_step;

                let pos = vec3(
                    radius * phi.sin() * theta.cos(),
                    radius * phi.cos(),
                    radius * phi.sin() * theta.sin(),
                );

                vertices.push(Vertex {
                    pos,
                    normal: pos.normalize(),
                    tangent: vec3(
                        -radius * phi.sin() * theta.sin(),
                        0.0,
                        radius * phi.sin() * theta.cos(),
                    ),
                    uv: vec2(theta / (2.0 * PI), phi / PI),
                });
            }
        }

        vertices.push(bottom_vertex);

        let mut indices = vec![];

        for i in 0..=slice_count {
            indices.push(0);
            indices.push(i + 1);
            indices.push(i);
        }

        let base_index = 1;
        let ring_vertex_count = slice_count + 1;

        for i in 0..(stack_count - 2) {
            for j in 0..slice_count {
                indices.push(base_index + i * ring_vertex_count + j);
                indices.push(base_index + i * ring_vertex_count + j + 1);
                indices.push(base_index + (i + 1) * ring_vertex_count + j);

                indices.push(base_index + (i + 1_ * ring_vertex_count + j));
                indices.push(base_index + i * ring_vertex_count + j + 1);
                indices.push(base_index + (i + 1) * ring_vertex_count + j + 1);
            }
        }

        let south_pole_index = vertices.len() as u32 - 1;

        let base_index = south_pole_index - ring_vertex_count;

        for i in 0..=slice_count {
            indices.push(south_pole_index);
            indices.push(base_index + 1);
            indices.push(base_index + i + 1);
        }

        MeshData {
            vertices: vertices,
            indices32: indices,
            indices16: Default::default(),
        }
    }

    pub fn create_geosphere(radius: f32, num_subdivisions: u32) -> MeshData {
        let num_subdivisions = num_subdivisions.min(6);

        const X: f32 = 0.525731;
        const Z: f32 = 0.850651;

        let indices = vec![
            1, 4, 0, 4, 9, 0, 4, 5, 9, 8, 5, 4, 1, 8, 4, 1, 10, 8, 10, 3, 8, 8, 3, 5, 3, 2, 5, 3,
            7, 2, 3, 10, 7, 10, 6, 7, 6, 11, 7, 6, 0, 11, 6, 1, 0, 10, 1, 6, 11, 0, 9, 2, 11, 9, 5,
            2, 9, 11, 2, 7,
        ];

        let pos = [
            vec3(-X, 0.0, Z),
            vec3(X, 0.0, Z),
            vec3(-X, 0.0, -Z),
            vec3(X, 0.0, Z),
            vec3(0.0, Z, X),
            vec3(0.0, Z, -X),
            vec3(0.0, -Z, X),
            vec3(0.0, -Z, -X),
            vec3(Z, X, 0.0),
            vec3(-Z, X, 0.0),
            vec3(Z, -X, 0.0),
            vec3(-Z, -X, 0.0),
        ];

        let mut vertices = Vec::with_capacity(pos.len());

        for pos in pos.into_iter() {
            vertices.push(Vertex {
                pos,
                ..Default::default()
            })
        }

        let mut mesh_data = MeshData {
            vertices,
            indices32: indices,
            indices16: Default::default(),
        };

        for _ in 0..num_subdivisions {
            Self::subdivide(&mut mesh_data);
        }

        for vert in mesh_data.vertices.iter_mut() {
            let n = vert.pos.normalize();
            let p = radius * n;

            vert.pos = p;
            vert.normal = n;

            let mut theta = vert.pos.z.atan2(vert.pos.x);

            if theta < 0.0 {
                theta += 2.0 * PI;
            }

            let phi = (vert.pos.y / radius).acos();

            vert.uv = vec2(theta / (2.0 * PI), phi / PI);
            vert.tangent = vec3(
                -radius * phi.sin() * theta.sin(),
                0.0,
                radius * phi.sin() * theta.cos(),
            )
            .normalize();
        }

        todo!()
    }

    pub fn create_cylinder(
        bottom_radius: f32,
        top_radius: f32,
        height: f32,
        slice_count: u32,
        stack_count: u32,
    ) -> MeshData {
        let mut vertices = vec![];

        let stack_height = height / stack_count as f32;
        let radius_step = (top_radius - bottom_radius) / stack_count as f32;
        let ring_count = stack_count + 1;

        for i in 0..ring_count {
            let y = -0.5 * height + i as f32 * stack_height;
            let r = bottom_radius + i as f32 * radius_step;

            let dtheta = 2.0 * PI / slice_count as f32;

            for j in 0..=slice_count {
                let c = (j as f32 * dtheta).cos();
                let s = (j as f32 * dtheta).sin();

                let dr = bottom_radius - top_radius;
                let bitangent = vec3(dr * c, -height, dr * s);
                let tangent = vec3(-s, 0.0, c);
                let normal = tangent.cross(bitangent).normalize();

                vertices.push(Vertex {
                    pos: vec3(r * c, y, r * s),
                    normal,
                    tangent,
                    uv: vec2(
                        j as f32 / slice_count as f32,
                        1.0 - i as f32 / stack_count as f32,
                    ),
                })
            }
        }

        let mut indices = vec![];

        let ring_vertex_count = slice_count + 1;

        for i in 0..stack_count {
            for j in 0..slice_count {
                indices.push(i * ring_vertex_count + j);
                indices.push((i + 1) * ring_vertex_count + j);
                indices.push((i + 1) * ring_vertex_count + (j + 1));

                indices.push(i * ring_vertex_count + j);
                indices.push((i + 1) * ring_vertex_count + j + 1);
                indices.push(i * ring_vertex_count + (j + 1));
            }
        }

        let mut mesh_data = MeshData {
            vertices,
            indices32: indices,
            indices16: Default::default(),
        };

        Self::build_cylider_top_cap(
            bottom_radius,
            top_radius,
            height,
            slice_count,
            stack_count,
            &mut mesh_data,
        );
        Self::build_cylider_bottom_cap(
            bottom_radius,
            top_radius,
            height,
            slice_count,
            stack_count,
            &mut mesh_data,
        );

        mesh_data
    }

    pub fn create_grid(width: f32, depth: f32, m: u32, n: u32) -> MeshData {
        let vertex_count = m * n;

        let half_width = width * 0.5;
        let half_depth = depth * 0.5;

        let dx = width / (n as f32 - 1.0);
        let dz = depth / (m as f32 - 1.0);

        let du = 1.0 / (n as f32 - 1.0);
        let dv = 1.0 / (m as f32 - 1.0);

        let mut vertices = Vec::with_capacity(vertex_count as usize);

        for i in 0..m {
            let z = half_depth - i as f32 * dz;
            for j in 0..n {
                let x = -half_width + j as f32 * dx;

                vertices.push(Vertex {
                    pos: vec3(x, 0.0, z),
                    normal: Vec3::Y,
                    tangent: Vec3::X,
                    uv: vec2(j as f32 * du, i as f32 * dv),
                });
            }
        }

        let mut indices = Vec::with_capacity(vertex_count as usize);

        for i in 0..(m - 1) {
            for j in 0..(n - 1) {
                indices.push(i * n + j);
                indices.push(i * n + j + 1);
                indices.push((i + 1) * n + j);

                indices.push((i + 1) * n + j);
                indices.push(i * n + j + 1);
                indices.push((i + 1) * n + j + 1);
            }
        }

        MeshData {
            vertices,
            indices32: indices,
            indices16: Default::default(),
        }
    }

    pub fn create_quad(x: f32, y: f32, w: f32, h: f32, depth: f32) -> MeshData {
        MeshData {
            vertices: vec![
                Vertex::new(x, y - h, depth, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0),
                Vertex::new(x, y, depth, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0),
                Vertex::new(x + w, y, depth, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0),
                Vertex::new(x + w, y - h, depth, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0),
            ],
            indices32: vec![0, 1, 2, 0, 2, 3],
            indices16: Default::default(),
        }
    }
}

impl GeometryGenerator {
    fn subdivide(mesh_data: &mut MeshData) {
        let mut copy = mesh_data.clone();

        copy.vertices.truncate(0);
        copy.indices32.truncate(0);
        copy.indices16.borrow_mut().truncate(0);

        for (idx, i) in copy.indices32.chunks_exact(3).enumerate() {
            let idx = idx as u32;
            let i0 = i[0];
            let i1 = i[1];
            let i2 = i[2];
            let [v0, v1, v2] = [
                copy.vertices[i0 as usize],
                copy.vertices[i1 as usize],
                copy.vertices[i2 as usize],
            ];

            let m0 = Self::mid_point(&v0, &v1);
            let m1 = Self::mid_point(&v1, &v2);
            let m2 = Self::mid_point(&v0, &v2);

            mesh_data.vertices.push(v0);
            mesh_data.vertices.push(v1);
            mesh_data.vertices.push(v2);
            mesh_data.vertices.push(m0);
            mesh_data.vertices.push(m1);
            mesh_data.vertices.push(m2);

            mesh_data.indices32.push(idx * 6 + 0);
            mesh_data.indices32.push(idx * 6 + 3);
            mesh_data.indices32.push(idx * 6 + 5);

            mesh_data.indices32.push(idx * 6 + 3);
            mesh_data.indices32.push(idx * 6 + 4);
            mesh_data.indices32.push(idx * 6 + 5);

            mesh_data.indices32.push(idx * 6 + 5);
            mesh_data.indices32.push(idx * 6 + 4);
            mesh_data.indices32.push(idx * 6 + 1);

            mesh_data.indices32.push(idx * 6 + 3);
            mesh_data.indices32.push(idx * 6 + 1);
            mesh_data.indices32.push(idx * 6 + 4);
        }
    }

    fn mid_point(v0: &Vertex, v1: &Vertex) -> Vertex {
        Vertex {
            pos: 0.5 * (v0.pos + v1.pos),
            normal: (0.5 * (v0.normal + v1.normal)).normalize(),
            tangent: (0.5 * (v0.tangent + v1.tangent)).normalize(),
            uv: 0.5 * (v0.uv + v1.uv),
        }
    }

    fn build_cylider_top_cap(
        _bottom_radius: f32,
        top_radius: f32,
        height: f32,
        slice_count: u32,
        _stack_count: u32,
        mesh_data: &mut MeshData,
    ) {
        let base_index = mesh_data.vertices.len() as u32;

        let y = 0.5 * height;
        let dtheta = 2.0 * PI / slice_count as f32;

        for i in 0..slice_count {
            let x = top_radius * (i as f32 * dtheta).cos();
            let z = top_radius * (i as f32 * dtheta).sin();

            let u = x / height + 0.5;
            let v = z / height + 0.5;

            mesh_data
                .vertices
                .push(Vertex::new(x, y, z, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, u, v));
        }

        mesh_data.vertices.push(Vertex::new(
            0.0, y, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.5, 0.5,
        ));

        let center = mesh_data.vertices.len() as u32 - 1;

        for i in 0..slice_count {
            mesh_data.indices32.push(center);
            mesh_data.indices32.push(base_index + i + 1);
            mesh_data.indices32.push(base_index + i);
        }
    }

    fn build_cylider_bottom_cap(
        bottom_radius: f32,
        _top_radius: f32,
        height: f32,
        slice_count: u32,
        _stack_count: u32,
        mesh_data: &mut MeshData,
    ) {
        let base_index = mesh_data.vertices.len() as u32;

        let y = -0.5 * height;
        let dtheta = 2.0 * PI / slice_count as f32;

        for i in 0..slice_count {
            let x = bottom_radius * (i as f32 * dtheta).cos();
            let z = bottom_radius * (i as f32 * dtheta).sin();

            let u = x / height + 0.5;
            let v = z / height + 0.5;

            mesh_data
                .vertices
                .push(Vertex::new(x, y, z, 0.0, -1.0, 0.0, 1.0, 0.0, 0.0, u, v));
        }

        mesh_data.vertices.push(Vertex::new(
            0.0, y, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.5, 0.5,
        ));

        let center = mesh_data.vertices.len() as u32 - 1;

        for i in 0..slice_count {
            mesh_data.indices32.push(center);
            mesh_data.indices32.push(base_index + i);
            mesh_data.indices32.push(base_index + i + 1);
        }
    }
}
