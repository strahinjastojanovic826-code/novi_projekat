#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Signal {
    SIGINT = 2,
    SIGKILL = 9,
    SIGUSR1 = 10,
    SIGSEGV = 11,
    SIGUSR2 = 12,
    SIGTERM = 15,
    SIGCONT = 18,
    SIGSTOP = 19,
}

impl Signal {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "SIGINT" | "2" => Some(Signal::SIGINT),
            "SIGKILL" | "9" => Some(Signal::SIGKILL),
            "SIGUSR1" | "10" => Some(Signal::SIGUSR1),
            "SIGSEGV" | "11" => Some(Signal::SIGSEGV),
            "SIGUSR2" | "12" => Some(Signal::SIGUSR2),
            "SIGTERM" | "15" => Some(Signal::SIGTERM),
            "SIGCONT" | "18" => Some(Signal::SIGCONT),
            "SIGSTOP" | "19" => Some(Signal::SIGSTOP),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SignalEvent {
    pub sender_pid: u32,
    pub target_pid: u32,
    pub signal: Signal,
    pub timestamp_ms: u64,
}