use std::f32::consts::TAU;

use super::Mesh;
use crate::math::{BoundingBox, Vec2, Vec3};

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    FlatTriangle(Triangle),
    Stroke(Stroke2D),
    Circle(Circle),
    Rectangle(Rect),
    Triangle(Triangle3),
    Sphere(Sphere),
    Cube(Cube),
    Torus(Torus),
    Mesh(Mesh),
}

impl Shape {
    pub fn mesh(self) -> Mesh {
        match self {
            Self::FlatTriangle(triangle) => triangle.into(),
            Self::Stroke(stroke) => stroke.into(),
            Self::Circle(circle) => circle.into(),
            Self::Rectangle(rect) => rect.into(),
            Self::Triangle(triangle) => triangle.into(),
            Self::Sphere(sphere) => sphere.into(),
            Self::Cube(cube) => cube.into(),
            Self::Torus(torus) => torus.into(),
            Self::Mesh(mesh) => mesh,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub p0: Vec2,
    pub p1: Vec2,
    pub p2: Vec2,
}

impl Triangle {
    pub fn new(p0: Vec2, p1: Vec2, p2: Vec2) -> Self {
        Self { p0, p1, p2 }
    }

    pub fn bounding_rect(self) -> BoundingBox {
        BoundingBox {
            min_x: self.p0.x.min(self.p1.x).min(self.p2.x).floor() as i32,
            max_x: self.p0.x.max(self.p1.x).max(self.p2.x).ceil() as i32,
            min_y: self.p0.y.min(self.p1.y).min(self.p2.y).floor() as i32,
            max_y: self.p0.y.max(self.p1.y).max(self.p2.y).ceil() as i32,
        }
    }

    pub fn contains_point(self, p: Vec2) -> bool {
        let p0 = self.p0;
        let mut p1 = self.p1;
        let mut p2 = self.p2;

        // Normalize the winding so that interior edge values are negative.
        if edge(p0, p1, p2) > 0.0 {
            std::mem::swap(&mut p1, &mut p2);
        }

        let e0 = edge(p0, p1, p);
        let e1 = edge(p1, p2, p);
        let e2 = edge(p2, p0, p);

        let inside_e0 = e0 < 0.0 || (e0 == 0.0 && is_top_left_edge(p0, p1));
        let inside_e1 = e1 < 0.0 || (e1 == 0.0 && is_top_left_edge(p1, p2));
        let inside_e2 = e2 < 0.0 || (e2 == 0.0 && is_top_left_edge(p2, p0));

        inside_e0 && inside_e1 && inside_e2
    }

    pub fn signed_area_twice(self) -> f32 {
        (self.p1 - self.p0).cross(self.p2 - self.p0)
    }

    pub fn is_degenerate(self) -> bool {
        self.signed_area_twice().abs() < f32::EPSILON
    }

    pub fn mesh(self) -> Mesh {
        Mesh::new(
            vec![
                Vec3::new(self.p0.x, self.p0.y, 0.0),
                Vec3::new(self.p1.x, self.p1.y, 0.0),
                Vec3::new(self.p2.x, self.p2.y, 0.0),
            ],
            vec![[0, 1, 2]],
        )
    }
}

impl From<Triangle> for Mesh {
    fn from(triangle: Triangle) -> Self {
        triangle.mesh()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Line2D {
    pub p0: Vec2,
    pub p1: Vec2,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Stroke2D {
    pub line: Line2D,
    pub width: f32,
}

impl Stroke2D {
    pub fn new(p0: Vec2, p1: Vec2, width: f32) -> Self {
        Self {
            line: Line2D { p0, p1 },
            width,
        }
    }

    pub fn mesh(self) -> Mesh {
        let p0 = self.line.p0;
        let p1 = self.line.p1;

        let delta = p1 - p0;
        let length = (delta.x * delta.x + delta.y * delta.y).sqrt();

        if length == 0.0 || self.width <= 0.0 {
            return Mesh::default();
        }

        let normal = Vec2::new(-delta.y / length, delta.x / length);
        let offset = normal * (self.width * 0.5);
        let p0_upper = p0 + offset;
        let p0_lower = p0 - offset;
        let p1_upper = p1 + offset;
        let p1_lower = p1 - offset;

        Mesh::new(
            vec![
                Vec3::new(p0_upper.x, p0_upper.y, 0.0),
                Vec3::new(p0_lower.x, p0_lower.y, 0.0),
                Vec3::new(p1_upper.x, p1_upper.y, 0.0),
                Vec3::new(p1_lower.x, p1_lower.y, 0.0),
            ],
            vec![[0, 1, 2], [2, 1, 3]],
        )
    }
}

impl From<Stroke2D> for Mesh {
    fn from(stroke: Stroke2D) -> Self {
        stroke.mesh()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }

    pub fn segment_count(radius: f32) -> usize {
        const MAX_EDGE_LENGTH: f32 = 4.0;

        if radius <= 0.0 {
            return 0;
        }

        (TAU * radius / MAX_EDGE_LENGTH).ceil().max(3.0) as usize
    }

    pub fn mesh(self) -> Mesh {
        let segments = Self::segment_count(self.radius);

        if self.radius <= 0.0 || segments < 3 {
            return Mesh::default();
        }

        let mut vertices = Vec::with_capacity(segments + 1);
        let mut indices = Vec::with_capacity(segments);

        vertices.push(Vec3::ZERO);

        for i in 0..segments {
            let angle = TAU * i as f32 / segments as f32;

            vertices.push(Vec3::new(
                self.radius * angle.cos(),
                self.radius * angle.sin(),
                0.0,
            ));
        }

        for i in 0..segments {
            let current = i + 1;
            let next = ((i + 1) % segments) + 1;

            indices.push([0, current, next]);
        }

        Mesh::new(vertices, indices)
    }
}

impl From<Circle> for Mesh {
    fn from(circle: Circle) -> Self {
        circle.mesh()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self {
            min: Vec2::new(min.x.min(max.x), min.y.min(max.y)),
            max: Vec2::new(min.x.max(max.x), min.y.max(max.y)),
        }
    }

    pub fn mesh(self) -> Mesh {
        Mesh::new(
            vec![
                Vec3::new(self.min.x, self.min.y, 0.0),
                Vec3::new(self.max.x, self.min.y, 0.0),
                Vec3::new(self.max.x, self.max.y, 0.0),
                Vec3::new(self.min.x, self.max.y, 0.0),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        )
    }
}

impl From<Rect> for Mesh {
    fn from(rect: Rect) -> Self {
        rect.mesh()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle3 {
    pub p0: Vec3,
    pub p1: Vec3,
    pub p2: Vec3,
}

impl Triangle3 {
    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3) -> Self {
        Self { p0, p1, p2 }
    }

    pub fn to_screen_triangle(self) -> Triangle {
        Triangle::new(
            Vec2::new(self.p0.x, self.p0.y),
            Vec2::new(self.p1.x, self.p1.y),
            Vec2::new(self.p2.x, self.p2.y),
        )
    }

    pub fn bounding_rect(self) -> BoundingBox {
        self.to_screen_triangle().bounding_rect()
    }

    pub fn contains_point(self, p: Vec2) -> bool {
        self.to_screen_triangle().contains_point(p)
    }

    pub fn signed_area_twice(self) -> f32 {
        self.to_screen_triangle().signed_area_twice()
    }

    pub fn is_degenerate(self) -> bool {
        self.signed_area_twice().abs() < f32::EPSILON
    }
}

impl From<Triangle3> for Mesh {
    fn from(tri: Triangle3) -> Self {
        Mesh::new(vec![tri.p0, tri.p1, tri.p2], vec![[0, 1, 2]])
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Sphere {
    pub radius: f32,
    pub lat_steps: usize,
    pub long_steps: usize,
}

impl Sphere {
    pub fn mesh(self) -> Mesh {
        Mesh::uv_sphere(self.radius, self.lat_steps, self.long_steps)
    }
}

impl From<Sphere> for Mesh {
    fn from(sphere: Sphere) -> Self {
        sphere.mesh()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cube {
    pub size: f32,
}

impl Cube {
    pub fn mesh(self) -> Mesh {
        if self.size <= 0.0 {
            return Mesh::default();
        }

        let half = self.size * 0.5;
        let min_x = -half;
        let max_x = half;
        let min_y = -half;
        let max_y = half;
        let min_z = -half;
        let max_z = half;

        let vertices = vec![
            Vec3::new(min_x, min_y, min_z),
            Vec3::new(max_x, min_y, min_z),
            Vec3::new(max_x, max_y, min_z),
            Vec3::new(min_x, max_y, min_z),
            Vec3::new(min_x, min_y, max_z),
            Vec3::new(max_x, min_y, max_z),
            Vec3::new(max_x, max_y, max_z),
            Vec3::new(min_x, max_y, max_z),
        ];

        let indices = vec![
            [0, 2, 1],
            [0, 3, 2],
            [4, 5, 6],
            [4, 6, 7],
            [0, 1, 5],
            [0, 5, 4],
            [3, 6, 2],
            [3, 7, 6],
            [1, 2, 6],
            [1, 6, 5],
            [0, 7, 3],
            [0, 4, 7],
        ];

        Mesh::new(vertices, indices)
    }
}

impl From<Cube> for Mesh {
    fn from(cube: Cube) -> Self {
        cube.mesh()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Torus {
    pub major_radius: f32,
    pub minor_radius: f32,
    pub seg_u: usize,
    pub seg_v: usize,
    pub tilt: f32,
}

impl Torus {
    pub fn mesh(self) -> Mesh {
        Mesh::torus(
            self.major_radius,
            self.minor_radius,
            self.seg_u,
            self.seg_v,
            self.tilt,
        )
    }
}

impl From<Torus> for Mesh {
    fn from(torus: Torus) -> Self {
        torus.mesh()
    }
}

impl From<Shape> for Mesh {
    fn from(shape: Shape) -> Self {
        shape.mesh()
    }
}

pub(crate) fn edge(a: Vec2, b: Vec2, p: Vec2) -> f32 {
    (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)
}

fn is_top_left_edge(a: Vec2, b: Vec2) -> bool {
    let dx = b.x - a.x;
    let dy = b.y - a.y;

    dy < 0.0 || (dy == 0.0 && dx > 0.0)
}
