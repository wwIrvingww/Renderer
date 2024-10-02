use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Color {
  r: u8,
  g: u8,
  b: u8,
}

impl Color {
  // Constructor to initialize the color using r, g, b values as u8
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Color { r, g, b }
  }

  // default color
  pub fn black() -> Self {
    Color { r: 0, g: 0, b: 0 }
  }

  // New constructor to initialize the color using r, g, b values as f32 (0.0 to 1.0)
  pub fn from_float(r: f32, g: f32, b: f32) -> Self {
    Color {
      r: (r.clamp(0.0, 1.0) * 255.0) as u8,
      g: (g.clamp(0.0, 1.0) * 255.0) as u8,
      b: (b.clamp(0.0, 1.0) * 255.0) as u8,
    }
  }

  // Function to create a color from a hex value
  pub fn from_hex(hex: u32) -> Self {
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    Color { r, g, b }
  }

  // Function to return the color as a hex value
  pub fn to_hex(&self) -> u32 {
    ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
  }

  // Lerp function for linear interpolation between two colors
  pub fn lerp(a: &Color, b: &Color, t: f32) -> Color {
    let r = a.r as f32 + t * (b.r as f32 - a.r as f32);
    let g = a.g as f32 + t * (b.g as f32 - a.g as f32);
    let b = a.b as f32 + t * (b.b as f32 - a.b as f32);
    Color {
      r: r.clamp(0.0, 255.0) as u8,
      g: g.clamp(0.0, 255.0) as u8,
      b: b.clamp(0.0, 255.0) as u8,
    }
  }
}
