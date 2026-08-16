// =============================================================================
// 1. BOJE I BAFERI
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0 }
    }

    pub fn clamp(&self, min: Color, max: Color) -> Color {
        Color::new(
            self.r.clamp(min.r, max.r),
            self.g.clamp(min.g, max.g),
            self.b.clamp(min.b, max.b),
        )
    }

    pub fn lerp(&self, other: Color, t: f32) -> Color {
        Color::new(
            self.r + (other.r - self.r) * t,
            self.g + (other.g - self.g) * t,
            self.b + (other.b - self.b) * t,
        )
    }
}

pub struct ImageBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x]
        } else {
            Color::black()
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }
}

// =============================================================================
// 2. HALTON SEKVENCA (Za TAA Sub-Pixel Podrhtavanje / Jitter)
// =============================================================================

pub struct HaltonSequence;

impl HaltonSequence {
    /// Generiše nisko-discrepantne kvazi-nasumične tačke za TAA jittering
    pub fn sample(index: usize, base: usize) -> f32 {
        let mut f = 1.0;
        let mut r = 0.0;
        let mut i = index;

        while i > 0 {
            f /= base as f32;
            r += f * (i % base) as f32;
            i /= base;
        }

        r - 0.5 // Centriramo u opseg [-0.5, 0.5]
    }
}

// =============================================================================
// 3. TEMPORAL ANTI-ALIASING (TAA) & NEURAL UPSCALER ENGINE
// =============================================================================

pub struct QuantumTemporalUpscaler {
    pub high_res_width: usize,
    pub high_res_height: usize,
    history_buffer: ImageBuffer,
    frame_index: usize,
}

impl QuantumTemporalUpscaler {
    pub fn new(high_res_width: usize, high_res_height: usize) -> Self {
        Self {
            high_res_width,
            high_res_height,
            history_buffer: ImageBuffer::new(high_res_width, high_res_height),
            frame_index: 0,
        }
    }

    /// Generiše trenutni pod-pikselni offset za kameru
    pub fn get_subpixel_jitter(&self) -> (f32, f32) {
        let jitter_x = HaltonSequence::sample((self.frame_index % 16) + 1, 2);
        let jitter_y = HaltonSequence::sample((self.frame_index % 16) + 1, 3);
        (jitter_x, jitter_y)
    }

    /// Glavni algoritam: Uzima Low-Res sliku i vrši TAA + Prostorno-Vremensko Uvećanje u High-Res
    pub fn process_frame(
        &mut self,
        low_res_frame: &ImageBuffer,
        motion_vector: (isize, isize), // Simulacija kretanja piksela
    ) -> ImageBuffer {
        let mut output_frame = ImageBuffer::new(self.high_res_width, self.high_res_height);
        let scale_x = low_res_frame.width as f32 / self.high_res_width as f32;
        let scale_y = low_res_frame.height as f32 / self.high_res_height as f32;

        let alpha = 0.15; // Težina novog frejma (15% novo, 85% akumulacija iz prošlosti)

        for hy in 0..self.high_res_height {
            for hx in 0..self.high_res_width {
                // 1. Bilinearno/Edge-guided uzorkovanje iz Low-Res bafera
                let lx_f = hx as f32 * scale_x;
                let ly_f = hy as f32 * scale_y;
                let lx = lx_f as usize;
                let ly = ly_f as usize;

                let current_color = low_res_frame.get_pixel(lx, ly);

                // 2. Određivanje Bounding Box-a komšiluka u Low-Res sluci (Za Anti-Ghosting Color Clamping)
                let mut min_color = current_color;
                let mut max_color = current_color;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = (lx as isize + dx).clamp(0, low_res_frame.width as isize - 1) as usize;
                        let ny = (ly as isize + dy).clamp(0, low_res_frame.height as isize - 1) as usize;
                        let neighbor = low_res_frame.get_pixel(nx, ny);

                        min_color.r = min_color.r.min(neighbor.r);
                        min_color.g = min_color.g.min(neighbor.g);
                        min_color.b = min_color.b.min(neighbor.b);

                        max_color.r = max_color.r.max(neighbor.r);
                        max_color.g = max_color.g.max(neighbor.g);
                        max_color.b = max_color.b.max(neighbor.b);
                    }
                }

                // 3. Reprojekcija u History Buffer preko Motion Vector-a
                let prev_hx = hx as isize - motion_vector.0;
                let prev_hy = hy as isize - motion_vector.1;

                let history_color = if prev_hx >= 0
                    && prev_hx < self.high_res_width as isize
                    && prev_hy >= 0
                    && prev_hy < self.high_res_height as isize
                {
                    self.history_buffer
                        .get_pixel(prev_hx as usize, prev_hy as usize)
                } else {
                    current_color
                };

                // 4. Color Clamping: Sečemo staru boju ako izlazi iz opsega komšiluka (Uklanja duhove/ghosting!)
                let clamped_history = history_color.clamp(min_color, max_color);

                // 5. Temporal Blending (Spajanje sadašnjosti i prošlosti)
                let final_color = clamped_history.lerp(current_color, alpha);

                output_frame.set_pixel(hx, hy, final_color);
            }
        }

        // Ažuriramo istorijski bafer za sledeći frejm
        self.history_buffer.pixels.copy_from_slice(&output_frame.pixels);
        self.frame_index += 1;

        output_frame
    }
}