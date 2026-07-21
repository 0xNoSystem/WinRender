use std::f32::consts::{PI, TAU};

use crate::math::{BoundingBox, Vec3};

use super::Transform3D;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Mesh {
    pub(crate) vertices: Vec<Vec3>,
    pub(crate) indices: Vec<[usize; 3]>,
    pub(crate) bbox: BoundingBox,
}

impl Mesh {
    pub fn new(vertices: Vec<Vec3>, indices: Vec<[usize; 3]>) -> Self {
        let bbox = BoundingBox::from_vertices_3d(&vertices);

        Self {
            vertices,
            indices,
            bbox,
        }
    }

    pub fn bounding_rect(&self) -> BoundingBox {
        self.bbox
    }

    pub fn transformed_bounding_rect(&self, transform: Transform3D) -> BoundingBox {
        if transform == Transform3D::default() {
            return self.bbox;
        }

        let vertices = self
            .vertices
            .iter()
            .copied()
            .map(|vertex| transform.transform_point(vertex))
            .collect::<Vec<_>>();

        BoundingBox::from_vertices_3d(&vertices)
    }

    pub fn uv_sphere(r: f32, lat_steps: usize, long_steps: usize) -> Self {
        assert!(lat_steps >= 2);
        assert!(long_steps >= 3);
        assert!(r > 0.0);

        let mut vertices = Vec::with_capacity((lat_steps + 1) * (long_steps + 1));

        for lat in 0..=lat_steps {
            let phi = PI * lat as f32 / lat_steps as f32;

            for lon in 0..=long_steps {
                let theta = TAU * lon as f32 / long_steps as f32;

                let x = r * phi.sin() * theta.cos();
                let y = r * phi.cos();
                let z = r * phi.sin() * theta.sin();

                vertices.push(Vec3::new(x, y, z));
            }
        }

        let mut indices = Vec::with_capacity(lat_steps * long_steps * 2);
        let stride = long_steps + 1;

        for lat in 0..lat_steps {
            for lon in 0..long_steps {
                let a = lat * stride + lon;
                let b = a + 1;
                let c = a + stride;
                let d = c + 1;

                indices.push([a, b, c]);
                indices.push([b, d, c]);
            }
        }

        Self::new(vertices, indices)
    }

    pub fn torus(
        major_radius: f32,
        minor_radius: f32,
        seg_u: usize,
        seg_v: usize,
        tilt: f32,
    ) -> Self {
        assert!(seg_u >= 3);
        assert!(seg_v >= 3);

        let mut vertices = Vec::with_capacity(seg_u * seg_v);
        let mut indices = Vec::with_capacity(seg_u * seg_v * 2);

        let cos_tilt = tilt.cos();
        let sin_tilt = tilt.sin();

        let idx = |u: usize, v: usize| -> usize { u * seg_v + v };

        for u in 0..seg_u {
            let a = TAU * u as f32 / seg_u as f32;
            let cos_a = a.cos();
            let sin_a = a.sin();

            for v in 0..seg_v {
                let b = TAU * v as f32 / seg_v as f32;
                let cos_b = b.cos();
                let sin_b = b.sin();

                let x = (major_radius + minor_radius * cos_b) * cos_a;
                let y = minor_radius * sin_b;
                let z = (major_radius + minor_radius * cos_b) * sin_a;

                // Rotate around the X axis.
                let rotated_y = y * cos_tilt - z * sin_tilt;
                let rotated_z = y * sin_tilt + z * cos_tilt;

                vertices.push(Vec3::new(x, rotated_y, rotated_z));
            }
        }

        for u in 0..seg_u {
            let next_u = (u + 1) % seg_u;

            for v in 0..seg_v {
                let next_v = (v + 1) % seg_v;

                let a = idx(u, v);
                let b = idx(next_u, v);
                let c = idx(next_u, next_v);
                let d = idx(u, next_v);

                indices.push([a, b, c]);
                indices.push([a, c, d]);
            }
        }

        Self::new(vertices, indices)
    }
}
