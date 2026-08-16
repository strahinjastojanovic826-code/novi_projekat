use crate::domain::QuquatVal;

pub const BLOCK_SIZE_QUQUATS: usize = 8; // 1 blok = 8 kvata
pub const TOTAL_DISK_BLOCKS: usize = 512; // Ukupno 4096 kvata na disku

#[derive(Clone, Copy)]
pub struct DiskBlock {
    pub data: [QuquatVal; BLOCK_SIZE_QUQUATS],
}

pub struct VirtualQuquatDisk {
    pub blocks: Vec<DiskBlock>,
    pub free_bitmap: Vec<bool>,
}

impl VirtualQuquatDisk {
    pub fn new() -> Self {
        Self {
            blocks: vec![DiskBlock { data: [QuquatVal::Q00; BLOCK_SIZE_QUQUATS] }; TOTAL_DISK_BLOCKS],
            free_bitmap: vec![true; TOTAL_DISK_BLOCKS],
        }
    }

    pub fn allocate_block(&mut self) -> Result<usize, &'static str> {
        for (idx, is_free) in self.free_bitmap.iter_mut().enumerate() {
            if *is_free {
                *is_free = false;
                return Ok(idx);
            }
        }
        Err("ENOSPC: Virtuelni disk je popunjen!")
    }

    pub fn free_block(&mut self, block_idx: usize) {
        if block_idx < TOTAL_DISK_BLOCKS {
            self.free_bitmap[block_idx] = true;
        }
    }

    pub fn read_block(&self, block_idx: usize) -> Result<&DiskBlock, &'static str> {
        self.blocks.get(block_idx).ok_or("EIO: Greška pri čitanju sektora diska!")
    }

    pub fn write_block(&mut self, block_idx: usize, data: [QuquatVal; BLOCK_SIZE_QUQUATS]) -> Result<(), &'static str> {
        if let Some(block) = self.blocks.get_mut(block_idx) {
            block.data = data;
            Ok(())
        } else {
            Err("EIO: Greška pri pisanju na sektor diska!")
        }
    }
}