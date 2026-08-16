#[derive(Debug, Clone)]
pub struct Document {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub doc_id: usize,
    pub title: String,
    pub content_snippet: String,
    pub score: f32,
    pub matched_terms: Vec<String>,
}