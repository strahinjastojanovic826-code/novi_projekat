#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Waveform {
    Sine,     // Čist ton (Sinus)
    Square,   // 8-bit Chiptune (Kvadratni talas)
    Sawtooth, // Oštar sintisajzer (Testerasti talas)
    Noise,    // Kvantni šum / Buka
}

#[derive(Debug, Clone, Copy)]
pub struct DspParams {
    pub frequency: f32,   // Osnovna frekvencija u Hz (20Hz - 4000Hz)
    pub waveform: Waveform,
    pub volume: f32,      // Jačina zvuka (0.0 - 1.0)
    pub cutoff_freq: f32, // Low-pass filter (200Hz - 8000Hz)
}

impl DspParams {
    /// Dekodira u64 registar (32 kvata) u DSP zvučne parametre
    pub fn from_register(reg: u64) -> Self {
        // Bitovi 0..15 (Kvati Q00-Q07): Pitch (Frekvencija)
        let pitch_raw = (reg & 0xFFFF) as f32;
        let frequency = 60.0 + (pitch_raw / 65535.0) * 3000.0; // Od 60Hz do 3060Hz

        // Bitovi 16..23 (Kvati Q08-Q11): Oblik zvučnog talasa
        let wave_raw = ((reg >> 16) & 0xFF) % 4;
        let waveform = match wave_raw {
            0 => Waveform::Sine,
            1 => Waveform::Square,
            2 => Waveform::Sawtooth,
            _ => Waveform::Noise,
        };

        // Bitovi 24..31 (Kvati Q12-Q15): Volume (Glasnoća)
        let vol_raw = ((reg >> 24) & 0xFF) as f32;
        let volume = (vol_raw / 255.0).clamp(0.0, 1.0);

        // Bitovi 32..47 (Kvati Q16-Q23): Cutoff Filter
        let cutoff_raw = ((reg >> 32) & 0xFFFF) as f32;
        let cutoff_freq = 200.0 + (cutoff_raw / 65535.0) * 7800.0;

        Self {
            frequency,
            waveform,
            volume,
            cutoff_freq,
        }
    }
}