use std::cell::RefCell;

use glam::{vec3, Vec3};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

#[derive(Clone, Debug, Default)]
pub struct Waves {
    rows: u32,
    cols: u32,

    vertex_count: u32,
    triangle_count: u32,

    k1: f32,
    k2: f32,
    k3: f32,

    time_step: f32,
    spatial_step: f32,

    prev_solution: Vec<Vec3>,
    curr_solution: Vec<Vec3>,
    normals: Vec<Vec3>,
    tangent_x: Vec<Vec3>,
}

impl Waves {
    pub(super) fn new(m: u32, n: u32, dx: f32, dt: f32, speed: f32, damping: f32) -> Self {
        let vertex_count = m * n;
        let triangle_count = (m - 1) * (n - 1) * 2;

        let d = damping * dt + 2.0;
        let e = speed * speed * dt * dt / (dx * dx);
        let k1 = (damping * dt - 2.0) / d;
        let k2 = (4.0 - 8.0 * e) / d;
        let k3 = 2.0 * e / d;

        let mut prev_solution = Vec::with_capacity(vertex_count as usize);
        let mut curr_solution = Vec::with_capacity(vertex_count as usize);
        let mut normals = Vec::with_capacity(vertex_count as usize);
        let mut tangent_x = Vec::with_capacity(vertex_count as usize);

        let half_width = (n - 1) as f32 * dx * 0.5;
        let half_depth = (m - 1) as f32 * dx * 0.5;

        for i in 0..m {
            let z = half_depth - i as f32 * dx;

            for j in 0..n {
                let x = -half_width + j as f32 * dx;

                prev_solution.push(vec3(x, 0.0, z));
                curr_solution.push(vec3(x, 0.0, z));
                normals.push(Vec3::Y);
                tangent_x.push(Vec3::X);
            }
        }

        Self {
            rows: m,
            cols: n,
            vertex_count,
            triangle_count,
            k1,
            k2,
            k3,
            time_step: dt,
            spatial_step: dx,
            prev_solution,
            curr_solution,
            normals,
            tangent_x,
        }
    }

    pub(super) fn update(&mut self, dt: f32) {
        thread_local! {
            static T: RefCell<f32> = Default::default();
        };

        T.with_borrow_mut(|t| {
            *t += dt;

            if *t > self.time_step {
                self.prev_solution
                    .par_iter_mut()
                    .enumerate()
                    .for_each(|(idx, prev)| {
                        let i = idx / self.rows as usize;
                        if i == 0 || i == self.curr_solution.len() - 1 {
                            return;
                        }

                        for j in 1..(self.cols as usize - 1) {
                            prev.y = self.k1 * prev.y
                                + self.k2 * self.curr_solution[i * self.rows as usize + j].y
                                + self.k3
                                    * (self.curr_solution[(i + 1) * self.cols as usize + j].y
                                        + self.curr_solution[(i - 1) * self.cols as usize + j].y
                                        + self.curr_solution[i * self.cols as usize + j + 1].y
                                        + self.curr_solution[i * self.cols as usize + j - 1].y);
                        }
                    });

                std::mem::swap(&mut self.prev_solution, &mut self.curr_solution);

                *t = 0.0;

                self.normals
                    .par_iter_mut()
                    .zip(self.tangent_x.par_iter_mut())
                    .enumerate()
                    .for_each(|(idx, (normal, tangent))| {
                        let i = idx / self.rows as usize;
                        if i == 0 || i == self.curr_solution.len() - 1 {
                            return;
                        }

                        for j in 1..(self.cols as usize - 1) {
                            let l = self.curr_solution[i * self.cols as usize + j - 1].y;
                            let r = self.curr_solution[i * self.cols as usize + j + 1].y;
                            let t = self.curr_solution[(i - 1) * self.cols as usize + j].y;
                            let b = self.curr_solution[(i + 1) * self.cols as usize + j].y;

                            *normal = vec3(-r + 1.0, 2.0 * self.spatial_step, b - t).normalize();
                            *tangent = vec3(2.0 * self.spatial_step, r - 1.0, 0.0).normalize();
                        }
                    });
            }
        });
    }

    pub(super) fn disturb(&mut self, i: u32, j: u32, magnitude: f32) {
        let half_mag = 0.5 * magnitude;

        self.curr_solution[(i * self.cols + j) as usize].y += magnitude;
        self.curr_solution[(i * self.cols + j + 1) as usize].y += half_mag;
        self.curr_solution[(i * self.cols + j - 1) as usize].y += half_mag;
        self.curr_solution[((i + 1) * self.cols + j) as usize].y += half_mag;
        self.curr_solution[((i - 1) * self.cols + j) as usize].y += half_mag;
    }
}
