#[derive(Debug, Clone, PartialEq)]
pub enum LintLevel {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub struct LintIssue {
    pub line: usize,
    pub level: LintLevel,
    pub rule_id: String,
    pub message: String,
    pub suggestion: String,
}

pub struct LintRules;

impl LintRules {
    pub fn analyze(code: &str) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;

            // 1. Provera presugačkih linija (> 80 karaktera)
            if line.len() > 80 {
                issues.push(LintIssue {
                    line: line_num,
                    level: LintLevel::Warning,
                    rule_id: "L001_LINE_LENGTH".into(),
                    message: format!("Linija ima {} karaktera (preporučeno max 80).", line.len()),
                    suggestion: "Razbij liniju u više redova radi lakše čitljivosti.".into(),
                });
            }

            // 2. Provera nepotrebnih razmaka na kraju linije (Trailing Whitespace)
            if line.ends_with(' ') || line.ends_with('\t') {
                issues.push(LintIssue {
                    line: line_num,
                    level: LintLevel::Info,
                    rule_id: "L002_TRAILING_WHITESPACE".into(),
                    message: "Pronađen nepotreban prazan prostor na kraju linije.".into(),
                    suggestion: "Ukloni prazne razmake na kraju linije.".into(),
                });
            }

            // 3. Detekcija zaboravljenih TODO / FIXME komentara
            if line.contains("TODO") || line.contains("FIXME") {
                issues.push(LintIssue {
                    line: line_num,
                    level: LintLevel::Warning,
                    rule_id: "L003_TODO_FOUND".into(),
                    message: "Detektovan nezavršen zadatak (TODO/FIXME).".into(),
                    suggestion: "Reši komentar ili otvori zadatak u Task Manageru.".into(),
                });
            }

            // 4. Provera konvencije naziva funkcija (Snake_case u Rustu)
            if line.trim().starts_with("fn ") {
                if let Some(fn_name) = line.split_whitespace().nth(1) {
                    let clean_name = fn_name.split('(').next().unwrap_or("");
                    if clean_name.chars().any(|c| c.is_uppercase()) {
                        issues.push(LintIssue {
                            line: line_num,
                            level: LintLevel::Error,
                            rule_id: "L004_SNAKE_CASE_FN".into(),
                            message: format!("Naziv funkcije '{}' koristi CamelCase umesto snake_case.", clean_name),
                            suggestion: "Preimenuj funkciju u mala slova sa donjim crtama.".into(),
                        });
                    }
                }
            }

            // 5. Opasne komande ili prazni catch blokovi
            if line.contains("unwrap()") {
                issues.push(LintIssue {
                    line: line_num,
                    level: LintLevel::Warning,
                    rule_id: "L005_UNWRAP_USAGE".into(),
                    message: "Upotreba unwrap() može izazvati rušenje (panic) u produkciji.".into(),
                    suggestion: "Koristi 'match' ili 'if let' za bezbednu obradu grešaka.".into(),
                });
            }
        }

        issues
    }
}