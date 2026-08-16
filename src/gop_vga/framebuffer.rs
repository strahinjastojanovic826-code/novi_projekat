#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const RED: Color = Color { r: 230, g: 50, b: 50, a: 255 };
    pub const GREEN: Color = Color { r: 40, g: 200, b: 80, a: 255 };
    pub const BLUE: Color = Color { r: 50, g: 120, b: 240, a: 255 };
    pub const CYAN: Color = Color { r: 0, g: 200, b: 220, a: 255 };
}

#[derive(Debug, Clone, PartialEq)]
pub enum PixelFormat {
    Rgb8,
    Bgr8,
    VgaPalette256,
}

#[derive(Debug, Clone)]
pub struct DisplayMode {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub pixels_per_scan_line: u32,
    pub pixel_format: PixelFormat,
    pub name: String,
}

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>, // RGBA 4 bajta po pikselu za renderovanje
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize;
        Self {
            width,
            height,
            buffer: vec![0; size],
        }
    }

    pub fn clear(&mut self, color: Color) {
        for chunk in self.buffer.chunks_exact_mut(4) {
            chunk[0] = color.r;
            chunk[1] = color.g;
            chunk[2] = color.b;
            chunk[3] = color.a;
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            let offset = ((y * self.width + x) * 4) as usize;
            if offset + 3 < self.buffer.len() {
                self.buffer[offset] = color.r;
                self.buffer[offset + 1] = color.g;
                self.buffer[offset + 2] = color.b;
                self.buffer[offset + 3] = color.a;
            }
        }
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color) {
        for py in y..(y + h).min(self.height) {
            for px in x..(x + w).min(self.width) {
                self.set_pixel(px, py, color);
            }
        }
    }
}