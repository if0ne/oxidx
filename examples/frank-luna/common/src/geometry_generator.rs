use std::cell::{Ref, RefCell};

use glam::{vec2, vec3, Vec2, Vec3};

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
        todo!()
    }

    pub fn create_geosphere(radius: f32, num_subdivisions: u32) -> MeshData {
        todo!()
    }

    pub fn create_cylinder(
        bottom_radius: f32,
        top_radius: f32,
        height: f32,
        slice_count: u32,
        stack_count: u32,
    ) -> MeshData {
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
        bottom_radius: f32,
        top_radius: f32,
        height: f32,
        slice_count: u32,
        stack_count: u32,
        mesh_data: &mut MeshData,
    ) -> Vertex {
        todo!()
    }

    fn build_cylider_bottom_cap(
        bottom_radius: f32,
        top_radius: f32,
        height: f32,
        slice_count: u32,
        stack_count: u32,
        mesh_data: &mut MeshData,
    ) -> Vertex {
        todo!()
    }
}
