use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Emergency,
    Error,
    Warning,
    Info,
    Debug,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Emergency => "EMERG",
            LogLevel::Error => "ERR",
            LogLevel::Warning => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
        }
    }
}

#[derive(Debug, Clone)]
pub struct KernelLogEntry {
    pub timestamp_ms: u64,
    pub level: LogLevel,
    pub subsystem: String,
    pub message: String,
}

pub struct KernelRingBuffer {
    pub capacity: usize,
    pub buffer: VecDeque<KernelLogEntry>,
}

impl KernelRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buffer: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, timestamp_ms: u64, level: LogLevel, subsystem: &str, message: &str) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front(); // Brišemo najstariju poruku kad se napuni bafer
        }
        self.buffer.push_back(KernelLogEntry {
            timestamp_ms,
            level,
            subsystem: subsystem.to_string(),
            message: message.to_string(),
        });
    }

    pub fn read_all(&self) -> Vec<KernelLogEntry> {
        self.buffer.iter().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}