pub mod dsp;
pub mod synth;

pub use dsp::DspParams;
pub use synth::QuantumSynthesizer;

use crate::driver::WinQuantumDriver;

pub struct QuantumAudioEngine {
    pub synth: QuantumSynthesizer,
    pub is_enabled: bool,
    pub current_params: DspParams,
}

impl QuantumAudioEngine {
    pub fn new() -> Self {
        let synth = QuantumSynthesizer::new(44100);
        let default_params = DspParams::from_register(0);

        Self {
            synth,
            is_enabled: false,
            current_params: default_params,
        }
    }

    /// Osvježava audio parametre čitanjem stanja sa atomskog hardware registra
    pub fn sync_with_hardware(&mut self, driver: &WinQuantumDriver) {
        let reg = driver.read_register();
        self.current_params = DspParams::from_register(reg);
    }

    /// Generiše PCM zvučni bafer ako je audio uključen u OS-u
    pub fn render_audio_frame(&mut self, driver: &WinQuantumDriver, samples: usize) -> Vec<f32> {
        self.sync_with_hardware(driver);
        if !self.is_enabled {
            return vec![0.0; samples];
        }
        self.synth.generate_buffer(&self.current_params, samples)
    }
}