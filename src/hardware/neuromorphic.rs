use core::sync::atomic::{AtomicU64, Ordering};

// --- 1. LEAKY INTEGRATE-AND-FIRE (LIF) NEURON MODEL ---

#[derive(Debug, Clone)]
pub struct LifNeuron {
    pub id: u64,
    pub v_membrane: f32,    // Trenutni potencijal membrane (mV)
    pub v_thresh: f32,      // Prag okidanja impulsa (Spike Threshold)
    pub v_reset: f32,       // Potencijal mirovanja nakon okidanja
    pub decay: f32,         // Faktor curenja potencijala (Leak Factor, e.g. 0.85)
    pub last_spike_tick: u64,
    pub total_spikes: u64,
}

impl LifNeuron {
    pub fn new(id: u64, v_thresh: f32, decay: f32) -> Self {
        Self {
            id,
            v_membrane: 0.0,
            v_thresh,
            v_reset: 0.0,
            decay,
            last_spike_tick: 0,
            total_spikes: 0,
        }
    }

    /// Integracija ulazne struje i provera okidanja impulsa (Spike generation)
    pub fn integrate(&mut self, input_current: f32, current_tick: u64) -> bool {
        // V(t+1) = V(t) * decay + I(t)
        self.v_membrane = (self.v_membrane * self.decay) + input_current;

        if self.v_membrane >= self.v_thresh {
            self.v_membrane = self.v_reset; // Reset potencijala
            self.last_spike_tick = current_tick;
            self.total_spikes += 1;
            true // Generisan biološki akcioni potencijal (Spike)!
        } else {
            false
        }
    }
}

// --- 2. SINAPSA I STDP PLASTIČNOST (HEBBIAN LEARNING) ---

#[derive(Debug, Clone)]
pub struct Synapse {
    pub pre_id: u64,
    pub post_id: u64,
    pub weight: f32, // Jačina sinaptičke veze
}

// --- 3. NEUROMORPHIC CORE SIMULATOR ---

pub struct NeuromorphicCore {
    pub neurons: Vec<LifNeuron>,
    pub synapses: Vec<Synapse>,
    pub total_spikes_processed: AtomicU64,
    pub current_tick: u64,
}

impl NeuromorphicCore {
    pub fn new(num_neurons: usize) -> Self {
        let mut neurons = Vec::new();
        for i in 0..num_neurons {
            neurons.push(LifNeuron::new(i as u64, 1.0, 0.80)); // Prag = 1.0mV, Curenje = 20% po tick-u
        }

        Self {
            neurons,
            synapses: Vec::new(),
            total_spikes_processed: AtomicU64::new(0),
            current_tick: 0,
        }
    }

    /// Povezuje dva neurona sinaptičkom vezom
    pub fn connect(&mut self, pre_id: u64, post_id: u64, weight: f32) {
        self.synapses.push(Synapse {
            pre_id,
            post_id,
            weight,
        });
    }

    /// STDP (Spike-Timing-Dependent Plasticity) urealnom vremenu:
    /// Ako je Pre-neuron opalio pre Post-neurona -> Sinapsa jača (Long-Term Potentiation)
    /// Ako je Post-neuron opalio pre Pre-neurona -> Sinapsa slabi (Long-Term Depression)
    pub fn apply_stdp(&mut self, pre_id: u64, post_id: u64, learning_rate: f32) {
        let pre_tick = self.neurons.iter().find(|n| n.id == pre_id).map(|n| n.last_spike_tick);
        let post_tick = self.neurons.iter().find(|n| n.id == post_id).map(|n| n.last_spike_tick);

        if let (Some(t_pre), Some(t_post)) = (pre_tick, post_tick) {
            for syn in self.synapses.iter_mut() {
                if syn.pre_id == pre_id && syn.post_id == post_id {
                    let dt = t_post as i64 - t_pre as i64;
                    if dt > 0 && dt < 10 {
                        syn.weight += learning_rate * (1.0 / dt as f32); // Jačanje veze
                    } else if dt < 0 && dt > -10 {
                        syn.weight -= learning_rate * (1.0 / dt.abs() as f32); // Slabljenje veze
                    }
                    syn.weight = syn.weight.clamp(0.05, 5.0);
                }
            }
        }
    }

    /// Izvršava jedan taktski impuls (Tick) na celom čipu
    pub fn tick(&mut self, external_stimuli: &[(u64, f32)]) -> Vec<u64> {
        self.current_tick += 1;
        let mut active_spikes = Vec::new();

        // 1. Injekcija eksternih strujnih stimulusa
        for &(neuron_id, current) in external_stimuli {
            if let Some(n) = self.neurons.iter_mut().find(|n| n.id == neuron_id) {
                if n.integrate(current, self.current_tick) {
                    active_spikes.push(n.id);
                }
            }
        }

        // 2. Event-driven propagacija impulsa kroz sinapse
        let mut propagated_inputs: Vec<(u64, f32, u64)> = Vec::new(); // (post_id, weight, pre_id)

        for &spiked_pre_id in &active_spikes {
            self.total_spikes_processed.fetch_add(1, Ordering::Relaxed);

            for syn in &self.synapses {
                if syn.pre_id == spiked_pre_id {
                    propagated_inputs.push((syn.post_id, syn.weight, syn.pre_id));
                }
            }
        }

        // 3. Post-sinaptičko napajanje i automatska STDP korekcija
        for (target_post_id, weight, pre_id) in propagated_inputs {
            if let Some(n) = self.neurons.iter_mut().find(|n| n.id == target_post_id) {
                if n.integrate(weight, self.current_tick) {
                    active_spikes.push(n.id);
                    // Nauči vezu u hodu!
                    self.apply_stdp(pre_id, target_post_id, 0.15);
                }
            }
        }

        active_spikes
    }
}