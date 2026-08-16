use crate::domain::QuquatVal;
use crate::vfs::QuquatVFS;
use super::storage::StorageEngine;

pub struct DbSnapshotter;

impl DbSnapshotter {
    /// Čuva trenutno stanje baze podataka u VFS fajl `/sys/qstore.db`
    pub fn save_to_vfs(storage: &StorageEngine, vfs: &mut QuquatVFS) -> Result<usize, &'static str> {
        let mut raw_bytes = Vec::new();

        for (key, record) in &storage.index {
            // Dodajemo dužinu ključa i sam ključ pretvorene u kvatne vrednosti
            for byte in key.bytes() {
                let q_val = match byte % 4 {
                    0 => QuquatVal::Q00,
                    1 => QuquatVal::Q01,
                    2 => QuquatVal::Q10,
                    _ => QuquatVal::Q11,
                };
                raw_bytes.push(q_val);
            }
            // Sadržaj sloga
            raw_bytes.extend_from_slice(&record.data);
        }

        let sys_inode = vfs.resolve_path("/sys").ok_or("ENOENT: /sys direktorijum ne postoji!")?;
        let db_file_id = match vfs.resolve_path("/sys/qstore.db") {
            Some(id) => id,
            None => vfs.create_file(sys_inode, "qstore.db")?,
        };

        vfs.write_bytes_to_inode(db_file_id, 0, &raw_bytes)
    }
}