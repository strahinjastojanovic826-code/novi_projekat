use std::collections::HashMap;

#[derive(Default)]
pub struct InvertedIndex {
    // Term -> HashMap<DocID, TermFrequency>
    pub postings: HashMap<String, HashMap<usize, usize>>,
    // DocID -> Ukupan broj reči u dokumentu
    pub doc_lengths: HashMap<usize, usize>,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            postings: HashMap::new(),
            doc_lengths: HashMap::new(),
        }
    }

    pub fn index_document(&mut self, doc_id: usize, tokens: &[String]) {
        self.doc_lengths.insert(doc_id, tokens.len());

        for token in tokens {
            self.postings
                .entry(token.clone())
                .or_default()
                .entry(doc_id)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    pub fn avg_doc_length(&self) -> f32 {
        if self.doc_lengths.is_empty() {
            return 0.0;
        }
        let total: usize = self.doc_lengths.values().sum();
        total as f32 / self.doc_lengths.len() as f32
    }

    // Okvirni Okapi BM25 algoritam
    pub fn calculate_bm25(
        &self,
        tf: usize,
        doc_len: usize,
        avg_len: f32,
        total_docs: usize,
        doc_freq: usize,
    ) -> f32 {
        let k1 = 1.2f32;
        let b = 0.75f32;

        let idf = ((total_docs as f32 - doc_freq as f32 + 0.5) / (doc_freq as f32 + 0.5) + 1.0).ln();
        let num = tf as f32 * (k1 + 1.0);
        let den = tf as f32 + k1 * (1.0 - b + b * (doc_len as f32 / (avg_len + 0.0001)));

        idf * (num / den)
    }
}