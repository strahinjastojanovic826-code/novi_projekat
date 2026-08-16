pub mod device;

use device::{BootDevice, BootDeviceType};

pub struct QuantumBootEngine {
    pub devices: Vec<BootDevice>,
    pub uefi_mode: bool,
    pub fast_boot: bool,
    pub boot_logs: Vec<String>,
    pub last_boot_status: String,
}

impl QuantumBootEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            devices: vec![
                BootDevice {
                    id: "NVME0".into(),
                    name: "NVMe SSD - Quantum OS Core (512 GB)".into(),
                    dev_type: BootDeviceType::NVMe,
                    enabled: true,
                    is_connected: true,
                },
                BootDevice {
                    id: "USB0".into(),
                    name: "USB Kingston DataTraveler 3.0 (32 GB)".into(),
                    dev_type: BootDeviceType::UsbFlash,
                    enabled: true,
                    is_connected: true,
                },
                BootDevice {
                    id: "NET0".into(),
                    name: "Realtek Gigabit PXE Network Boot".into(),
                    dev_type: BootDeviceType::NetworkPxe,
                    enabled: false,
                    is_connected: true,
                },
                BootDevice {
                    id: "SATA0".into(),
                    name: "SATA SSD - Secondary Storage (1 TB)".into(),
                    dev_type: BootDeviceType::SataSSD,
                    enabled: true,
                    is_connected: true,
                },
            ],
            uefi_mode: true,
            fast_boot: true,
            boot_logs: Vec::new(),
            last_boot_status: "Sistem u stanju pripravnosti.".to_string(),
        };

        engine.boot_logs.push("Inicijalizovan Boot Manager. Učitana UEFI sekvenca.".into());
        engine
    }

    pub fn move_up(&mut self, index: usize) {
        if index > 0 && index < self.devices.len() {
            self.devices.swap(index, index - 1);
            self.boot_logs.push(format!("Prioritet promenjen: '{}' pomeren gore.", self.devices[index - 1].name));
        }
    }

    pub fn move_down(&mut self, index: usize) {
        if index < self.devices.len() - 1 {
            self.devices.swap(index, index + 1);
            self.boot_logs.push(format!("Prioritet promenjen: '{}' pomeren dole.", self.devices[index + 1].name));
        }
    }

    pub fn simulate_boot_sequence(&mut self) -> Result<String, String> {
        self.boot_logs.push("=== ZAPOČETA TEST BONUS SEKVENCA ===".into());

        for dev in &self.devices {
            if dev.enabled && dev.is_connected {
                let status = format!("Uspešno pročitan Boot Sector sa: '{}' [{:?}]", dev.name, dev.dev_type);
                self.last_boot_status = status.clone();
                self.boot_logs.push(status.clone());
                return Ok(status);
            } else {
                self.boot_logs.push(format!("Preskočen uređaj '{}' (Omogućen: {}, Povezan: {})", dev.name, dev.enabled, dev.is_connected));
            }
        }

        let err = "KRITIČNA GREŠKA: Nije pronađen nijedan bootabilan uređaj!".to_string();
        self.last_boot_status = err.clone();
        self.boot_logs.push(err.clone());
        Err(err)
    }
}