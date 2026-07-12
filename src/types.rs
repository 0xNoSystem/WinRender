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
        if x >= self.w as i32 || y >= self.h as i32 {
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

    pub fn fill_triangle(&mut self, tri: Triangle, fill: TriangleFillType) {
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

                self.set_pixel_value(x, y, color);
            }
        }
    }

    
}

//3D
impl ScreenBuffer{
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

    pub fn fill_triangle_3d(&mut self, tri: Triangle3, fill: TriangleFillType){
        if tri.is_degenerate() {
            return;
        }
        let Some(bbox) = tri.bounding_rect().clamp(self.w, self.h) else {
            return;
        };

        let screen_tri = tri.to_screen_triangle();
        let area = edge(screen_tri.p1, screen_tri.p2, screen_tri.p0);

        for y in bbox.min_y..=bbox.max_y {
            for x in bbox.min_x..=bbox.max_x {
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);

                if !tri.contains_point(p) {
                    continue;
                }

                let w0 = edge(screen_tri.p1, screen_tri.p2, p) / area;
                let w1 = edge(screen_tri.p2, screen_tri.p0, p) / area;
                let w2 = edge(screen_tri.p0, screen_tri.p1, p) / area;

                let color = match fill {
                    TriangleFillType::Solid(color) => color,

                    TriangleFillType::Gradient { c0, c1, c2 } => (c0 * w0 + c1 * w1 + c2 * w2).to_u32(),
                };
                let z = tri.p0.z * w0 + tri.p1.z * w1 + tri.p2.z * w2;

                self.set_pixel_depth(x, y,z, color);
            }
        }

    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TriangleFillType {
    Solid(u32),
    Gradient {
        c0: ColorRgb,
        c1: ColorRgb,
        c2: ColorRgb,
    },
}

impl Default for TriangleFillType {
    fn default() -> Self {
        Self::Solid(Color::White as u32)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorRgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl ColorRgb {
    pub fn to_u32(self) -> u32 {
        let r = self.r.clamp(0.0, 255.0) as u32;
        let g = self.g.clamp(0.0, 255.0) as u32;
        let b = self.b.clamp(0.0, 255.0) as u32;

        (r << 16) | (g << 8) | b
    }
}

impl Add for ColorRgb {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul<f32> for ColorRgb {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black = 0x00000000,
    White = 0x00FFFFFF,
    Red = 0x00FF0000,
    Green = 0x0000FF00,
    Blue = 0x000000FF,
    Yellow = 0x00FFFF00,
    Cyan = 0x0000FFFF,
    Magenta = 0x00FF00FF,
    Gray = 0x00808080,
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        color as u32
    }
}

impl Color {
    pub fn to_rgb(self) -> ColorRgb {
        let value = self as u32;

        ColorRgb {
            r: ((value >> 16) & 0xFF) as f32,
            g: ((value >> 8) & 0xFF) as f32,
            b: (value & 0xFF) as f32,
        }
    }
}

impl From<Color> for ColorRgb {
    fn from(color: Color) -> Self {
        let value = color as u32;

        Self {
            r: ((value >> 16) & 0xFF) as f32,
            g: ((value >> 8) & 0xFF) as f32,
            b: (value & 0xFF) as f32,
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
        let mut p0 = self.p0;
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
}

#[derive(Clone, Copy, Debug)]
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
}

fn edge(a: Vec2, b: Vec2, p: Vec2) -> f32 {
    (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)
}

fn is_top_left_edge(a: Vec2, b: Vec2) -> bool {
    let dx = b.x - a.x;
    let dy = b.y - a.y;

    dy < 0.0 || (dy == 0.0 && dx > 0.0)
}

use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(self, rhs: Self) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalized(self) -> Self {
        let length = self.length();

        if length == 0.0 {
            return Self::default();
        }

        self / length
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        Self::new(self.x / scalar, self.y / scalar)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalized(self) -> Self {
        let length = self.length();

        if length == 0.0 {
            return Self::default();
        }

        self / length
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}
