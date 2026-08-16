use std::collections::VecDeque;

// =============================================================================
// 1. NVMe STORAGE CONTROLLER (SUBMISSION/COMPLETION QUEUES & DOORBELLS)
// =============================================================================

#[derive(Debug, Clone, Copy)]
pub struct NvmeCommand {
    pub command_id: u16,
    pub opcode: u8,      // 0x01 = Write, 0x02 = Read
    pub nsid: u32,        // Namespace ID (npr. 1 za /dev/nvme0n1)
    pub prp1: u64,       // Physical Region Page 1 (DMA adresa bafera)
    pub lba: u64,        // Logical Block Address (Sektor na SSD-u)
    pub block_count: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct NvmeCompletion {
    pub command_id: u16,
    pub sq_head: u16,
    pub status: u16, // 0 = Uspeh
}

pub struct NvmeQueuePair {
    pub sq: VecDeque<NvmeCommand>,
    pub cq: VecDeque<NvmeCompletion>,
    pub sq_tail: u16,
    pub cq_head: u16,
    pub doorbell_sq_tail_addr: u64,
}

impl NvmeQueuePair {
    pub fn new(doorbell_addr: u64) -> Self {
        Self {
            sq: VecDeque::new(),
            cq: VecDeque::new(),
            sq_tail: 0,
            cq_head: 0,
            doorbell_sq_tail_addr: doorbell_addr,
        }
    }

    /// CPU ubacuje komandu u Submission Queue i "zvonjava" na Doorbell registar
    pub fn submit_command(&mut self, cmd: NvmeCommand) {
        self.sq.push_back(cmd);
        self.sq_tail = self.sq_tail.wrapping_add(1);
    }

    /// NVMe Kontroler procesira komandu preko DMA i upisuje rezultat u Completion Queue
    pub fn process_hardware(&mut self) -> Option<NvmeCompletion> {
        if let Some(cmd) = self.sq.pop_front() {
            let completion = NvmeCompletion {
                command_id: cmd.command_id,
                sq_head: self.sq_tail,
                status: 0, // Success
            };
            self.cq.push_back(completion);
            Some(completion)
        } else {
            None
        }
    }
}

// =============================================================================
// 2. EMBEDDED BUS PROTOCOLS (UART, SPI, I2C, CAN BUS)
// =============================================================================

/// UART Kontroler (Asinhrona serijska veza)
pub struct UartController {
    pub baud_rate: u32,
    pub tx_fifo: VecDeque<u8>,
    pub rx_fifo: VecDeque<u8>,
}

impl UartController {
    pub fn new(baud_rate: u32) -> Self {
        Self {
            baud_rate,
            tx_fifo: VecDeque::new(),
            rx_fifo: VecDeque::new(),
        }
    }

    pub fn transmit_byte(&mut self, byte: u8) {
        self.tx_fifo.push_back(byte);
    }

    pub fn receive_byte(&mut self) -> Option<u8> {
        self.rx_fifo.pop_front()
    }
}

/// SPI Kontroler (Full-Duplex Sinhrona Magistrala)
#[derive(Debug, Clone, Copy)]
pub enum SpiMode {
    Mode0, // CPOL=0, CPHA=0
    Mode1, // CPOL=0, CPHA=1
    Mode2, // CPOL=1, CPHA=0
    Mode3, // CPOL=1, CPHA=1
}

pub struct SpiController {
    pub mode: SpiMode,
    pub clock_mhz: u32,
}

impl SpiController {
    pub fn new(mode: SpiMode, clock_mhz: u32) -> Self {
        Self { mode, clock_mhz }
    }

    /// Full-Duplex prenosi 1 bajt sa MOSI linije i istovremeno čita 1 bajt sa MISO linije
    pub fn transfer_byte(&self, _mosi_data: u8, slave_response: u8) -> u8 {
        // U pravom hardveru, takt (SCLK) pomera bit po bit u oba smera istovremeno
        slave_response
    }
}

/// I2C Master Kontroler (Multi-Master, Addressed Bus sa ACK/NACK)
pub struct I2cMaster {
    pub bus_speed_khz: u32,
}

impl I2cMaster {
    pub fn new(speed_khz: u32) -> Self {
        Self { bus_speed_khz: speed_khz }
    }

    /// I2C Start Condition -> Slave Address + Write Bit -> ACK
    pub fn write_bytes(&self, target_address: u8, _data: &[u8]) -> Result<bool, &'static str> {
        if target_address > 0x7F {
            return Err("I2C Error: Adresa mora biti 7-bitna!");
        }
        // Simuliramo slanje Start Bita, Adrese, Proveru ACK i Slanje Podataka
        Ok(true) // ACK primljen
    }
}

/// CAN Bus Okvir (Automotive / Industrial Differential Frame)
#[derive(Debug, Clone)]
pub struct CanFrame {
    pub id: u32,          // 11-bit Standard ili 29-bit Extended ID
    pub is_extended: bool,
    pub dlc: u8,           // Data Length Code (0..8 bajtova)
    pub data: [u8; 8],
}

pub struct CanBusEngine;

impl CanBusEngine {
    /// Nedestruktivna bit-po-bit arbitraža: Niži ID ima veći prioritet (Dominant bit '0' nadvladava Recessive bit '1')
    pub fn arbitrate(node_a: &CanFrame, node_b: &CanFrame) -> CanFrame {
        if node_a.id <= node_b.id {
            node_a.clone() // Čvor A pobeđuje na bus-u i nastavlja slanje bez kolizije!
        } else {
            node_b.clone() // Čvor B pobeđuje
        }
    }
}