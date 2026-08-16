pub fn generate_mandelbrot(width: usize, height: usize, zoom: f32) -> Vec<u8> {
    let mut pixels = vec![255u8; width * height * 4];
    let max_iterations = 60;

    for y in 0..height {
        for x in 0..width {
            let cx = (x as f32 - width as f32 / 2.0) / (0.5 * zoom * width as f32);
            let cy = (y as f32 - height as f32 / 2.0) / (0.5 * zoom * height as f32);

            let mut zx = 0.0f32;
            let mut zy = 0.0f32;
            let mut iter = 0;

            while zx * zx + zy * zy <= 4.0 && iter < max_iterations {
                let temp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = temp;
                iter += 1;
            }

            let idx = (y * width + x) * 4;
            if iter == max_iterations {
                pixels[idx] = 0;
                pixels[idx + 1] = 0;
                pixels[idx + 2] = 0;
            } else {
                pixels[idx] = (iter * 12 % 255) as u8;
                pixels[idx + 1] = (iter * 7 % 255) as u8;
                pixels[idx + 2] = (iter * 18 % 255) as u8;
            }
            pixels[idx + 3] = 255;
        }
    }
    pixels
}

pub fn generate_checkerboard(width: usize, height: usize, square_size: usize) -> Vec<u8> {
    let mut pixels = vec![255u8; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4;
            let is_dark = ((x / square_size) + (y / square_size)) % 2 == 0;
            let color = if is_dark { 40 } else { 220 };

            pixels[idx] = color;
            pixels[idx + 1] = color;
            pixels[idx + 2] = color;
            pixels[idx + 3] = 255;
        }
    }
    pixels
}

pub fn generate_gradient(width: usize, height: usize) -> Vec<u8> {
    let mut pixels = vec![255u8; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4;
            pixels[idx] = ((x * 255) / width) as u8;
            pixels[idx + 1] = ((y * 255) / height) as u8;
            pixels[idx + 2] = 180;
            pixels[idx + 3] = 255;
        }
    }
    pixels
}