use std::f64::consts::PI;

// =============================================================================
// 1. HARDWARE DATA STRUCTURES: OPTICAL WAVES & WAVEGUIDES
// =============================================================================

/// Optički talas koji putuje kroz silicijumski talasovod
#[derive(Debug, Clone, Copy)]
pub struct OpticalWave {
    pub amplitude: f64,     // Intenzitet svetlosti A >= 0.0
    pub phase_rad: f64,     // Faza talasa φ u radijanima [0, 2π)
    pub wavelength_nm: f64, // Talasna dužina λ (npr. 1550nm - telekom opseg)
}

impl OpticalWave {
    pub fn zero() -> Self {
        Self {
            amplitude: 0.0,
            phase_rad: 0.0,
            wavelength_nm: 1550.0,
        }
    }

    /// Superpozicija (Superposition / Interference) dva optička talasa
    pub fn interfere_with(&self, other: &Self) -> Self {
        // Kompleksna reprezentacija talasa: E = A * (cos(φ) + i*sin(φ))
        let x1 = self.amplitude * self.phase_rad.cos();
        let y1 = self.amplitude * self.phase_rad.sin();

        let x2 = other.amplitude * other.phase_rad.cos();
        let y2 = other.amplitude * other.phase_rad.sin();

        let x_total = x1 + x2;
        let y_total = y1 + y2;

        let result_amp = (x_total * x_total + y_total * y_total).sqrt();
        let result_phase = y_total.atan2(x_total);

        Self {
            amplitude: result_amp,
            phase_rad: if result_phase < 0.0 { result_phase + 2.0 * PI } else { result_phase },
            wavelength_nm: self.wavelength_nm,
        }
    }
}

// =============================================================================
// 2. ESOTERIC PHOTONIC INSTRUCTIONS
// =============================================================================

#[derive(Debug, Clone)]
pub enum PhotonicInstruction {
    /// PUMP_LASER <channel> <amplitude> <phase_deg> - Pali laserski izvor
    PumpLaser { channel: usize, amplitude: f64, phase_deg: f64 },
    
    /// PHASE_SHIFT <channel> <degrees> - Greje grijač na talasovodu i pomera fazu za Δφ
    PhaseShift { channel: usize, shift_deg: f64 },

    /// ATTENUATE <channel> <factor> - Prigušuje svetlost (množenje skalarom < 1)
    Attenuate { channel: usize, factor: f64 },

    /// BEAM_SPLITTER <in1> <in2> <out1> <out2> - 50/50 Optički delilac snopa
    BeamSplitter { in1: usize, in2: usize, out1: usize, out2: usize },

    /// MZI_ROTATE <in1> <in2> <theta_deg> <phi_deg> - Mach-Zehnder Interferometar Unitarni Množač
    MziRotate { in1: usize, in2: usize, theta_deg: f64, phi_deg: f64 },

    /// PHOTODETECT <channel> - Prevara fotona u digitalni napon (ADC Photodiode Readout)
    Photodetect { channel: usize },
}

// =============================================================================
// 3. PHOTONIC CHIP EXECUTION ENGINE
// =============================================================================

pub const MAX_WAVEGUIDES: usize = 8;

pub struct PhotonicChipSim {
    pub waveguides: [OpticalWave; MAX_WAVEGUIDES],
    pub thermal_drift_noise: f64, // Šum faznog drifta usled zagrevanja čipa
}

impl PhotonicChipSim {
    pub fn new(thermal_drift_noise: f64) -> Self {
        Self {
            waveguides: [OpticalWave::zero(); MAX_WAVEGUIDES],
            thermal_drift_noise,
        }
    }

    pub fn execute_instruction(&mut self, inst: &PhotonicInstruction) -> Option<f64> {
        match inst {
            PhotonicInstruction::PumpLaser { channel, amplitude, phase_deg } => {
                if *channel < MAX_WAVEGUIDES {
                    self.waveguides[*channel] = OpticalWave {
                        amplitude: *amplitude,
                        phase_rad: phase_deg.to_radians(),
                        wavelength_nm: 1550.0,
                    };
                }
                None
            }

            PhotonicInstruction::PhaseShift { channel, shift_deg } => {
                if *channel < MAX_WAVEGUIDES {
                    // Dodajemo faza shift + termički šum hardvera
                    let noise = (rand_simple() - 0.5) * self.thermal_drift_noise;
                    let new_phase = self.waveguides[*channel].phase_rad + shift_deg.to_radians() + noise;
                    self.waveguides[*channel].phase_rad = new_phase % (2.0 * PI);
                }
                None
            }

            PhotonicInstruction::Attenuate { channel, factor } => {
                if *channel < MAX_WAVEGUIDES {
                    self.waveguides[*channel].amplitude *= factor.clamp(0.0, 1.0);
                }
                None
            }

            PhotonicInstruction::BeamSplitter { in1, in2, out1, out2 } => {
                if *in1 < MAX_WAVEGUIDES && *in2 < MAX_WAVEGUIDES {
                    let w1 = self.waveguides[*in1];
                    let w2 = self.waveguides[*in2];

                    // 50/50 Beam Splitter unosi fazni skok od π/2 (90 deg) na reflektovanom zraku!
                    let out1_wave = w1.interfere_with(&OpticalWave {
                        amplitude: w2.amplitude / 2.0_f64.sqrt(),
                        phase_rad: (w2.phase_rad + PI / 2.0) % (2.0 * PI),
                        wavelength_nm: w2.wavelength_nm,
                    });

                    let out2_wave = w2.interfere_with(&OpticalWave {
                        amplitude: w1.amplitude / 2.0_f64.sqrt(),
                        phase_rad: (w1.phase_rad + PI / 2.0) % (2.0 * PI),
                        wavelength_nm: w1.wavelength_nm,
                    });

                    if *out1 < MAX_WAVEGUIDES { self.waveguides[*out1] = out1_wave; }
                    if *out2 < MAX_WAVEGUIDES { self.waveguides[*out2] = out2_wave; }
                }
                None
            }

            PhotonicInstruction::MziRotate { in1, in2, theta_deg, phi_deg } => {
                // Mach-Zehnder Interferometer (MZI) izvođenje U(2) unitarne rotacije
                if *in1 < MAX_WAVEGUIDES && *in2 < MAX_WAVEGUIDES {
                    let theta = theta_deg.to_radians();
                    let phi = phi_deg.to_radians();

                    let a = self.waveguides[*in1].amplitude;
                    let b = self.waveguides[*in2].amplitude;

                    // MZI Transfer matrica
                    let out1_amp = (theta / 2.0).cos() * a - (theta / 2.0).sin() * b;
                    let out2_amp = (theta / 2.0).sin() * a + (theta / 2.0).cos() * b;

                    self.waveguides[*in1].amplitude = out1_amp.abs();
                    self.waveguides[*in1].phase_rad = (self.waveguides[*in1].phase_rad + phi) % (2.0 * PI);

                    self.waveguides[*in2].amplitude = out2_amp.abs();
                }
                None
            }

            PhotonicInstruction::Photodetect { channel } => {
                if *channel < MAX_WAVEGUIDES {
                    // Intenzitet svetlosti I = |A|^2 sa kvantnim šumom (Shot Noise)
                    let base_intensity = self.waveguides[*channel].amplitude.powi(2);
                    let shot_noise = (rand_simple() - 0.5) * 0.02; // Kvantni šum
                    let measured_voltage = (base_intensity + shot_noise).max(0.0);
                    Some(measured_voltage)
                } else {
                    None
                }
            }
        }
    }
}

// Jednostavni pseudo-random generator za fizički šum
fn rand_simple() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    (nanos % 1000) as f64 / 1000.0
}