use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FunctionProfile {
    pub name: String,
    pub call_count: u64,
    pub total_time_us: u64,
    pub min_time_us: u64,
    pub max_time_us: u64,
}

pub struct QuantumProfiler {
    pub metrics: HashMap<String, FunctionProfile>,
}

impl QuantumProfiler {
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
        }
    }

    pub fn record_call(&mut self, func_name: &str, duration_us: u64) {
        let entry = self.metrics.entry(func_name.to_string()).or_insert(FunctionProfile {
            name: func_name.to_string(),
            call_count: 0,
            total_time_us: 0,
            min_time_us: u64::MAX,
            max_time_us: 0,
        });

        entry.call_count += 1;
        entry.total_time_us += duration_us;
        if duration_us < entry.min_time_us {
            entry.min_time_us = duration_us;
        }
        if duration_us > entry.max_time_us {
            entry.max_time_us = duration_us;
        }
    }

    pub fn get_avg_time_us(&self, func_name: &str) -> u64 {
        if let Some(prof) = self.metrics.get(func_name) {
            if prof.call_count > 0 {
                return prof.total_time_us / prof.call_count;
            }
        }
        0
    }
}