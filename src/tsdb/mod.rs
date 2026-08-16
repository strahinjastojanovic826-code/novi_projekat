pub mod metrics;

use metrics::{AggregationFunc, MetricSeries};
use std::collections::HashMap;

pub struct QuantumTimeSeriesEngine {
    pub series_map: HashMap<String, MetricSeries>,
    pub logs: Vec<String>,
    pub total_ingested_points: u64,
}

impl QuantumTimeSeriesEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            series_map: HashMap::new(),
            logs: Vec::new(),
            total_ingested_points: 0,
        };

        // Registracija ključnih sistemskih metričkih nizova
        engine.create_series("system.cpu.usage", "%", 100);
        engine.create_series("system.ram.used_mb", "MB", 100);
        engine.create_series("system.network.throughput", "KB/s", 100);
        engine.create_series("system.vfs.iops", "IOPS", 100);

        // Generisanje početnih demonstracionih podataka
        engine.seed_initial_data();

        engine.logs.push("Time-Series DB Engine uspešno inicijalizovan.".into());
        engine
    }

    pub fn create_series(&mut self, name: &str, unit: &str, capacity: usize) {
        if !self.series_map.contains_key(name) {
            self.series_map.insert(
                name.to_string(),
                MetricSeries::new(name, unit, capacity),
            );
            self.logs.push(format!("Kreirana nova TSDB serija: '{}' [{}]", name, unit));
        }
    }

    pub fn record_metric(&mut self, name: &str, value: f64) {
        if let Some(series) = self.series_map.get_mut(name) {
            series.add_point(value);
            self.total_ingested_points += 1;
        } else {
            let mut series = MetricSeries::new(name, "raw", 100);
            series.add_point(value);
            self.series_map.insert(name.to_string(), series);
            self.total_ingested_points += 1;
        }
    }

    pub fn seed_initial_data(&mut self) {
        let cpu_samples = [12.5, 18.0, 45.2, 89.1, 92.4, 34.0, 22.1, 15.0, 28.4, 40.0];
        let ram_samples = [1024.0, 1050.0, 1120.0, 1300.0, 1450.0, 1420.0, 1380.0, 1200.0, 1180.0, 1150.0];

        for &val in &cpu_samples {
            self.record_metric("system.cpu.usage", val);
        }
        for &val in &ram_samples {
            self.record_metric("system.ram.used_mb", val);
        }
    }

    pub fn query_aggregate(&self, name: &str, func: AggregationFunc) -> Option<f64> {
        self.series_map.get(name).and_then(|s| s.aggregate(func))
    }

    pub fn prune_all(&mut self) {
        let mut pruned = 0;
        for series in self.series_map.values_mut() {
            let count = series.points.len();
            series.points.clear();
            pruned += count;
        }
        self.logs.push(format!("TSDB Prune: Ukupno očišćeno {} tačaka.", pruned));
    }
}