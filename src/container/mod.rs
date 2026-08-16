use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ContainerStatus {
    Created,
    Running,
    Paused,
    Stopped,
}

#[derive(Debug, Clone)]
pub struct QContainer {
    pub id: String,
    pub name: String,
    pub image: String,
    pub allocated_memory_mb: usize,
    pub status: ContainerStatus,
    pub ip_address: String,
    pub cpu_shares: u32,
    pub logs: Vec<String>,
}

pub struct HypervisorEngine {
    pub containers: HashMap<String, QContainer>,
    pub available_images: Vec<String>,
    next_container_id: u32,
}

impl HypervisorEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            containers: HashMap::new(),
            available_images: vec![
                "ququat/ubuntu-mini:latest".to_string(),
                "ququat/quantum-redis:v1".to_string(),
                "ququat/python-core:3.11".to_string(),
            ],
            next_container_id: 101,
        };

        // Inicijalni podrazumevani kontejner
        engine.create_container("web_server", "ququat/ubuntu-mini:latest", 64, 2);
        engine
    }

    pub fn create_container(&mut self, name: &str, image: &str, memory_mb: usize, cpu_shares: u32) -> String {
        let id = format!("cnt-{:04x}", self.next_container_id);
        self.next_container_id += 1;

        let ip_address = format!("172.18.0.{}", self.containers.len() + 2);

        let container = QContainer {
            id: id.clone(),
            name: name.to_string(),
            image: image.to_string(),
            allocated_memory_mb: memory_mb,
            status: ContainerStatus::Created,
            ip_address,
            cpu_shares,
            logs: vec![format!("Container '{}' kreiran iz imidža '{}'", name, image)],
        };

        self.containers.insert(id.clone(), container);
        id
    }

    pub fn start_container(&mut self, id: &str) -> bool {
        if let Some(c) = self.containers.get_mut(id) {
            c.status = ContainerStatus::Running;
            c.logs.push("Kontejner je uspešno pokrenut.".to_string());
            true
        } else {
            false
        }
    }

    pub fn stop_container(&mut self, id: &str) -> bool {
        if let Some(c) = self.containers.get_mut(id) {
            c.status = ContainerStatus::Stopped;
            c.logs.push("Kontejner je zaustavljen.".to_string());
            true
        } else {
            false
        }
    }

    pub fn pause_container(&mut self, id: &str) -> bool {
        if let Some(c) = self.containers.get_mut(id) {
            c.status = ContainerStatus::Paused;
            c.logs.push("Kontejner je pauziran (zamrznut RAM).".to_string());
            true
        } else {
            false
        }
    }

    pub fn remove_container(&mut self, id: &str) -> bool {
        self.containers.remove(id).is_some()
    }
}