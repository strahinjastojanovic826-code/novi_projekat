pub mod signals;

use signals::{Signal, SignalEvent};
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Message {
    pub msg_id: u64,
    pub sender_pid: u32,
    pub target_pid: u32,
    pub payload: String,
}

pub struct QuantumIpcEngine {
    pub message_queues: HashMap<u32, VecDeque<Message>>,
    pub pending_signals: HashMap<u32, VecDeque<SignalEvent>>,
    pub active_pipes: HashMap<String, VecDeque<String>>,
    next_msg_id: u64,
    pub logs: Vec<String>,
}

impl QuantumIpcEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            message_queues: HashMap::new(),
            pending_signals: HashMap::new(),
            active_pipes: HashMap::new(),
            next_msg_id: 1,
            logs: Vec::new(),
        };

        engine.logs.push("Quantum IPC & Signal Engine inicijalizovan.".into());
        engine.seed_demo_ipc();
        engine
    }

    fn current_time_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    pub fn send_signal(&mut self, sender_pid: u32, target_pid: u32, signal: Signal) {
        let sig_event = SignalEvent {
            sender_pid,
            target_pid,
            signal,
            timestamp_ms: Self::current_time_ms(),
        };

        self.pending_signals
            .entry(target_pid)
            .or_insert_with(VecDeque::new)
            .push_back(sig_event);

        self.logs.push(format!(
            "⚡ SIGNAL: PID {} poslao {:?} procesu PID {}",
            sender_pid, signal, target_pid
        ));
    }

    pub fn send_message(&mut self, sender_pid: u32, target_pid: u32, payload: &str) -> u64 {
        let msg_id = self.next_msg_id;
        self.next_msg_id += 1;

        let msg = Message {
            msg_id,
            sender_pid,
            target_pid,
            payload: payload.to_string(),
        };

        self.message_queues
            .entry(target_pid)
            .or_insert_with(VecDeque::new)
            .push_back(msg);

        self.logs.push(format!(
            "✉ IPC MSG #{}: PID {} -> PID {}: '{}'",
            msg_id, sender_pid, target_pid, payload
        ));

        msg_id
    }

    pub fn read_messages(&mut self, target_pid: u32) -> Vec<Message> {
        if let Some(queue) = self.message_queues.get_mut(&target_pid) {
            let msgs: Vec<Message> = queue.drain(..).collect();
            if !msgs.is_empty() {
                self.logs.push(format!(
                    "📥 PID {} pročitao {} IPC poruka.",
                    target_pid, msgs.len()
                ));
            }
            msgs
        } else {
            Vec::new()
        }
    }

    pub fn write_pipe(&mut self, pipe_name: &str, data: &str) {
        self.active_pipes
            .entry(pipe_name.to_string())
            .or_insert_with(VecDeque::new)
            .push_back(data.to_string());

        self.logs.push(format!("🚰 PIPE ['{}'] upisano: '{}'", pipe_name, data));
    }

    pub fn read_pipe(&mut self, pipe_name: &str) -> Option<String> {
        if let Some(buffer) = self.active_pipes.get_mut(pipe_name) {
            let data = buffer.pop_front();
            if let Some(ref d) = data {
                self.logs.push(format!("🚰 PIPE ['{}'] pročitano: '{}'", pipe_name, d));
            }
            data
        } else {
            None
        }
    }

    pub fn seed_demo_ipc(&mut self) {
        self.send_message(1001, 1002, "PING: System Readiness Check");
        self.send_signal(1, 1002, Signal::SIGUSR1);
        self.write_pipe("sys_log_pipe", "CRITICAL_EVENT: Kernel IPC Subsystem Active");
    }
}