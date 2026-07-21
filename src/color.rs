use std::ops::{Add, Mul};

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
        color.to_rgb()
    }
}
