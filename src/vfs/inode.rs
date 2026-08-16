
pub type InodeId = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeType {
    File,
    Directory,
}

#[derive(Clone, Debug)]
pub struct Inode {
    pub id: InodeId,
    pub name: String,
    pub node_type: NodeType,
    pub parent: Option<InodeId>,
    pub children: Vec<InodeId>,
    
    // Blokovska alokacija (Direct Block Pointers)
    pub block_pointers: Vec<usize>,
    pub size_in_ququats: usize,
}

impl Inode {
    pub fn new_dir(id: InodeId, name: &str, parent: Option<InodeId>) -> Self {
        Self {
            id,
            name: name.to_string(),
            node_type: NodeType::Directory,
            parent,
            children: Vec::new(),
            block_pointers: Vec::new(),
            size_in_ququats: 0,
        }
    }

    pub fn new_file(id: InodeId, name: &str, parent: Option<InodeId>) -> Self {
        Self {
            id,
            name: name.to_string(),
            node_type: NodeType::File,
            parent,
            children: Vec::new(),
            block_pointers: Vec::new(),
            size_in_ququats: 0,
        }
    }
}