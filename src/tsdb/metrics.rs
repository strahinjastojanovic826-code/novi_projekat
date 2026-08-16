use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct DataPoint {
    pub timestamp: u64, // Unix timestamp u sekundama
    pub value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AggregationFunc {
    Average,
    Min,
    Max,
    Sum,
    Count,
}

#[derive(Debug, Clone)]
pub struct MetricSeries {
    pub name: String,
    pub unit: String,
    pub points: Vec<DataPoint>,
    pub max_capacity: usize, // Retention policy: Maksimalan broj tačaka u memoriji
}

impl MetricSeries {
    pub fn new(name: &str, unit: &str, max_capacity: usize) -> Self {
        Self {
            name: name.to_string(),
            unit: unit.to_string(),
            points: Vec::new(),
            max_capacity,
        }
    }

    pub fn add_point(&mut self, value: f64) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.points.push(DataPoint { timestamp, value });

        // Automatsko izbacivanje najstarijih tačaka kada se pređe kapacitet
        if self.points.len() > self.max_capacity {
            self.points.remove(0);
        }
    }

    pub fn aggregate(&self, func: AggregationFunc) -> Option<f64> {
        if self.points.is_empty() {
            return None;
        }

        let values: Vec<f64> = self.points.iter().map(|p| p.value).collect();

        match func {
            AggregationFunc::Average => Some(values.iter().sum::<f64>() / values.len() as f64),
            AggregationFunc::Min => values.iter().cloned().reduce(f64::min),
            AggregationFunc::Max => values.iter().cloned().reduce(f64::max),
            AggregationFunc::Sum => Some(values.iter().sum()),
            AggregationFunc::Count => Some(values.len() as f64),
        }
    }
}