pub fn apply_grayscale(pixels: &mut [u8]) {
    for chunk in pixels.chunks_exact_mut(4) {
        let gray = (chunk[0] as u32 * 30 + chunk[1] as u32 * 59 + chunk[2] as u32 * 11) / 100;
        let g = gray as u8;
        chunk[0] = g;
        chunk[1] = g;
        chunk[2] = g;
    }
}

pub fn apply_invert(pixels: &mut [u8]) {
    for chunk in pixels.chunks_exact_mut(4) {
        chunk[0] = 255 - chunk[0];
        chunk[1] = 255 - chunk[1];
        chunk[2] = 255 - chunk[2];
    }
}

pub fn apply_sepia(pixels: &mut [u8]) {
    for chunk in pixels.chunks_exact_mut(4) {
        let r = chunk[0] as f32;
        let g = chunk[1] as f32;
        let b = chunk[2] as f32;

        chunk[0] = (r * 0.393 + g * 0.769 + b * 0.189).min(255.0) as u8;
        chunk[1] = (r * 0.349 + g * 0.686 + b * 0.168).min(255.0) as u8;
        chunk[2] = (r * 0.272 + g * 0.534 + b * 0.131).min(255.0) as u8;
    }
}

pub fn apply_brightness_contrast(pixels: &mut [u8], brightness: i32, contrast: f32) {
    for chunk in pixels.chunks_exact_mut(4) {
        for i in 0..3 {
            let mut val = chunk[i] as f32;
            val = (val - 128.0) * contrast + 128.0 + brightness as f32;
            chunk[i] = val.clamp(0.0, 255.0) as u8;
        }
    }
}

pub fn apply_color_tint(pixels: &mut [u8], tint_r: u8, tint_g: u8, tint_b: u8) {
    for chunk in pixels.chunks_exact_mut(4) {
        chunk[0] = ((chunk[0] as u32 * tint_r as u32) / 255) as u8;
        chunk[1] = ((chunk[1] as u32 * tint_g as u32) / 255) as u8;
        chunk[2] = ((chunk[2] as u32 * tint_b as u32) / 255) as u8;
    }
}

pub fn apply_box_blur(pixels: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut result = pixels.to_vec();
    let w = width as i32;
    let h = height as i32;

    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let idx = (((y + dy) * w + (x + dx)) * 4) as usize;
                    r_sum += pixels[idx] as u32;
                    g_sum += pixels[idx + 1] as u32;
                    b_sum += pixels[idx + 2] as u32;
                }
            }

            let out_idx = ((y * w + x) * 4) as usize;
            result[out_idx] = (r_sum / 9) as u8;
            result[out_idx + 1] = (g_sum / 9) as u8;
            result[out_idx + 2] = (b_sum / 9) as u8;
        }
    }
    result
}