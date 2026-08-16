pub mod rules;

use rules::{LintIssue, LintLevel, LintRules};

pub struct QuantumCodeLinterEngine {
    pub source_code: String,
    pub issues: Vec<LintIssue>,
    pub health_score: u32, // 0 do 100%
    pub formatted_code: String,
    pub logs: Vec<String>,
}

impl QuantumCodeLinterEngine {
    pub fn new() -> Self {
        let sample_code = r#"// QuantumOS Example Code
fn TestFunction() {
    let x = 10; // TODO: dodaj kalkulaciju
    let text = "Ovo je izuzetno dugačka linija koda koja definitvno prelazi osamdeset karaktera u jednom redu!"; 
    let res = x.unwrap();   
}
"#;

        let mut engine = Self {
            source_code: sample_code.to_string(),
            issues: Vec::new(),
            health_score: 100,
            formatted_code: String::new(),
            logs: Vec::new(),
        };

        engine.logs.push("Code Linter & Formatter Inicijalizovan [Rast pravila ugrađen].".into());
        engine.analyze();
        engine
    }

    pub fn set_code(&mut self, code: &str) {
        self.source_code = code.to_string();
        self.analyze();
    }

    pub fn analyze(&mut self) {
        self.issues = LintRules::analyze(&self.source_code);

        // Proračun ocene kvaliteta koda (Health Score)
        let mut score: i32 = 100;
        for issue in &self.issues {
            match issue.level {
                LintLevel::Error => score -= 20,
                LintLevel::Warning => score -= 10,
                LintLevel::Info => score -= 2,
            }
        }
        self.health_score = score.clamp(0, 100) as u32;

        self.logs.push(format!(
            "Analiza završena. Pronađeno problema: {} | Health Score: {}%",
            self.issues.len(),
            self.health_score
        ));
    }

    pub fn format_code(&mut self) {
        let mut lines: Vec<String> = Vec::new();
        let mut indent_level: i32 = 0;

        for line in self.source_code.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                lines.push(String::new());
                continue;
            }

            if trimmed.starts_with('}') {
                indent_level = indent_level.saturating_sub(1);
            }

            let indent = "    ".repeat(indent_level as usize);
            lines.push(format!("{}{}", indent, trimmed));

            if trimmed.ends_with('{') {
                indent_level += 1;
            }
        }

        self.formatted_code = lines.join("\n");
        self.source_code = self.formatted_code.clone();
        self.logs.push("Kod je uspešno auto-formatiran (Auto-Indentation & Trim).".into());
        self.analyze(); // Ponovna analiza osvežava greške
    }
}