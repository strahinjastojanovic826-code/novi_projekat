pub mod command;

#[derive(Debug, Clone)]
pub enum LineType {
    Prompt,
    Info,
    Success,
    Error,
    Warning,
    Header,
}

#[derive(Debug, Clone)]
pub struct TerminalLine {
    pub text: String,
    pub line_type: LineType,
}

pub struct TerminalEngine {
    pub input_buffer: String,
    pub output_lines: Vec<TerminalLine>,
    pub command_history: Vec<String>,
    pub history_index: usize,
}

impl TerminalEngine {
    pub fn new() -> Self {
        let mut term = Self {
            input_buffer: String::new(),
            output_lines: Vec::new(),
            command_history: Vec::new(),
            history_index: 0,
        };

        term.print("QuantumOS Terminal Emulator (QShell v1.0)", LineType::Success);
        term.print("Kucajte 'help' za spisak dostupnih komandi.\n", LineType::Info);
        term
    }

    pub fn print(&mut self, text: &str, line_type: LineType) {
        self.output_lines.push(TerminalLine {
            text: text.to_string(),
            line_type,
        });
    }

    pub fn clear(&mut self) {
        self.output_lines.clear();
    }
}