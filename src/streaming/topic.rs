use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Record {
    pub offset: u64,
    pub timestamp_ms: u64,
    pub key: String,
    pub value: String,
}

pub struct Partition {
    pub id: u32,
    pub log: Vec<Record>,
    pub next_offset: u64,
}

impl Partition {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            log: Vec::new(),
            next_offset: 0,
        }
    }

    pub fn append(&mut self, key: &str, value: &str) -> u64 {
        let offset = self.next_offset;
        self.next_offset += 1;

        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        self.log.push(Record {
            offset,
            timestamp_ms,
            key: key.to_string(),
            value: value.to_string(),
        });

        offset
    }

    pub fn read_from_offset(&self, start_offset: u64, limit: usize) -> Vec<Record> {
        self.log
            .iter()
            .filter(|r| r.offset >= start_offset)
            .take(limit)
            .cloned()
            .collect()
    }
}

pub struct Topic {
    pub name: String,
    pub partitions: Vec<Partition>,
    round_robin_counter: usize,
}

impl Topic {
    pub fn new(name: &str, num_partitions: usize) -> Self {
        let mut partitions = Vec::new();
        for i in 0..num_partitions {
            partitions.push(Partition::new(i as u32));
        }

        Self {
            name: name.to_string(),
            partitions,
            round_robin_counter: 0,
        }
    }

    /// Objavljivanje poruke na particiju (Round-Robin raspodela)
    pub fn publish(&mut self, key: &str, value: &str) -> (u32, u64) {
        let partition_idx = self.round_robin_counter % self.partitions.len();
        self.round_robin_counter += 1;

        let partition = &mut self.partitions[partition_idx];
        let offset = partition.append(key, value);

        (partition.id, offset)
    }
}