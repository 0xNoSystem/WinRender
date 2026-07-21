use crate::color::TriangleFillType;
use crate::math::{Vec2, Vec3};
use crate::three_d::{Mesh, Object, Triangle, Triangle3, edge};

pub type FrameBuffer = Vec<u32>;
pub type DepthBuffer = Vec<f32>;

pub struct ScreenBuffer {
    pub h: u32,
    pub w: u32,
    buffer: FrameBuffer,
    depth_buffer: DepthBuffer,
}

impl ScreenBuffer {
    pub fn new(w: u32, h: u32, color: Option<u32>) -> Self {
        let len = (w * h) as usize;
        Self {
            h,
            w,
            buffer: vec![color.unwrap_or(0u32); len],
            depth_buffer: vec![f32::INFINITY; len],
        }
    }

    pub fn clear(&mut self, color: Option<u32>) {
        self.buffer.fill(color.unwrap_or(0u32));
        self.depth_buffer.fill(f32::INFINITY);
    }

    pub fn set_pixel_value(&mut self, x: i32, y: i32, color: u32) {
        if x < 0 || y < 0 || x >= self.w as i32 || y >= self.h as i32 {
            return;
        }

        let idx = y as usize * self.w as usize + x as usize;
        self.buffer[idx] = color;
    }

    pub fn pixels(&self) -> &[u32] {
        &self.buffer
    }

    pub fn draw_line(&mut self, p1: Vec2, p2: Vec2, color: u32) {
        let delta = p2 - p1;
        let steps = delta.x.abs().max(delta.y.abs()).ceil() as u32;

        if steps == 0 {
            self.set_pixel_value(p1.x.round() as i32, p1.y.round() as i32, color);
            return;
        }

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let pt = p1 + delta * t;

            self.set_pixel_value(pt.x.round() as i32, pt.y.round() as i32, color);
        }
    }

    pub fn draw_triangle(&mut self, Triangle { p0, p1, p2 }: Triangle, color: u32) {
        self.draw_line(p0, p1, color);
        self.draw_line(p1, p2, color);
        self.draw_line(p2, p0, color);
    }

    pub fn fill_triangle(&mut self, tri: Triangle, z_index: Option<f32>, fill: TriangleFillType) {
        if tri.is_degenerate() {
            return;
        }

        let Some(bbox) = tri.bounding_rect().clamp(self.w, self.h) else {
            return;
        };

        let area = edge(tri.p1, tri.p2, tri.p0);

        for y in bbox.min_y..=bbox.max_y {
            for x in bbox.min_x..=bbox.max_x {
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);

                if !tri.contains_point(p) {
                    continue;
                }

                let color = match fill {
                    TriangleFillType::Solid(color) => color,
                    TriangleFillType::Gradient { c0, c1, c2 } => {
                        let w0 = edge(tri.p1, tri.p2, p) / area;
                        let w1 = edge(tri.p2, tri.p0, p) / area;
                        let w2 = edge(tri.p0, tri.p1, p) / area;

                        (c0 * w0 + c1 * w1 + c2 * w2).to_u32()
                    }
                };

                if let Some(z) = z_index {
                    self.set_pixel_depth(x, y, z, color);
                } else {
                    self.set_pixel_value(x, y, color);
                }
            }
        }
    }
}

impl ScreenBuffer {
    pub fn set_pixel_depth(&mut self, x: i32, y: i32, z: f32, color: u32) {
        if x < 0 || y < 0 || x >= self.w as i32 || y >= self.h as i32 {
            return;
        }

        let idx = y as usize * self.w as usize + x as usize;

        if z < self.depth_buffer[idx] {
            self.depth_buffer[idx] = z;
            self.buffer[idx] = color;
        }
    }

    pub fn draw_line_depth(&mut self, p1: Vec3, p2: Vec3, color: u32) {
        let delta = p2 - p1;
        let steps = delta.x.abs().max(delta.y.abs()).ceil() as u32;

        if steps == 0 {
            self.set_pixel_depth(p1.x.round() as i32, p1.y.round() as i32, p1.z, color);
            return;
        }

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let pt = p1 + delta * t;

            self.set_pixel_depth(pt.x.round() as i32, pt.y.round() as i32, pt.z, color);
        }
    }

    pub fn draw_triangle_3d(&mut self, Triangle3 { p0, p1, p2 }: Triangle3, color: u32) {
        self.draw_line_depth(p0, p1, color);
        self.draw_line_depth(p1, p2, color);
        self.draw_line_depth(p2, p0, color);
    }

    pub fn fill_triangle_3d(&mut self, tri: Triangle3, fill: TriangleFillType) {
        let screen_tri = tri.to_screen_triangle();
        let area = edge(screen_tri.p1, screen_tri.p2, screen_tri.p0);

        let Some(bbox) = tri.bounding_rect().clamp(self.w, self.h) else {
            return;
        };

        let inv_area = 1.0 / area;

        for y in bbox.min_y..=bbox.max_y {
            for x in bbox.min_x..=bbox.max_x {
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);

                let e0 = edge(screen_tri.p1, screen_tri.p2, p);
                let e1 = edge(screen_tri.p2, screen_tri.p0, p);
                let e2 = edge(screen_tri.p0, screen_tri.p1, p);

                if e0 < 0.0 || e1 < 0.0 || e2 < 0.0 {
                    continue;
                }

                let w0 = e0 * inv_area;
                let w1 = e1 * inv_area;
                let w2 = e2 * inv_area;
                let z = tri.p0.z * w0 + tri.p1.z * w1 + tri.p2.z * w2;

                let color = match fill {
                    TriangleFillType::Solid(color) => color,
                    TriangleFillType::Gradient { c0, c1, c2 } => {
                        (c0 * w0 + c1 * w1 + c2 * w2).to_u32()
                    }
                };

                self.set_pixel_depth(x, y, z, color);
            }
        }
    }

    pub fn fill_indexed_mesh(
        &mut self,
        vertices: &[Vec3],
        indices: &[[usize; 3]],
        fill: TriangleFillType,
    ) {
        for &[a, b, c] in indices {
            let tri = Triangle3::new(vertices[a], vertices[b], vertices[c]);

            self.fill_triangle_3d(tri, fill);
        }
    }
}

pub struct Renderer {
    pub screen: ScreenBuffer,
}

impl Renderer {
    pub fn new(w: u32, h: u32, color: Option<u32>) -> Self {
        Self {
            screen: ScreenBuffer::new(w, h, color),
        }
    }

    pub fn draw_mesh(&mut self, object: &Object, mesh: &Mesh) {
        if !object.visible {
            return;
        }

        let transformed: Vec<Vec3> = mesh
            .vertices
            .iter()
            .map(|&vertex| object.transform.transform_point(vertex))
            .collect();

        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for vertex in &transformed {
            min_x = min_x.min(vertex.x);
            max_x = max_x.max(vertex.x);
            min_y = min_y.min(vertex.y);
            max_y = max_y.max(vertex.y);
        }

        let outside_screen = max_x < 0.0
            || max_y < 0.0
            || min_x >= self.screen.w as f32
            || min_y >= self.screen.h as f32;

        if outside_screen {
            return;
        }

        for &[a, b, c] in &mesh.indices {
            let tri = Triangle3::new(transformed[a], transformed[b], transformed[c]);

            self.screen
                .fill_triangle_3d(tri, TriangleFillType::Solid(object.material_id.0));
        }
    }
}
