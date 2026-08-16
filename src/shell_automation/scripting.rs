#[derive(Debug, Clone)]
pub struct ShellScript {
    pub name: String,
    pub code: String,
    pub is_running: bool,
}

impl ShellScript {
    pub fn new(name: &str, code: &str) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
            is_running: false,
        }
    }

    pub fn lines(&self) -> Vec<String> {
        self.code
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with('#')) // Ignorišemo komentare
            .map(|l| l.to_string())
            .collect()
    }
}