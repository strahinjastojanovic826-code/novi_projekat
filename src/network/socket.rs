use super::packet::QuquatPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Closed,
    Listening,
    Connected,
}

pub struct QuantumSocket {
    pub port: u16,
    pub owner_pid: u32,
    pub state: SocketState,
    pub rx_buffer: Vec<QuquatPacket>,
    pub tx_buffer: Vec<QuquatPacket>,
}

impl QuantumSocket {
    pub fn new(port: u16, owner_pid: u32) -> Self {
        Self {
            port,
            owner_pid,
            state: SocketState::Listening,
            rx_buffer: Vec::new(),
            tx_buffer: Vec::new(),
        }
    }

    pub fn send(&mut self, packet: QuquatPacket) {
        self.tx_buffer.push(packet);
    }

    pub fn receive(&mut self) -> Option<QuquatPacket> {
        if self.rx_buffer.is_empty() {
            None
        } else {
            Some(self.rx_buffer.remove(0))
        }
    }
}