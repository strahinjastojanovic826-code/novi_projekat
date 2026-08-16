#[derive(Debug, Clone, PartialEq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

pub struct VideoFrame {
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
}

pub struct VideoEngine {
    pub current_video: String,
    pub state: PlaybackState,
    pub current_frame: usize,
    pub total_frames: usize,
    pub fps: u32,
    pub width: usize,
    pub height: usize,
    pub available_videos: Vec<String>,
}

impl VideoEngine {
    pub fn new() -> Self {
        Self {
            current_video: "quantum_intro.qvid".to_string(),
            state: PlaybackState::Stopped,
            current_frame: 0,
            total_frames: 300,
            fps: 30,
            width: 320,
            height: 180,
            available_videos: vec![
                "quantum_intro.qvid".to_string(),
                "matrix_stream.qvid".to_string(),
                "smpte_test_bars.qvid".to_string(),
            ],
        }
    }

    pub fn play(&mut self) {
        self.state = PlaybackState::Playing;
    }

    pub fn pause(&mut self) {
        self.state = PlaybackState::Paused;
    }

//Jebi se binarni sistemu
//Jebi se Jebi se Jebi se Jebi se Jebi se Jebi se
//JEBIIII SEEEE!!!!!!

    pub fn stop(&mut self) {
        self.state = PlaybackState::Stopped;
        self.current_frame = 0;
    }

    pub fn load_video(&mut self, name: &str) {
        self.current_video = name.to_string();
        self.stop();
    }

    // Dekoder: Generiše i dekodira RGBA kompresovane frejmove u realnom vremenu
    pub fn decode_next_frame(&mut self) -> VideoFrame {
        if self.state == PlaybackState::Playing {
            self.current_frame = (self.current_frame + 1) % self.total_frames;
        }

        let mut rgba = vec![0u8; self.width * self.height * 4];
        let frame_idx = self.current_frame;

        match self.current_video.as_str() {
            "matrix_stream.qvid" => {
                // Dekodiranje "Matrix Rain" efekta
                for y in 0..self.height {
                    for x in 0..self.width {
                        let idx = (y * self.width + x) * 4;
                        let green_intensity = ((x * 7 + y * 13 + frame_idx * 5) % 255) as u8;
                        rgba[idx] = 0;                     // Red
                        rgba[idx + 1] = green_intensity;   // Green
                        rgba[idx + 2] = 20;                // Blue
                        rgba[idx + 3] = 255;               // Alpha
                    }
                }
            }
            "smpte_test_bars.qvid" => {
                // Dekodiranje SMPTE TV kolor linija sa pomeranjem
                for y in 0..self.height {
                    for x in 0..self.width {
                        let idx = (y * self.width + x) * 4;
                        let section = (x + frame_idx) % self.width / (self.width / 7 + 1);
                        let (r, g, b) = match section {
                            0 => (200, 200, 200),
                            1 => (200, 200, 0),
                            2 => (0, 200, 200),
                            3 => (0, 200, 0),
                            4 => (200, 0, 200),
                            5 => (200, 0, 0),
                            _ => (0, 0, 200),
                        };
                        rgba[idx] = r;
                        rgba[idx + 1] = g;
                        rgba[idx + 2] = b;
                        rgba[idx + 3] = 255;
                    }
                }
            }
            _ => {
                // Podrazumevani "Quantum Ball" animirani demo
                let ball_x = ((frame_idx * 4) % self.width) as i32;
                let ball_y = (self.height / 2) as i32 + ((frame_idx as f32 * 0.1).sin() * 40.0) as i32;

                for y in 0..self.height {
                    for x in 0..self.width {
                        let idx = (y * self.width + x) * 4;
                        let dx = x as i32 - ball_x;
                        let dy = y as i32 - ball_y;
                        let dist_sq = dx * dx + dy * dy;

                        if dist_sq < 400 {
                            // Loptica
                            rgba[idx] = 0;
                            rgba[idx + 1] = 220;
                            rgba[idx + 2] = 255;
                            rgba[idx + 3] = 255;
                        } else {
                            // Pozadina
                            rgba[idx] = 15;
                            rgba[idx + 1] = 18;
                            rgba[idx + 2] = 28;
                            rgba[idx + 3] = 255;
                        }
                    }
                }
            }
        }

        VideoFrame {
            width: self.width,
            height: self.height,
            rgba,
        }
    }
}