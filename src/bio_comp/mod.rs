
// =============================================================================
// 1. DNK MOLEKULARNI SOLVER (DNA HYBRIDIZATION COMPUTING)
// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DnaBase {
    A, // Adenin
    T, // Timin
    C, // Citozin
    G, // Guanin
}

impl DnaBase {
    /// Komplementarni bazni par sa vodoničnim vezama
    pub fn complement(&self) -> Self {
        match self {
            DnaBase::A => DnaBase::T,
            DnaBase::T => DnaBase::A,
            DnaBase::C => DnaBase::G,
            DnaBase::G => DnaBase::C,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DnaStrand {
    pub sequence: Vec<DnaBase>,
}

impl DnaStrand {
    pub fn from_str(seq: &str) -> Self {
        let sequence = seq.chars().filter_map(|c| match c {
            'A' | 'a' => Some(DnaBase::A),
            'T' | 't' => Some(DnaBase::T),
            'C' | 'c' => Some(DnaBase::C),
            'G' | 'g' => Some(DnaBase::G),
            _ => None,
        }).collect();
        Self { sequence }
    }

    pub fn to_string(&self) -> String {
        self.sequence.iter().map(|b| match b {
            DnaBase::A => 'A',
            DnaBase::T => 'T',
            DnaBase::C => 'C',
            DnaBase::G => 'G',
        }).collect()
    }

    /// Vraća komplementarni lanac za hibridizaciju (vezivanje u dvostruku heliks strukturu)
    pub fn create_complementary(&self) -> Self {
        let sequence = self.sequence.iter().map(|b| b.complement()).collect();
        Self { sequence }
    }
}

/// Simulacija DNK epruvete za rešavanje grafa (Hamiltonian Path)
pub struct DnaTestTube {
    pub strands: Vec<DnaStrand>,
}

impl DnaTestTube {
    pub fn new() -> Self {
        Self { strands: Vec::new() }
    }

    pub fn add_strand(&mut self, strand: DnaStrand) {
        self.strands.push(strand);
    }

    /// Enzymatic Filter: Simulira PCR/gel elektroforezu koja ostavlja samo lance tačne dužine
    pub fn filter_by_length(&mut self, target_length: usize) {
        self.strands.retain(|s| s.sequence.len() == target_length);
    }
}

// =============================================================================
// 2. BIOLOŠKI NEURON (LEAKY INTEGRATE-AND-FIRE / SPIKE ENCODER)
// =============================================================================

/// Leaky Integrate-and-Fire (LIF) model biološkog neurona
pub struct LifNeuron {
    pub v_membrane: f64,    // Trenutni potencijal membrane (mV)
    pub v_rest: f64,        // Potencijal mirovanja (-70 mV)
    pub v_threshold: f64,   // Prag za okidanje spajka (-55 mV)
    pub v_reset: f64,       // Potencijal nakon pražnjenja (-75 mV)
    pub leak_factor: f64,   // Curenje membrane tokom vremena
}

impl LifNeuron {
    pub fn new() -> Self {
        Self {
            v_membrane: -70.0,
            v_rest: -70.0,
            v_threshold: -55.0,
            v_reset: -75.0,
            leak_factor: 0.85, // 15% curenja u svakom vremenskom koraku
        }
    }

    /// Obrada ulazne struje i generisanje biološkog akcionog potencijala (Spike)
    pub fn step(&mut self, input_current: f64) -> bool {
        // 1. Integracija ulazne struje uz curenje (Leakage)
        self.v_membrane = self.v_rest + (self.v_membrane - self.v_rest) * self.leak_factor + input_current;

        // 2. Okidanje spajka ako je prešao prag (Action Potential)
        if self.v_membrane >= self.v_threshold {
            self.v_membrane = self.v_reset; // Resetuj potencijal
            true // Generisan spajk!
        } else {
            false
        }
    }
}

//Nemam api za igrice ni z buffer ni 4x4 matricu za AAA igre
// al zato imam kodove za fotoniku i neurone
//greota da se ne mogu tu pokrenuti igrice