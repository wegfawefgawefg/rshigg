use glam::Vec2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn scaled(self, factor: f32) -> Self {
        let clamp = |value: f32| -> u8 { value.clamp(0.0, 255.0) as u8 };
        Self {
            r: clamp(self.r as f32 * factor),
            g: clamp(self.g as f32 * factor),
            b: clamp(self.b as f32 * factor),
            a: self.a,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub position: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self { position, size }
    }

    pub fn from_normalized(position: Vec2, size: Vec2, resolution: Vec2) -> Self {
        Self {
            position: position * resolution,
            size: size * resolution,
        }
    }
}

pub trait DrawBackend {
    fn fill_rect(&mut self, rect: Rect, color: Color);
    fn draw_line(&mut self, start: Vec2, end: Vec2, color: Color, thickness: f32);
    fn draw_text(&mut self, text: &str, position: Vec2, font_size: f32, color: Color);
}
