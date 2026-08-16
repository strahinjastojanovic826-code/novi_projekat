#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedirectionType {
    Write(String),  // > (prepiši fajl)
    Append(String), // >> (dodaj na kraj)
    Read(String),   // < (čitaj iz fajla)
}

#[derive(Debug, Clone)]
pub struct PipelineStage {
    pub command: String,
    pub args: Vec<String>,
    pub redirection: Option<RedirectionType>,
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub stages: Vec<PipelineStage>,
}

impl Pipeline {
    pub fn parse(raw_line: &str) -> Self {
        let mut stages = Vec::new();

        // Delimo komandnu liniju po pipe operatoru |
        for pipe_segment in raw_line.split('|') {
            let segment = pipe_segment.trim();
            if segment.is_empty() {
                continue;
            }

            let mut redirection = None;
            let mut clean_segment = segment.to_string();

            // Provera za >> (Append)
            if let Some(pos) = segment.find(">>") {
                let file_path = segment[pos + 2..].trim().to_string();
                clean_segment = segment[..pos].trim().to_string();
                redirection = Some(RedirectionType::Append(file_path));
            }
            // Provera za > (Write/Overwrite)
            else if let Some(pos) = segment.find('>') {
                let file_path = segment[pos + 1..].trim().to_string();
                clean_segment = segment[..pos].trim().to_string();
                redirection = Some(RedirectionType::Write(file_path));
            }
            // Provera za < (Read)
            else if let Some(pos) = segment.find('<') {
                let file_path = segment[pos + 1..].trim().to_string();
                clean_segment = segment[..pos].trim().to_string();
                redirection = Some(RedirectionType::Read(file_path));
            }

            let parts: Vec<String> = clean_segment
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            if !parts.is_empty() {
                let command = parts[0].clone();
                let args = parts[1..].to_vec();
                stages.push(PipelineStage {
                    command,
                    args,
                    redirection,
                });
            }
        }

        Pipeline { stages }
    }
}