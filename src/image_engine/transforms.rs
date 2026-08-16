pub fn flip_horizontal(pixels: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut out = vec![0u8; pixels.len()];
    for y in 0..height {
        for x in 0..width {
            let src_idx = (y * width + x) * 4;
            let dst_idx = (y * width + (width - 1 - x)) * 4;
            out[dst_idx..dst_idx + 4].copy_from_slice(&pixels[src_idx..src_idx + 4]);
        }
    }
    out
}

pub fn flip_vertical(pixels: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut out = vec![0u8; pixels.len()];
    for y in 0..height {
        let src_row = y * width * 4;
        let dst_row = (height - 1 - y) * width * 4;
        out[dst_row..dst_row + width * 4].copy_from_slice(&pixels[src_row..src_row + width * 4]);
    }
    out
}

pub fn rotate_clockwise(pixels: &[u8], width: usize, height: usize) -> (Vec<u8>, usize, usize) {
    let new_width = height;
    let new_height = width;
    let mut out = vec![0u8; pixels.len()];

    for y in 0..height {
        for x in 0..width {
            let src_idx = (y * width + x) * 4;
            let new_x = height - 1 - y;
            let new_y = x;
            let dst_idx = (new_y * new_width + new_x) * 4;
            out[dst_idx..dst_idx + 4].copy_from_slice(&pixels[src_idx..src_idx + 4]);
        }
    }
    (out, new_width, new_height)
}

pub fn apply_sobel_edge_detection(pixels: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut out = vec![255u8; pixels.len()];
    let w = width as i32;
    let h = height as i32;

    let gx = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    let gy = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];

    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            let mut sum_x = 0i32;
            let mut sum_y = 0i32;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let idx = (((y + dy) * w + (x + dx)) * 4) as usize;
                    let gray = (pixels[idx] as i32 + pixels[idx + 1] as i32 + pixels[idx + 2] as i32) / 3;
                    let kx = gx[(dy + 1) as usize][(dx + 1) as usize];
                    let ky = gy[(dy + 1) as usize][(dx + 1) as usize];

                    sum_x += gray * kx;
                    sum_y += gray * ky;
                }
            }

            let magnitude = ((sum_x * sum_x + sum_y * sum_y) as f32).sqrt().clamp(0.0, 255.0) as u8;
            let out_idx = ((y * w + x) * 4) as usize;
            out[out_idx] = magnitude;
            out[out_idx + 1] = magnitude;
            out[out_idx + 2] = magnitude;
            out[out_idx + 3] = 255;
        }
    }
    out
}