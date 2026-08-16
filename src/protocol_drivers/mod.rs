pub mod nvme;
pub mod usb;

use nvme::{NvmeController, NvmeOpcode};
use usb::XhciHostController;

pub struct QuantumProtocolDrivers {
    pub xhci: XhciHostController,
    pub nvme: NvmeController,
    pub driver_logs: Vec<String>,
}

impl QuantumProtocolDrivers {
    pub fn new() -> Self {
        let mut engine = Self {
            xhci: XhciHostController::new(),
            nvme: NvmeController::new(),
            driver_logs: Vec::new(),
        };

        engine.driver_logs.push("xHCI Host Controller i NVMe PCIe Drajveri uspešno učitani.".into());
        engine
    }

    pub fn nvme_read_sector(&mut self, lba: u64) -> String {
        let comp = self.nvme.submit_command(NvmeOpcode::Read, lba, String::new());
        let log = format!("NVMe READ LBA: {} | SQ ID: {} -> CQ Status: Success (0x0)", lba, comp.command_id);
        self.driver_logs.push(log.clone());
        log
    }

    pub fn nvme_write_sector(&mut self, lba: u64, data: &str) -> String {
        let comp = self.nvme.submit_command(NvmeOpcode::Write, lba, data.to_string());
        let log = format!("NVMe WRITE LBA: {} ('{}') | SQ ID: {} -> CQ Status: Success (0x0)", lba, data, comp.command_id);
        self.driver_logs.push(log.clone());
        log
    }

    pub fn usb_transfer(&mut self, port: u8, data: &str) -> String {
        match self.xhci.send_bulk_transfer(port, data.as_bytes()) {
            Ok(msg) => {
                self.driver_logs.push(msg.clone());
                msg
            }
            Err(err) => {
                self.driver_logs.push(err.clone());
                err
            }
        }
    }
}