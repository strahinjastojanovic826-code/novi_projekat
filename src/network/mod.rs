pub mod packet;
pub mod socket;

pub use packet::{QuquatPacket, PacketType};
pub use socket::QuantumSocket;

use crate::driver::WinQuantumDriver;

pub struct NetworkEngine {
    pub sockets: Vec<QuantumSocket>,
    pub tx_bus: Vec<QuquatPacket>, // Virtuelni mrežni kabl / magistrala
    pub packets_sent: u64,
    pub packets_received: u64,
    pub network_log: Vec<String>,
}

impl NetworkEngine {
    pub fn new() -> Self {
        Self {
            sockets: Vec::new(),
            tx_bus: Vec::new(),
            packets_sent: 0,
            packets_received: 0,
            network_log: vec!["[NET] QNet Virtuelni Mrežni Drajver Inicijalizovan.".to_string()],
        }
    }

    /// Otvara novi soket za određeni proces i port
    pub fn open_socket(&mut self, port: u16, pid: u32) -> bool {
        if self.sockets.iter().any(|s| s.port == port) {
            self.network_log.push(format!("[NET_ERR] Port {} je već zauzet!", port));
            return false;
        }
        self.sockets.push(QuantumSocket::new(port, pid));
        self.network_log.push(format!("[NET] PID {} je otvorio Soket na portu {}", pid, port));
        true
    }

    /// Šalje paket na mrežnu magistralu
    pub fn transmit(&mut self, packet: QuquatPacket) {
        self.packets_sent += 1;
        self.network_log.push(format!(
            "[TX] PID {} -> PID {} (Port {}): {:?} [Payload: {} kvat/a]",
            packet.src_pid, packet.dest_pid, packet.dest_port, packet.packet_type, packet.payload.len()
        ));
        self.tx_bus.push(packet);
    }

    /// Takt mrežnog drajvera: Rutira pakete iz magistrale ka odgovarajućim soketima
    pub fn poll(&mut self, driver: &WinQuantumDriver) {
        while let Some(pkt) = self.tx_bus.pop() {
            if !pkt.is_valid() {
                self.network_log.push(format!("[NET_ERR] Korumpiran paket bačen! Checksum Mismatch."));
                continue;
            }

            // Ako je tip paketa QuantumSync, upisujemo prvi kvat u fizički registar
            if pkt.packet_type == PacketType::QuantumSync && !pkt.payload.is_empty() {
                driver.set_ququat(0, pkt.payload[0]);
                self.network_log.push(format!("[NET_SYNC] Hardware Registar sinhronizovan sa mrežnim paketom."));
            }

            // Rutiranje ka soketu na ciljnom portu
            if let Some(sock) = self.sockets.iter_mut().find(|s| s.port == pkt.dest_port) {
                sock.rx_buffer.push(pkt);
                self.packets_received += 1;
            } else {
                self.network_log.push(format!("[NET_DROP] Paket za port {} odbačen (Nema otvorenog soketa).", pkt.dest_port));
            }
        }
    }
}