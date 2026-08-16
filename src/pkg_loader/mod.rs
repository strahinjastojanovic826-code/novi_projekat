pub mod elf;
pub mod qmod;

use elf::{ElfFile, ElfParser};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub author: String,
    pub size_bytes: usize,
    pub is_loaded: bool,
}

#[derive(Debug, Clone)]
pub struct LoadedModule {
    pub name: String,
    pub base_address: u64,
    pub exported_symbols: Vec<String>,
    pub elf_details: Option<ElfFile>,
}

pub struct QuantumPkgEngine {
    pub installed_packages: HashMap<String, InstalledPackage>,
    pub loaded_modules: HashMap<String, LoadedModule>,
    pub repository_catalog: Vec<InstalledPackage>,
    pub logs: Vec<String>,
    next_load_address: u64,
}

impl QuantumPkgEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            installed_packages: HashMap::new(),
            loaded_modules: HashMap::new(),
            repository_catalog: Vec::new(),
            logs: Vec::new(),
            next_load_address: 0x7FFF0000,
        };

        engine.logs.push("Quantum Package Manager & Loader spreman.".into());
        engine.seed_demo_packages();
        engine
    }

    pub fn install_package(&mut self, pkg_name: &str) -> bool {
        if let Some(repo_pkg) = self.repository_catalog.iter().find(|p| p.name == pkg_name) {
            self.installed_packages.insert(pkg_name.to_string(), repo_pkg.clone());
            self.logs.push(format!("📦 INSTALIRAN PAKET: {} (v{})", repo_pkg.name, repo_pkg.version));
            true
        } else {
            self.logs.push(format!("❌ Paket '{}' nije pronađen u repozitorijumu.", pkg_name));
            false
        }
    }

    pub fn uninstall_package(&mut self, pkg_name: &str) -> bool {
        if self.installed_packages.remove(pkg_name).is_some() {
            self.unload_module(pkg_name);
            self.logs.push(format!("🗑️ UKLONJEN PAKET: {}", pkg_name));
            true
        } else {
            false
        }
    }

    pub fn load_qmod_bytes(&mut self, bytes: &[u8]) -> Result<String, String> {
        let mut mock_bytes = bytes.to_vec();
        if mock_bytes.is_empty() || !mock_bytes.starts_with(b"\x7FELF") {
            // Dodajemo pravi ELF magic ako ga nema radi demonstracije parsiranja
            let mut elf_demo = vec![0x7F, b'E', b'L', b'F', 2, 1, 1, 0];
            elf_demo.resize(128, 0);
            mock_bytes = elf_demo;
        }

        let parsed_elf = ElfParser::parse(&mock_bytes)?;
        let mod_name = format!("mod_0x{:X}", self.next_load_address);
        let base_addr = self.next_load_address;
        self.next_load_address += 0x00010000;

        let exported_symbols = parsed_elf.symbols.iter().map(|s| s.name.clone()).collect();

        let loaded_mod = LoadedModule {
            name: mod_name.clone(),
            base_address: base_addr,
            exported_symbols,
            elf_details: Some(parsed_elf),
        };

        self.loaded_modules.insert(mod_name.clone(), loaded_mod);
        self.logs.push(format!("⚙️ DINAMIČKI UČITAN MODUL: {} na 0x{:X}", mod_name, base_addr));

        Ok(mod_name)
    }

    pub fn unload_module(&mut self, mod_name: &str) -> bool {
        if self.loaded_modules.remove(mod_name).is_some() {
            if let Some(pkg) = self.installed_packages.get_mut(mod_name) {
                pkg.is_loaded = false;
            }
            self.logs.push(format!("🔌 IZbAČEN MODUL: {}", mod_name));
            true
        } else {
            false
        }
    }

    pub fn seed_demo_packages(&mut self) {
        self.repository_catalog.push(InstalledPackage {
            name: "quantum_crypto_ext".into(),
            version: "2.1.0".into(),
            author: "Quantum Core".into(),
            size_bytes: 48500,
            is_loaded: false,
        });

        self.repository_catalog.push(InstalledPackage {
            name: "net_filter_driver".into(),
            version: "1.0.4".into(),
            author: "NetSec".into(),
            size_bytes: 32100,
            is_loaded: false,
        });

        self.repository_catalog.push(InstalledPackage {
            name: "gpu_vulkan_sim".into(),
            version: "0.9.1".into(),
            author: "Graphics Team".into(),
            size_bytes: 124000,
            is_loaded: false,
        });

        // Auto instaliraj i učitaj jedan demo paket
        self.install_package("quantum_crypto_ext");
        let sample_payload = vec![0x7F, b'E', b'L', b'F', 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0];
        let _ = self.load_qmod_bytes(&sample_payload);
    }
}