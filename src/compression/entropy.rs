#[derive(Debug, Clone)]
pub struct CompressionStats {
    pub original_size: usize,
    pub compressed_size: usize,
    pub ratio_percentage: f32,
    pub shannon_entropy: f64,
}

pub struct EntropyEngine;

impl EntropyEngine {
    /// Izračunava Shannon-ovu entropiju u bitovima po bajtu (0.0 do 8.0)
    pub fn calculate_shannon_entropy(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mut counts = [0u32; 256];
        for &byte in data {
            counts[byte as usize] += 1;
        }

        let len = data.len() as f64;
        let mut entropy = 0.0;

        for &count in &counts {
            if count > 0 {
                let p = count as f64 / len;
                entropy -= p * p.log2();
            }
        }

        entropy
    }

    /// RLE (Run-Length Encoding) kompresija
    pub fn compress_rle(data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut compressed = Vec::new();
        let mut current_byte = data[0];
        let mut count: u8 = 1;

        for &byte in &data[1..] {
            if byte == current_byte && count < 255 {
                count += 1;
            } else {
                compressed.push(count);
                compressed.push(current_byte);
                current_byte = byte;
                count = 1;
            }
        }
        compressed.push(count);
        compressed.push(current_byte);

        compressed
    }

    /// RLE dekompresija
    pub fn decompress_rle(data: &[u8]) -> Vec<u8> {
        let mut decompressed = Vec::new();
        let mut i = 0;

        while i + 1 < data.len() {
            let count = data[i];
            let byte = data[i + 1];
            decompressed.extend(vec![byte; count as usize]);
            i += 2;
        }

        decompressed
    }

    pub fn get_stats(original: &[u8], compressed: &[u8]) -> CompressionStats {
        let orig_sz = original.len();
        let comp_sz = compressed.len();
        let ratio = if orig_sz > 0 {
            (1.0 - (comp_sz as f32 / orig_sz as f32)) * 100.0
        } else {
            0.0
        };

        CompressionStats {
            original_size: orig_sz,
            compressed_size: comp_sz,
            ratio_percentage: ratio,
            shannon_entropy: Self::calculate_shannon_entropy(original),
        }
    }
}