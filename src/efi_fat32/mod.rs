pub mod fat;

use fat::{BiosParameterBlock, FatDirectoryEntry};

pub struct QuantumEfiFatEngine {
    pub bpb: BiosParameterBlock,
    pub directory_entries: Vec<FatDirectoryEntry>,
    pub fat_table: Vec<u32>, // Simulacija FAT32 tabele alokacije
    pub is_mounted: bool,
    pub logs: Vec<String>,
}

impl QuantumEfiFatEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            bpb: BiosParameterBlock {
                oem_name: "MSWIN4.1".into(),
                bytes_per_sector: 512,
                sectors_per_cluster: 8, // 4 KB klaster
                reserved_sectors: 32,
                num_fats: 2,
                root_cluster: 2,
                total_sectors: 1048576, // 512 MB ESP Particija
                volume_label: "SYSTEM_ESP".into(),
            },
            directory_entries: Vec::new(),
            fat_table: vec![0xFFFFFFF8, 0xFFFFFFFF, 3, 4, 0xFFFFFFFF, 6, 0xFFFFFFFF],
            is_mounted: false,
            logs: Vec::new(),
        };

        engine.mount_esp_partition();
        engine
    }

    pub fn mount_esp_partition(&mut self) {
        self.directory_entries = vec![
            FatDirectoryEntry {
                name: "EFI".into(),
                ext: "".into(),
                is_dir: true,
                is_read_only: false,
                start_cluster: 2,
                file_size_bytes: 0,
                path: "/EFI".into(),
            },
            FatDirectoryEntry {
                name: "BOOTX64".into(),
                ext: "EFI".into(),
                is_dir: false,
                is_read_only: true,
                start_cluster: 3,
                file_size_bytes: 245760, // 240 KB
                path: "/EFI/BOOT/BOOTX64.EFI".into(),
            },
            FatDirectoryEntry {
                name: "KERNEL".into(),
                ext: "EFI".into(),
                is_dir: false,
                is_read_only: true,
                start_cluster: 5,
                file_size_bytes: 8388608, // 8 MB Quantum Kernel Image
                path: "/EFI/QUANTUM/KERNEL.EFI".into(),
            },
            FatDirectoryEntry {
                name: "STARTUP".into(),
                ext: "NSH".into(),
                is_dir: false,
                is_read_only: false,
                start_cluster: 7,
                file_size_bytes: 128,
                path: "/EFI/BOOT/STARTUP.NSH".into(),
            },
        ];

        self.is_mounted = true;
        self.logs.push("FAT32: Uspešno parsiran BPB zaglavlje i montirana EFI System Particija (ESP).".into());
    }

    pub fn get_cluster_size_bytes(&self) -> u32 {
        self.bpb.bytes_per_sector as u32 * self.bpb.sectors_per_cluster as u32
    }

    pub fn read_mock_file_content(&self, path: &str) -> String {
        match path {
            "/EFI/BOOT/BOOTX64.EFI" => "PE32+ executable (EFI application) [Quantum Core Loader v2.4]".to_string(),
            "/EFI/QUANTUM/KERNEL.EFI" => "PE32+ executable (EFI driver) [QuantumOS Microkernel v1.0.4-x86_64]".to_string(),
            "/EFI/BOOT/STARTUP.NSH" => "@echo off\nload \\EFI\\QUANTUM\\KERNEL.EFI\nboot".to_string(),
            _ => "Greška: Nevaljala lokacija ili oštećen FAT32 klaster!".to_string(),
        }
    }
}