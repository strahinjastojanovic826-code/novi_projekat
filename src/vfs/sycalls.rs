use crate::domain::QuquatVal;
use crate::vfs::file_descriptor::{FileDescriptor, OpenFile, ProcessFDTable};
use crate::vfs::QuquatVFS;

pub struct KernelSyscallHandler;

impl KernelSyscallHandler {
    /// `sys_open`: Traži fajl po putanji, dodeljuje FD u tabelu procesa i postavlja kursor na 0
    pub fn sys_open(
        fd_table: &mut ProcessFDTable,
        vfs: &QuquatVFS,
        path: &str,
        flags: u8,
    ) -> Result<FileDescriptor, &'static str> {
        let inode_id = vfs.resolve_path(path).ok_or("ENOENT: Fajl ne postoji na navedenoj putanji!")?;
        
        let open_file = OpenFile {
            inode_id,
            offset: 0,
            flags,
        };

        fd_table.allocate(open_file)
    }

    /// `sys_read`: Čita 'count' kvata počevši od trenutnog offset-a i pomera offset unapred
    pub fn sys_read(
        fd_table: &mut ProcessFDTable,
        vfs: &QuquatVFS,
        fd: FileDescriptor,
        count: usize,
    ) -> Result<Vec<QuquatVal>, &'static str> {
        let open_file = fd_table.get_mut(fd).ok_or("EBADF: Nevažeći fajl deskriptor!")?;
        let data = vfs.read_bytes_from_inode(open_file.inode_id, open_file.offset, count)?;
        open_file.offset += data.len();
        Ok(data)
    }

    /// `sys_write`: Upisuje kvate u fajl i dodeljuje nove disk blokove po potrebi
    pub fn sys_write(
        fd_table: &mut ProcessFDTable,
        vfs: &mut QuquatVFS,
        fd: FileDescriptor,
        data: &[QuquatVal],
    ) -> Result<usize, &'static str> {
        let open_file = fd_table.get_mut(fd).ok_or("EBADF: Nevažeći fajl deskriptor!")?;
        let written = vfs.write_bytes_to_inode(open_file.inode_id, open_file.offset, data)?;
        open_file.offset += written;
        Ok(written)
    }

    /// `sys_close`: Oslobađa fajl deskriptor iz procesa
    pub fn sys_close(fd_table: &mut ProcessFDTable, fd: FileDescriptor) -> Result<(), &'static str> {
        if fd_table.close(fd) {
            Ok(())
        } else {
            Err("EBADF: Neuspešno zatvaranje deskriptora!")
        }
    }
}