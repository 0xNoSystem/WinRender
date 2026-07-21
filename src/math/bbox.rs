use super::{Vec2, Vec3};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct BoundingBox {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

impl BoundingBox {
    pub fn clamp(self, width: u32, height: u32) -> Option<Self> {
        if width == 0 || height == 0 {
            return None;
        }

        let min_x = self.min_x.max(0);
        let min_y = self.min_y.max(0);
        let max_x = self.max_x.min(width as i32 - 1);
        let max_y = self.max_y.min(height as i32 - 1);

        if min_x > max_x || min_y > max_y {
            return None;
        }

        Some(Self {
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }

    pub fn from_vertices_2d(vertices: &[Vec2]) -> Self {
        if vertices.is_empty() {
            return Self::default();
        }

        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for vertex in vertices {
            min_x = min_x.min(vertex.x);
            max_x = max_x.max(vertex.x);
            min_y = min_y.min(vertex.y);
            max_y = max_y.max(vertex.y);
        }

        Self {
            min_x: min_x.floor() as i32,
            max_x: max_x.ceil() as i32,
            min_y: min_y.floor() as i32,
            max_y: max_y.ceil() as i32,
        }
    }

    pub fn from_vertices_3d(vertices: &[Vec3]) -> Self {
        if vertices.is_empty() {
            return Self::default();
        }

        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for vertex in vertices {
            min_x = min_x.min(vertex.x);
            max_x = max_x.max(vertex.x);
            min_y = min_y.min(vertex.y);
            max_y = max_y.max(vertex.y);
        }

        Self {
            min_x: min_x.floor() as i32,
            max_x: max_x.ceil() as i32,
            min_y: min_y.floor() as i32,
            max_y: max_y.ceil() as i32,
        }
    }
}
