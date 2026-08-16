#[derive(Debug, Clone, PartialEq)]
pub enum BootDeviceType {
    NVMe,
    SataSSD,
    UsbFlash,
    NetworkPxe,
    OpticalDrive,
}

#[derive(Debug, Clone)]
pub struct BootDevice {
    pub id: String,
    pub name: String,
    pub dev_type: BootDeviceType,
    pub enabled: bool,
    pub is_connected: bool,
}