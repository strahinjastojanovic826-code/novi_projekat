pub mod inode;
pub mod block_dev;
pub mod file_descriptor;
pub mod sycalls;

pub use inode::{Inode, InodeId, NodeType};
pub use block_dev::{VirtualQuquatDisk, BLOCK_SIZE_QUQUATS, TOTAL_DISK_BLOCKS};
pub use file_descriptor::ProcessFDTable;

use crate::domain::QuquatVal;

pub struct VfsEntryInfo {
    pub inode: usize,
    pub name: String,
    pub is_directory: bool,
    pub size: usize,
}

pub struct QuquatVFS {
    pub nodes: Vec<Inode>,
    pub disk: VirtualQuquatDisk,
    pub current_dir: InodeId,
}

impl QuquatVFS {
    pub fn new() -> Self {
        let root = Inode::new_dir(0, "/", None);
        let mut vfs = Self {
            nodes: vec![root],
            disk: VirtualQuquatDisk::new(),
            current_dir: 0,
        };

        // Inicijalna struktura sistemskih direktorijuma
        let sys_id = vfs.mkdir(0, "sys").unwrap_or(0);
        let bin_id = vfs.mkdir(sys_id, "bin").unwrap_or(0);
        
        // Pravimo sistemski boot fajl i pišemo inicijalni bajtkod na disk
        if let Ok(file_id) = vfs.create_file(bin_id, "init.qbin") {
            let boot_code = vec![QuquatVal::Q01, QuquatVal::Q10, QuquatVal::Q11, QuquatVal::Q00];
            let _ = vfs.write_bytes_to_inode(file_id, 0, &boot_code);
        }

        vfs
    }

     pub fn read_dir(&self, _dir_inode: usize) -> Option<Vec<VfsEntryInfo>> {
    let mut entries = Vec::new();
    for node in &self.nodes {
        let is_directory = matches!(node.node_type, NodeType::Directory);

        entries.push(VfsEntryInfo {
            inode: node.id,
            name: node.name.clone(),
            is_directory,
            size: 0,
        });
    }
    Some(entries)
}

// 2. Čitanje sadržaja fajla za komandu 'cat'
pub fn read_file(&self, inode: usize) -> Option<Vec<u8>> {
    self.nodes.iter().find(|n| n.id == inode).map(|_| vec![])
}

    pub fn mkdir(&mut self, parent: InodeId, name: &str) -> Result<InodeId, &'static str> {
        let id = self.nodes.len();
        let new_node = Inode::new_dir(id, name, Some(parent));
        self.nodes.push(new_node);
        self.nodes[parent].children.push(id);
        Ok(id)
    }

    pub fn create_file(&mut self, parent: InodeId, name: &str) -> Result<InodeId, &'static str> {
        let id = self.nodes.len();
        let new_node = Inode::new_file(id, name, Some(parent));
        self.nodes.push(new_node);
        self.nodes[parent].children.push(id);
        Ok(id)
    }

    pub fn resolve_path(&self, path: &str) -> Option<InodeId> {
        if path == "/" { return Some(0); }
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut curr = 0;

        for part in parts {
            let mut found = false;
            for &child_id in &self.nodes[curr].children {
                if self.nodes[child_id].name == part {
                    curr = child_id;
                    found = true;
                    break;
                }
            }
            if !found { return None; }
        }
        Some(curr)
    }

    pub fn read_bytes_from_inode(&self, inode_id: InodeId, offset: usize, count: usize) -> Result<Vec<QuquatVal>, &'static str> {
        let node = self.nodes.get(inode_id).ok_or("EINVAL: Neuspešno čitanje Inode-a!")?;
        let mut result = Vec::new();

        for i in 0..count {
            let ququat_idx = offset + i;
            if ququat_idx >= node.size_in_ququats { break; }

            let block_list_idx = ququat_idx / BLOCK_SIZE_QUQUATS;
            let offset_in_block = ququat_idx % BLOCK_SIZE_QUQUATS;

            let physical_block = node.block_pointers[block_list_idx];
            let block = self.disk.read_block(physical_block)?;
            result.push(block.data[offset_in_block]);
        }

        Ok(result)
    }

    pub fn write_bytes_to_inode(&mut self, inode_id: InodeId, offset: usize, data: &[QuquatVal]) -> Result<usize, &'static str> {
        let mut written = 0;

        for &val in data {
            let ququat_idx = offset + written;
            let block_list_idx = ququat_idx / BLOCK_SIZE_QUQUATS;
            let offset_in_block = ququat_idx % BLOCK_SIZE_QUQUATS;

            // Proveri da li treba da alociramo novi blok na disku za ovaj Inode
            if block_list_idx >= self.nodes[inode_id].block_pointers.len() {
                let new_block = self.disk.allocate_block()?;
                self.nodes[inode_id].block_pointers.push(new_block);
            }

            let physical_block = self.nodes[inode_id].block_pointers[block_list_idx];
            let mut block_data = self.disk.read_block(physical_block)?.data;
            block_data[offset_in_block] = val;

            self.disk.write_block(physical_block, block_data)?;
            written += 1;
        }

        if offset + written > self.nodes[inode_id].size_in_ququats {
            self.nodes[inode_id].size_in_ququats = offset + written;
        }

        Ok(written)
    }
}