pub type FrameBuffer = Vec<u32>;

pub struct ScreenBuffer {
    pub h: u32,
    pub w: u32,
    buffer: FrameBuffer,
}

impl ScreenBuffer {
    pub fn new(w: u32, h: u32, color: Option<u32>) -> Self {
        Self {
            h,
            w,
            buffer: vec![color.unwrap_or(0u32); (w * h) as usize],
        }
    }

    pub fn clear(&mut self, color: Option<u32>) {
        self.buffer.fill(color.unwrap_or(0u32));
    }

    pub fn set_pixel_value(&mut self, x: u32, y: u32, color: u32) {
        if x >= self.w || y >= self.h {
            return;
        }

        let idx = y as usize * self.w as usize + x as usize;
        self.buffer[idx] = color;
    }

    pub fn pixels(&self) -> &[u32] {
        &self.buffer
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
