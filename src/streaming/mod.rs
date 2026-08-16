pub mod topic;

use topic::{Record, Topic};
use std::collections::HashMap;

pub struct QuantumEventStreamEngine {
    pub topics: HashMap<String, Topic>,
    // Consumer Group Offsets: group_name -> (topic_partition_key -> committed_offset)
    pub consumer_offsets: HashMap<String, HashMap<String, u64>>,
    pub total_messages_processed: u64,
    pub logs: Vec<String>,
}

impl QuantumEventStreamEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            topics: HashMap::new(),
            consumer_offsets: HashMap::new(),
            total_messages_processed: 0,
            logs: Vec::new(),
        };

        engine.logs.push("Quantum Event Streaming Engine (Kafka-like) inicijalizovan.".into());
        engine.create_topic("system.events", 3);
        engine.create_topic("network.telemetry", 2);
        engine.seed_demo_events();

        engine
    }

    pub fn create_topic(&mut self, name: &str, num_partitions: usize) {
        if !self.topics.contains_key(name) {
            self.topics.insert(name.to_string(), Topic::new(name, num_partitions));
            self.logs.push(format!("Kreirana tema '{}' sa {} particije/a.", name, num_partitions));
        }
    }

    pub fn publish_event(&mut self, topic_name: &str, key: &str, value: &str) -> Option<(u32, u64)> {
        if let Some(topic) = self.topics.get_mut(topic_name) {
            let (part_id, offset) = topic.publish(key, value);
            self.total_messages_processed += 1;
            self.logs.push(format!(
                "PRODUCER -> Topic: '{}' | Particija: {} | Offset: {} | K: '{}' V: '{}'",
                topic_name, part_id, offset, key, value
            ));
            Some((part_id, offset))
        } else {
            None
        }
    }

    pub fn consume_events(
        &mut self,
        group_id: &str,
        topic_name: &str,
        partition_id: u32,
        fetch_limit: usize,
    ) -> Vec<Record> {
        let part_key = format!("{}:{}", topic_name, partition_id);
        let current_offset = self
            .consumer_offsets
            .entry(group_id.to_string())
            .or_insert_with(HashMap::new)
            .get(&part_key)
            .cloned()
            .unwrap_or(0);

        if let Some(topic) = self.topics.get(topic_name) {
            if let Some(partition) = topic.partitions.get(partition_id as usize) {
                let records = partition.read_from_offset(current_offset, fetch_limit);

                if let Some(last_record) = records.last() {
                    let new_offset = last_record.offset + 1;
                    self.consumer_offsets
                        .get_mut(group_id)
                        .unwrap()
                        .insert(part_key, new_offset);

                    self.logs.push(format!(
                        "CONSUMER [{}] -> Topic: '{}' P:{} | Pročitano {} poruka. Novi Offset: {}",
                        group_id, topic_name, partition_id, records.len(), new_offset
                    ));
                }

                return records;
            }
        }

        Vec::new()
    }

    pub fn seed_demo_events(&mut self) {
        self.publish_event("system.events", "AUTH", "User 'admin' logged in from 127.0.0.1");
        self.publish_event("system.events", "KERNEL", "Memory Allocation Success (64MB)");
        self.publish_event("system.events", "VFS", "File '/etc/config.json' modified");
        self.publish_event("network.telemetry", "PING", "Gateway latency: 1.2ms");
        self.publish_event("network.telemetry", "TRAFFIC", "Inbound 1024 KB/s");
    }
}