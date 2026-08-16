pub mod profiler;
pub mod ring_buffer;

use profiler::QuantumProfiler;
use ring_buffer::{KernelRingBuffer, LogLevel};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct TelemetrySnapshot {
    pub cpu_usage_pct: f32,
    pub ram_used_mb: u64,
    pub ram_total_mb: u64,
    pub iops: u32,
    pub context_switches: u64,
    pub irq_counter: u64,
}

pub struct QuantumTelemetryEngine {
    pub ring_buffer: KernelRingBuffer,
    pub profiler: QuantumProfiler,
    pub snapshot: TelemetrySnapshot,
    pub logs: Vec<String>,
}

impl QuantumTelemetryEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            ring_buffer: KernelRingBuffer::new(256),
            profiler: QuantumProfiler::new(),
            snapshot: TelemetrySnapshot {
                cpu_usage_pct: 12.4,
                ram_used_mb: 2048,
                ram_total_mb: 16384,
                iops: 4200,
                context_switches: 142050,
                irq_counter: 98120,
            },
            logs: Vec::new(),
        };

        engine.logs.push("Telemetry & Kernel Ring Buffer Subsystem pokrenut.".into());
        engine.seed_demo_telemetry();
        engine
    }

    fn now_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    pub fn log_kmessage(&mut self, level: LogLevel, subsystem: &str, msg: &str) {
        self.ring_buffer.push(Self::now_ms(), level, subsystem, msg);
    }

    pub fn update_snapshot(&mut self, cpu: f32, ram_mb: u64, iops: u32) {
        self.snapshot.cpu_usage_pct = cpu;
        self.snapshot.ram_used_mb = ram_mb;
        self.snapshot.iops = iops;
        self.snapshot.context_switches += 150;
        self.snapshot.irq_counter += 80;
    }

    pub fn seed_demo_telemetry(&mut self) {
        let t = Self::now_ms();
        self.ring_buffer.push(t - 5000, LogLevel::Info, "KERNEL", "Booting QuantumOS Microkernel v4.2...");
        self.ring_buffer.push(t - 4000, LogLevel::Info, "VFS", "Virtual File System mounted on /");
        self.ring_buffer.push(t - 3000, LogLevel::Warning, "NET", "Interface eth0 lost carrier, switching to wifi0");
        self.ring_buffer.push(t - 2000, LogLevel::Error, "DRV_GOP", "FrameBuffer flickering detected, applying vsync fix");
        self.ring_buffer.push(t - 1000, LogLevel::Debug, "SCHED", "Context switch delay: 1.2us");

        self.profiler.record_call("vfs_read_block", 45);
        self.profiler.record_call("vfs_read_block", 60);
        self.profiler.record_call("crypto_aes_encrypt", 120);
        self.profiler.record_call("ipc_send_message", 15);
        self.profiler.record_call("ipc_send_message", 18);
    }
}