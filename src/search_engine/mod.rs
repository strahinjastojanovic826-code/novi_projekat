pub mod document;
pub mod index;
pub mod tokenizer;

use std::collections::HashMap;
use document::{Document, SearchResult};
use index::InvertedIndex;

pub struct QuantumSearchEngine {
    pub documents: HashMap<usize, Document>,
    pub index: InvertedIndex,
    next_id: usize,
}

impl QuantumSearchEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            documents: HashMap::new(),
            index: InvertedIndex::new(),
            next_id: 1,
        };

        // Inicijalni podaci
        engine.index_doc(
            "Rust Programski Jezik",
            "Rust pruža sigurnost memorije bez garbage collector-a i vrhunske performanse.",
            vec!["rust".into(), "programming".into(), "os".into()],
        );
        engine.index_doc(
            "Quantum Kernel Architecture",
            "Sopstveni operativni sistem sa mikro-kernel arhitekturom i virtuelnom memorijom.",
            vec!["kernel".into(), "system".into(), "os".into()],
        );
        engine.index_doc(
            "In-Memory Database Engine",
            "Brzi klijent-server sistem baze podataka za keširanje i brzu pretragu u realnom vremenu.",
            vec!["db".into(), "search".into(), "memory".into()],
        );

        engine
    }

    pub fn index_doc(&mut self, title: &str, content: &str, tags: Vec<String>) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let doc = Document {
            id,
            title: title.to_string(),
            content: content.to_string(),
            tags,
        };

        let full_text = format!("{} {}", doc.title, doc.content);
        let tokens = tokenizer::tokenize(&full_text);

        self.index.index_document(id, &tokens);
        self.documents.insert(id, doc);

        id
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let query_tokens = tokenizer::tokenize(query);
        if query_tokens.is_empty() {
            return Vec::new();
        }

        let total_docs = self.documents.len();
        let avg_len = self.index.avg_doc_length();
        let mut doc_scores: HashMap<usize, (f32, Vec<String>)> = HashMap::new();

        for term in &query_tokens {
            if let Some(postings) = self.index.postings.get(term) {
                let doc_freq = postings.len();

                for (&doc_id, &tf) in postings {
                    let doc_len = *self.index.doc_lengths.get(&doc_id).unwrap_or(&1);
                    let score = self.index.calculate_bm25(tf, doc_len, avg_len, total_docs, doc_freq);

                    let entry = doc_scores.entry(doc_id).or_insert((0.0, Vec::new()));
                    entry.0 += score;
                    if !entry.1.contains(term) {
                        entry.1.push(term.clone());
                    }
                }
            }
        }

        let mut results: Vec<SearchResult> = doc_scores
            .into_iter()
            .filter_map(|(doc_id, (score, matched_terms))| {
                self.documents.get(&doc_id).map(|doc| SearchResult {
                    doc_id,
                    title: doc.title.clone(),
                    content_snippet: doc.content.clone(),
                    score,
                    matched_terms,
                })
            })
            .collect();

        // Sortiramo po BM25 skor opadajuće
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results
    }
}