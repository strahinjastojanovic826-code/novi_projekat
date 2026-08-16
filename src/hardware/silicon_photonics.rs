use core::sync::atomic::{AtomicU64, Ordering};

// --- 1. OPTICAL WDM CHANNEL (Wavelength Division Multiplexing) ---

#[derive(Debug, Clone)]
pub struct OpticalChannel {
    pub wavelength_nm: f32, // Talasna dužina (C-band, npr. 1550 nm)
    pub power_mw: f32,      // Snaga lasera u milivatima
    pub data_bit: bool,     // Modulisana svetlosna vrednost (0 ili 1)
}

// --- 2. MACH-ZEHNDER MODULATOR (E/O Converter) ---

#[derive(Debug, Clone)]
pub struct MachZehnderModulator {
    pub v_pi: f32,          // Napon potreban za fazni pomeraj π rad (E/O prelaz)
    pub insertion_loss_db: f32, // Gubitak unutar samog modulatora
}

impl MachZehnderModulator {
    pub fn new(v_pi: f32, insertion_loss_db: f32) -> Self {
        Self { v_pi, insertion_loss_db }
    }

    /// Električno-optička modulacija: Pretvara napon CPU-a u optički intenzitet svetlosti
    pub fn modulate(&self, voltage: f32) -> f32 {
        let phase = core::f32::consts::PI * (voltage / self.v_pi);
        let transmission = 0.5 * (1.0 + phase.cos()); // Interferometrijska jednačina
        let loss_factor = 10.0f32.powf(-self.insertion_loss_db / 10.0);
        transmission * loss_factor
    }
}

// --- 3. SILICON PHOTONIC INTERCONNECT ENGINE ---

pub struct PhotonicsEngine {
    pub channels: Vec<OpticalChannel>,
    pub modulator: MachZehnderModulator,
    pub waveguide_length_cm: f32,      // Dužina svetlovoda na čipu
    pub waveguide_loss_db_per_cm: f32, // Slabljenje po centimetru
    pub total_photons_transmitted: AtomicU64,
    pub energy_saved_pj: AtomicU64,    // pJ ušteđeni u odnosu na bakar
}

impl PhotonicsEngine {
    pub fn new(num_wdm_channels: usize) -> Self {
        let mut channels = Vec::new();
        let base_lambda = 1550.0; // Standardni optički prozor

        for i in 0..num_wdm_channels {
            channels.push(OpticalChannel {
                wavelength_nm: base_lambda + (i as f32 * 0.8), // 0.8 nm DWDM razmak
                power_mw: 1.0,
                data_bit: false,
            });
        }

        Self {
            channels,
            modulator: MachZehnderModulator::new(2.5, 1.2),
            waveguide_length_cm: 4.0, // 4 cm optičke magistrale
            waveguide_loss_db_per_cm: 0.3,
            total_photons_transmitted: AtomicU64::new(0),
            energy_saved_pj: AtomicU64::new(0),
        }
    }

    /// Paralelni prenos paketa podataka kroz jedinstveni fotonski svetlovod
    pub fn transmit_wdm_packet(&mut self, data_bits: &[bool]) -> Vec<(f32, f32, bool)> {
        let mut results = Vec::new();

        for (i, &bit) in data_bits.iter().enumerate() {
            if i < self.channels.len() {
                let voltage = if bit { 2.5 } else { 0.0 };
                let optical_intensity = self.modulator.modulate(voltage);

                // Slabljenje svetlosti duž silicijumskog provodnika
                let total_loss_db = self.waveguide_loss_db_per_cm * self.waveguide_length_cm;
                let received_intensity = optical_intensity * 10.0f32.powf(-total_loss_db / 10.0);

                // Detekcija preko fotodiode (O/E konverzija)
                let detected_bit = received_intensity > 0.15;

                self.channels[i].data_bit = detected_bit;
                results.push((self.channels[i].wavelength_nm, received_intensity, detected_bit));

                self.total_photons_transmitted.fetch_add(1, Ordering::Relaxed);
            }
        }

        // Bakar troši ~10 pJ/bit, fotonski svetlovod ~0.5 pJ/bit
        let saved = (data_bits.len() as f64 * 9.5) as u64;
        self.energy_saved_pj.fetch_add(saved, Ordering::Relaxed);

        results
    }

    /// Proračun kašnjenja optičkog signala kroz silicijum ($v = c / n_{\text{eff}}$)
    pub fn calculate_latency_ps(&self) -> f32 {
        let c_cm_per_ps = 0.0299792; // Brzina svetlosti u vakuumu (cm/ps)
        let n_eff = 3.45;            // Indeks prelamanja silicijuma
        let group_velocity = c_cm_per_ps / n_eff;
        self.waveguide_length_cm / group_velocity
    }
}