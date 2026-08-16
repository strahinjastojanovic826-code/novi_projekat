use crate::domain::QuquatVal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketType {
    Handshake = 0x01,
    Data = 0x02,
    Ack = 0x03,
    QuantumSync = 0x04,
}

#[derive(Debug, Clone)]
pub struct QuquatPacket {
    pub src_pid: u32,
    pub dest_pid: u32,
    pub dest_port: u16,
    pub packet_type: PacketType,
    pub payload: Vec<QuquatVal>,
    pub checksum: u16,
}

impl QuquatPacket {
    pub fn new(src_pid: u32, dest_pid: u32, dest_port: u16, packet_type: PacketType, payload: Vec<QuquatVal>) -> Self {
        let mut pkt = Self {
            src_pid,
            dest_pid,
            dest_port,
            packet_type,
            payload,
            checksum: 0,
        };
        pkt.checksum = pkt.calculate_checksum();
        pkt
    }

    /// Računa paritet i jednostavan kontrolni zbir na osnovu stanja kvata
    pub fn calculate_checksum(&self) -> u16 {
        let mut sum: u32 = (self.src_pid ^ self.dest_pid ^ (self.dest_port as u32)) + (self.packet_type as u32);
        for q in &self.payload {
            sum = sum.wrapping_add(*q as u32);
        }
        (sum & 0xFFFF) as u16
    }

    pub fn is_valid(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }
}