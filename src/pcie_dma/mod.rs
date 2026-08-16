use std::collections::HashMap;

// =============================================================================
// 1. IOMMU (INPUT-OUTPUT MEMORY MANAGEMENT UNIT)
// =============================================================================

/// IOMMU tabela prevođenja: IOVA (I/O Virtual Address) -> HPA (Host Physical Address)
pub struct Iommu {
    pub page_table: HashMap<u64, u64>,
    pub enabled: bool,
}

impl Iommu {
    pub fn new() -> Self {
        Self {
            page_table: HashMap::new(),
            enabled: true,
        }
    }

    pub fn map_page(&mut self, iova: u64, physical_addr: u64) {
        self.page_table.insert(iova, physical_addr);
    }

    /// Prevodi I/O virtualnu adresu u fizičku adresu u RAM-u
    pub fn translate(&self, iova: u64) -> Result<u64, &'static str> {
        if !self.enabled {
            // Bez IOMMU-a, IOVA se direktno tretira kao fizička adresa (Ranjivo na DMA attacks!)
            return Ok(iova);
        }

        self.page_table
            .get(&iova)
            .cloned()
            .ok_or("IOMMU PAGE FAULT: PCIe Uređaj je pokušao pristup neautorizovanoj memoriji!")
    }
}

// =============================================================================
// 2. SCATTER-GATHER DMA DESKRIPTORI & TLP PAKETI
// =============================================================================

/// Ring Buffer entry za DMA prenos
#[derive(Debug, Clone, Copy)]
pub struct DmaDescriptor {
    pub buffer_iova: u64, // I/O Virtualna adresa u RAM-u gde uređaj treba da upiše podatke
    pub length: usize,    // Max kapacitet bafera
    pub is_owned_by_device: bool, // Flag: 1 = Uređaj popunjava, 0 = CPU čita
}

/// Transaction Layer Packet (TLP) koji putuje kroz PCIe Sabirnicu
#[derive(Debug, Clone)]
pub enum TlpPacket {
    MemoryRead { iova: u64, length: usize },
    MemoryWrite { iova: u64, payload: Vec<u8> },
}

// =============================================================================
// 3. PCIE DEVICE & DMA ENGINE
// =============================================================================

pub struct PcieDeviceSim {
    pub name: String,
    pub vendor_id: u16,
    pub device_id: u16,
    pub bar0_mmio_base: u64, // MMIO Registar za konfiguraciju
}

impl PcieDeviceSim {
    pub fn new(name: &str, vendor_id: u16, device_id: u16, bar0: u64) -> Self {
        Self {
            name: name.to_string(),
            vendor_id,
            device_id,
            bar0_mmio_base: bar0,
        }
    }

    /// Izvršava Bus Master DMA Write: Upisuje podatke direktno u Sistemski RAM bez učešća CPU-a
    pub fn execute_dma_transfer(
        &self,
        descriptor: &mut DmaDescriptor,
        incoming_data: &[u8],
        iommu: &Iommu,
        system_ram: &mut [u8],
    ) -> Result<usize, &'static str> {
        if !descriptor.is_owned_by_device {
            return Err("DMA Error: Deskriptor je još uvek u vlasništvu CPU-a!");
        }

        // 1. Prevođenje IOVA -> Physical Address preko IOMMU-a
        let phys_addr = iommu.translate(descriptor.buffer_iova)?;

        let bytes_to_copy = incoming_data.len().min(descriptor.length);
        let start_idx = phys_addr as usize;
        let end_idx = start_idx + bytes_to_copy;

        if end_idx > system_ram.len() {
            return Err("DMA Error: Upis prelazi fizičke granice RAM-a!");
        }

        // 2. Generisanje PCIe TLP Memory Write i direktan upis u RAM (Direct Access)
        system_ram[start_idx..end_idx].copy_from_slice(&incoming_data[..bytes_to_copy]);

        // 3. Vraćanje vlasništva nad deskriptorom CPU-u
        descriptor.is_owned_by_device = false;

        Ok(bytes_to_copy)
    }
}