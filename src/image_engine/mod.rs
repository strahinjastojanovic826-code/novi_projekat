pub mod filters;
pub mod generator;
pub mod transforms;

#[derive(Clone)]
pub struct ImageState {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

pub struct QuantumImageEngine {
    pub current_image: ImageState,
    undo_stack: Vec<ImageState>,
    redo_stack: Vec<ImageState>,
    pub brightness: i32,
    pub contrast: f32,
    pub tint_r: u8,
    pub tint_g: u8,
    pub tint_b: u8,
}

impl QuantumImageEngine {
    pub fn new() -> Self {
        let w = 320;
        let h = 240;
        let pixels = generator::generate_gradient(w, h);

        Self {
            current_image: ImageState {
                width: w,
                height: h,
                pixels,
            },
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            brightness: 0,
            contrast: 1.0,
            tint_r: 255,
            tint_g: 255,
            tint_b: 255,
        }
    }

    fn push_undo(&mut self) {
        self.undo_stack.push(self.current_image.clone());
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.current_image.clone());
            self.current_image = prev;
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.current_image.clone());
            self.current_image = next;
        }
    }

    pub fn apply_grayscale(&mut self) {
        self.push_undo();
        filters::apply_grayscale(&mut self.current_image.pixels);
    }

    pub fn apply_invert(&mut self) {
        self.push_undo();
        filters::apply_invert(&mut self.current_image.pixels);
    }

    pub fn apply_sepia(&mut self) {
        self.push_undo();
        filters::apply_sepia(&mut self.current_image.pixels);
    }

    pub fn apply_blur(&mut self) {
        self.push_undo();
        self.current_image.pixels = filters::apply_box_blur(
            &self.current_image.pixels,
            self.current_image.width,
            self.current_image.height,
        );
    }

    pub fn apply_sobel(&mut self) {
        self.push_undo();
        self.current_image.pixels = transforms::apply_sobel_edge_detection(
            &self.current_image.pixels,
            self.current_image.width,
            self.current_image.height,
        );
    }

    pub fn rotate_90(&mut self) {
        self.push_undo();
        let (new_pixels, nw, nh) = transforms::rotate_clockwise(
            &self.current_image.pixels,
            self.current_image.width,
            self.current_image.height,
        );
        self.current_image.pixels = new_pixels;
        self.current_image.width = nw;
        self.current_image.height = nh;
    }

    pub fn flip_h(&mut self) {
        self.push_undo();
        self.current_image.pixels = transforms::flip_horizontal(
            &self.current_image.pixels,
            self.current_image.width,
            self.current_image.height,
        );
    }

    pub fn flip_v(&mut self) {
        self.push_undo();
        self.current_image.pixels = transforms::flip_vertical(
            &self.current_image.pixels,
            self.current_image.width,
            self.current_image.height,
        );
    }

    pub fn load_mandelbrot(&mut self) {
        self.push_undo();
        self.current_image.width = 320;
        self.current_image.height = 240;
        self.current_image.pixels = generator::generate_mandelbrot(320, 240, 1.0);
    }

    pub fn load_checkerboard(&mut self) {
        self.push_undo();
        self.current_image.width = 320;
        self.current_image.height = 240;
        self.current_image.pixels = generator::generate_checkerboard(320, 240, 20);
    }

    pub fn load_gradient(&mut self) {
        self.push_undo();
        self.current_image.width = 320;
        self.current_image.height = 240;
        self.current_image.pixels = generator::generate_gradient(320, 240);
    }

    pub fn apply_color_adjustments(&mut self) {
        self.push_undo();
        filters::apply_brightness_contrast(&mut self.current_image.pixels, self.brightness, self.contrast);
        filters::apply_color_tint(&mut self.current_image.pixels, self.tint_r, self.tint_g, self.tint_b);
    }
}