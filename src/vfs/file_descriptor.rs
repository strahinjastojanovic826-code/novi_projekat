pub type FileDescriptor = usize;

#[derive(Clone, Debug)]
pub struct OpenFile {
    pub inode_id: usize,
    pub offset: usize, // Kursor: trenutan položaj glave za čitanje
    pub flags: u8,     // 0 = Read, 1 = Write, 2 = ReadWrite
}

#[derive(Clone)]
pub struct ProcessFDTable {
    descriptors: Vec<Option<OpenFile>>,
    max_open_files: usize,
}

impl ProcessFDTable {
    pub fn new(max_open_files: usize) -> Self {
        let mut descriptors = Vec::with_capacity(max_open_files);
        // Standardne Unix/POSIX struje
        descriptors.push(None); // 0: STDIN
        descriptors.push(None); // 1: STDOUT
        descriptors.push(None); // 2: STDERR
        Self { descriptors, max_open_files }
    }

    pub fn allocate(&mut self, open_file: OpenFile) -> Result<FileDescriptor, &'static str> {
        for (fd, slot) in self.descriptors.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(open_file);
                return Ok(fd);
            }
        }
        if self.descriptors.len() < self.max_open_files {
            self.descriptors.push(Some(open_file));
            Ok(self.descriptors.len() - 1)
        } else {
            Err("EMFILE: Premašen limit otvorenih fajlova za ovaj proces!")
        }
    }

    pub fn get_mut(&mut self, fd: FileDescriptor) -> Option<&mut OpenFile> {
        self.descriptors.get_mut(fd)?.as_mut()
    }

    pub fn close(&mut self, fd: FileDescriptor) -> bool {
        if fd < self.descriptors.len() {
            self.descriptors[fd].take().is_some()
        } else {
            false
        }
    }
}