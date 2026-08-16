pub mod storage;
pub mod variable;

use storage::NvramStorage;
use variable::NvramAttribute;

pub struct QuantumNvramEngine {
    pub storage: NvramStorage,
    pub logs: Vec<String>,
}

impl QuantumNvramEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            storage: NvramStorage::new(64), // 64 KB NVRAM čip
            logs: Vec::new(),
        };

        engine.init_factory_defaults();
        engine
    }

    pub fn init_factory_defaults(&mut self) {
        let _ = self.storage.set("BootOrder", b"Disk0,NetBoot,USB", vec![NvramAttribute::NonVolatile, NvramAttribute::BootService]);
        let _ = self.storage.set("SecureBoot", b"Enabled", vec![NvramAttribute::NonVolatile, NvramAttribute::RuntimeService]);
        let _ = self.storage.set("DisplayMode", b"1920x1080@60Hz", vec![NvramAttribute::NonVolatile, NvramAttribute::RuntimeService]);
        let _ = self.storage.set("SystemVolume", b"80%", vec![NvramAttribute::NonVolatile, NvramAttribute::RuntimeService]);
        let _ = self.storage.set("FirmwareVersion", b"v2.4.0-QuantumUEFI", vec![NvramAttribute::ReadOnly, NvramAttribute::RuntimeService]);
        let _ = self.storage.set("KernelDebugFlags", b"0x00FF88A1", vec![NvramAttribute::NonVolatile, NvramAttribute::BootService]);

        self.logs.push("Učitana podrazumevana UEFI NVRAM podešavanja.".into());
    }

    pub fn set_var(&mut self, name: &str, val: &str) -> Result<(), String> {
        let res = self.storage.set(
            name,
            val.as_bytes(),
            vec![NvramAttribute::NonVolatile, NvramAttribute::RuntimeService],
        );

        if res.is_ok() {
            self.logs.push(format!("Ažuriran NVRAM: {} = {}", name, val));
        }
        res
    }

    pub fn get_var(&self, name: &str) -> Option<String> {
        self.storage.get(name).map(|v| v.get_string_val())
    }

    pub fn factory_reset(&mut self) {
        self.storage.memory.clear();
        self.storage.current_used_bytes = 0;
        self.init_factory_defaults();
        self.logs.push("⚠️ Izvršen fabrički reset NVRAM-a!".into());
    }
}