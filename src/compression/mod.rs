pub mod entropy;

use entropy::{CompressionStats, EntropyEngine};

#[derive(Debug, Clone)]
pub struct ArchivedFile {
    pub name: String,
    pub uncompressed_size: usize,
    pub compressed_size: usize,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct QuantumArchive {
    pub archive_name: String,
    pub files: Vec<ArchivedFile>,
    pub total_raw_bytes: usize,
    pub total_compressed_bytes: usize,
}

pub struct QuantumArchiverEngine {
    pub active_archive: QuantumArchive,
    pub last_stats: Option<CompressionStats>,
    pub logs: Vec<String>,
}

impl QuantumArchiverEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            active_archive: QuantumArchive {
                archive_name: "system_backup.qarc".into(),
                files: Vec::new(),
                total_raw_bytes: 0,
                total_compressed_bytes: 0,
            },
            last_stats: None,
            logs: Vec::new(),
        };

        engine.logs.push("Archiver Engine Inicijalizovan [RLE / Shannon Entropy podmenadžer spreman].".into());
        
        // Dodavanje testnih fajlova u arhivu
        engine.add_file_to_archive("kernel_log.txt", "AAAAAABBBBBCCCCCDDDDD111122223333");
        engine.add_file_to_archive("config.json", "{\"status\": \"active\", \"mode\": \"quantum\"}");

        engine
    }

    pub fn add_file_to_archive(&mut self, name: &str, content: &str) {
        let raw_bytes = content.as_bytes();
        let compressed_bytes = EntropyEngine::compress_rle(raw_bytes);
        let stats = EntropyEngine::get_stats(raw_bytes, &compressed_bytes);

        let archived = ArchivedFile {
            name: name.to_string(),
            uncompressed_size: raw_bytes.len(),
            compressed_size: compressed_bytes.len(),
            data: compressed_bytes,
        };

        self.active_archive.total_raw_bytes += raw_bytes.len();
        self.active_archive.total_compressed_bytes += archived.compressed_size;
        self.active_archive.files.push(archived);

        self.last_stats = Some(stats.clone());
        self.logs.push(format!(
            "Arhiviran fajl '{}' | Entropija: {:.2} b/B | Ušteda: {:.1}%",
            name, stats.shannon_entropy, stats.ratio_percentage
        ));
    }

    pub fn test_raw_compression(&mut self, text: &str) -> CompressionStats {
        let raw = text.as_bytes();
        let compressed = EntropyEngine::compress_rle(raw);
        let stats = EntropyEngine::get_stats(raw, &compressed);
        self.last_stats = Some(stats.clone());
        
        self.logs.push(format!(
            "Test RLE kompresije: {}B -> {}B (Entropija: {:.2})",
            stats.original_size, stats.compressed_size, stats.shannon_entropy
        ));

        stats
    }
}