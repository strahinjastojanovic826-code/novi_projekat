use super::dsp::{DspParams, Waveform};
use std::f32::consts::PI;

pub struct QuantumSynthesizer {
    pub sample_rate: u32,
    phase: f32,
}

impl QuantumSynthesizer {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            phase: 0.0,
        }
    }

//Izvini shakira sto nema ovde za pravljenje muzike
//Gledaj sa vedrije strane to
//Nema vise ima ili nema muzike nego
//Nema pa radis pa doradis pa ima

    /// Izračunava sledeći audio sample (-1.0 do 1.0)
    pub fn next_sample(&mut self, params: &DspParams) -> f32 {
        if params.volume <= 0.001 {
            return 0.0;
        }

        let phase_step = params.frequency / self.sample_rate as f32;
        self.phase = (self.phase + phase_step) % 1.0;

        let raw_sample = match params.waveform {
            Waveform::Sine => (self.phase * 2.0 * PI).sin(),
            Waveform::Square => if self.phase < 0.5 { 0.7 } else { -0.7 },
            Waveform::Sawtooth => 2.0 * self.phase - 1.0,
            Waveform::Noise => (pseudo_random(self.phase) * 2.0) - 1.0,
        };

        // Meki Low-pass filter matematička aproksimacija
        let filtered = raw_sample * (params.cutoff_freq / 8000.0).clamp(0.1, 1.0);

        filtered * params.volume
    }

    /// Generiše pun bafer zvučnih semplova za izlaz
    pub fn generate_buffer(&mut self, params: &DspParams, buffer_size: usize) -> Vec<f32> {
        let mut buffer = Vec::with_capacity(buffer_size);
        for _ in 0..buffer_size {
            buffer.push(self.next_sample(params));
        }
        buffer
    }
}

/// Matematički pseudo-random generator (bez spoljnih biblioteka) za šum
fn pseudo_random(seed: f32) -> f32 {
    let x = (seed * 12.9898 + 78.233).sin() * 43758.5453;
    x - x.floor()
}