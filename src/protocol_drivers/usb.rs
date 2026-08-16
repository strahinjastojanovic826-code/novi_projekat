#[derive(Debug, Clone, PartialEq)]
pub enum UsbSpeed {
    FullSpeed,  // 12 Mbps
    HighSpeed,  // 480 Mbps
    SuperSpeed, // 5 Gbps
}

#[derive(Debug, Clone)]
pub struct UsbDevice {
    pub port: u8,
    pub vendor_id: u16,
    pub product_id: u16,
    pub speed: UsbSpeed,
    pub name: String,
    pub connected: bool,
}

pub struct XhciHostController {
    pub ports: Vec<UsbDevice>,
    pub transfer_count: usize,
}

impl XhciHostController {
    pub fn new() -> Self {
        Self {
            ports: vec![
                UsbDevice {
                    port: 1,
                    vendor_id: 0x0951,
                    product_id: 0x1666,
                    speed: UsbSpeed::SuperSpeed,
                    name: "Kingston DataTraveler 3.0".into(),
                    connected: true,
                },
                UsbDevice {
                    port: 2,
                    vendor_id: 0x046D,
                    product_id: 0xC52B,
                    speed: UsbSpeed::FullSpeed,
                    name: "Logitech Unifying Receiver".into(),
                    connected: true,
                },
            ],
            transfer_count: 0,
        }
    }

    pub fn send_bulk_transfer(&mut self, port: u8, data: &[u8]) -> Result<String, String> {
        if let Some(dev) = self.ports.iter().find(|d| d.port == port && d.connected) {
            self.transfer_count += 1;
            Ok(format!(
                "xHCI Bulk Transfer uspešan [{:?}] -> {} (Poslato {} B, TX ID: #{})",
                dev.speed, dev.name, data.len(), self.transfer_count
            ))
        } else {
            Err(format!("xHCI Greška: Nema povezanog USB uređaja na portu {}!", port))
        }
    }
}