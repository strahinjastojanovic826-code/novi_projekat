#[derive(Debug, Clone)]
pub struct BiosParameterBlock {
    pub oem_name: String,
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub num_fats: u8,
    pub root_cluster: u32,
    pub total_sectors: u32,
    pub volume_label: String,
}

#[derive(Debug, Clone)]
pub struct FatDirectoryEntry {
    pub name: String,
    pub ext: String,
    pub is_dir: bool,
    pub is_read_only: bool,
    pub start_cluster: u32,
    pub file_size_bytes: u32,
    pub path: String,
}

impl FatDirectoryEntry {
    pub fn full_name(&self) -> String {
        if self.is_dir {
            self.name.clone()
        } else if self.ext.is_empty() {
            self.name.clone()
        } else {
            format!("{}.{}", self.name, self.ext)
        }
    }
}