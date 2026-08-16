pub mod framebuffer;

use framebuffer::{Color, DisplayMode, Framebuffer, PixelFormat};

pub struct QuantumGopVgaEngine {
    pub active_mode: DisplayMode,
    pub available_modes: Vec<DisplayMode>,
    pub framebuffer: Framebuffer,
    pub framebuffer_base_address: u64,
    pub is_vga_legacy_fallback: bool,
    pub logs: Vec<String>,
}

impl QuantumGopVgaEngine {
    pub fn new() -> Self {
        let modes = vec![
            DisplayMode {
                id: 0,
                width: 1920,
                height: 1080,
                pixels_per_scan_line: 1920,
                pixel_format: PixelFormat::Rgb8,
                name: "1080p Full HD (UEFI GOP)".into(),
            },
            DisplayMode {
                id: 1,
                width: 1280,
                height: 720,
                pixels_per_scan_line: 1280,
                pixel_format: PixelFormat::Rgb8,
                name: "720p HD (UEFI GOP)".into(),
            },
            DisplayMode {
                id: 2,
                width: 1024,
                height: 768,
                pixels_per_scan_line: 1024,
                pixel_format: PixelFormat::Bgr8,
                name: "XGA Standard (UEFI GOP)".into(),
            },
            DisplayMode {
                id: 3,
                width: 320,
                height: 200,
                pixels_per_scan_line: 320,
                pixel_format: PixelFormat::VgaPalette256,
                name: "VGA Mode 13h (Legacy Standard)".into(),
            },
        ];

        let default_mode = modes[0].clone();
        let fb = Framebuffer::new(240, 135); // Simulacija minijaturnog preview bafera za UI
        
        let mut engine = Self {
            active_mode: default_mode,
            available_modes: modes,
            framebuffer: fb,
            framebuffer_base_address: 0xE000_0000, // Simulirana BAR0 fizicka adresa
            is_vga_legacy_fallback: false,
            logs: Vec::new(),
        };

        engine.draw_test_pattern();
        engine.logs.push("GOP Drajver: Inicijalizovan UEFI Framebuffer na adresi 0xE0000000.".into());
        engine
    }

    pub fn set_mode(&mut self, mode_id: u32) -> Result<String, String> {
        if let Some(mode) = self.available_modes.iter().find(|m| m.id == mode_id).cloned() {
            self.active_mode = mode.clone();
            self.is_vga_legacy_fallback = mode.pixel_format == PixelFormat::VgaPalette256;
            
            self.draw_test_pattern();

            let msg = format!("Promenjen grafički mod u ID {}: {} ({}x{})", mode.id, mode.name, mode.width, mode.height);
            self.logs.push(msg.clone());
            Ok(msg)
        } else {
            Err(format!("Greška: Grafički mod ID {} nije podržan!", mode_id))
        }
    }

    pub fn draw_test_pattern(&mut self) {
        self.framebuffer.clear(Color::BLACK);

        let w = self.framebuffer.width;
        let h = self.framebuffer.height;

        // Crtanje Color-Bar šeme
        let bar_width = w / 5;
        self.framebuffer.draw_rect(0, 0, bar_width, h, Color::RED);
        self.framebuffer.draw_rect(bar_width, 0, bar_width, h, Color::GREEN);
        self.framebuffer.draw_rect(bar_width * 2, 0, bar_width, h, Color::BLUE);
        self.framebuffer.draw_rect(bar_width * 3, 0, bar_width, h, Color::CYAN);
        self.framebuffer.draw_rect(bar_width * 4, 0, w - (bar_width * 4), h, Color::WHITE);
    }
}