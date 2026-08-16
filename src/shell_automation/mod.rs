pub mod pipeline;
pub mod scripting;

use pipeline::{Pipeline, RedirectionType};
use scripting::ShellScript;
use std::collections::HashMap;

pub struct QuantumShellEngine {
    pub env_vars: HashMap<String, String>,
    pub scripts: Vec<ShellScript>,
    pub logs: Vec<String>,
    pub last_pipeline_output: String,
}

impl QuantumShellEngine {
    pub fn new() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert("USER".into(), "quantum_root".into());
        env_vars.insert("SHELL".into(), "/bin/qsh".into());
        env_vars.insert("PATH".into(), "/bin:/usr/bin:/quantum/sys".into());
        env_vars.insert("SYS_MODE".into(), "QUANTUM_STRICT".into());
        env_vars.insert("STATUS".into(), "OK".into());

        let mut engine = Self {
            env_vars,
            scripts: Vec::new(),
            logs: Vec::new(),
            last_pipeline_output: String::new(),
        };

        engine.logs.push("Quantum Shell Pipeline & Automation Engine spreman.".into());
        engine.seed_demo_scripts();
        engine
    }

    pub fn set_env(&mut self, key: &str, val: &str) {
        self.env_vars.insert(key.to_string(), val.to_string());
        self.logs.push(format!("💲 ENV: {}={}", key, val));
    }

    pub fn get_env(&self, key: &str) -> String {
        self.env_vars.get(key).cloned().unwrap_or_default()
    }

    // Zamena $VAR sa pravom vrednošću u tekstu komande
    pub fn expand_variables(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (k, v) in &self.env_vars {
            let var_pattern = format!("${}", k);
            result = result.replace(&var_pattern, v);
        }
        result
    }

    // Izvršavanje cevovoda (Pipes & Redirections)
    pub fn run_pipeline(&mut self, raw_line: &str) -> String {
        let expanded = self.expand_variables(raw_line);
        let pipeline = Pipeline::parse(&expanded);

        if pipeline.stages.is_empty() {
            return "Prazna komanda ili nevažeća sintaksa.".into();
        }

        self.logs.push(format!("🔗 EXEC PIPELINE: '{}'", expanded));

        let mut current_input = String::new();

        for (idx, stage) in pipeline.stages.iter().enumerate() {
            let mut output = format!(
                "[Stage {}: '{}' sa arg: {:?}] -> Input: '{}'",
                idx + 1,
                stage.command,
                stage.args,
                if current_input.is_empty() { "STDIN" } else { &current_input }
            );

            // Simulacija procesiranja komandi kroz pajp
            match stage.command.as_str() {
                "echo" => {
                    output = stage.args.join(" ");
                }
                "grep" => {
                    let pattern = stage.args.first().cloned().unwrap_or_default();
                    output = current_input
                        .lines()
                        .filter(|l| l.contains(&pattern))
                        .collect::<Vec<_>>()
                        .join("\n");
                    if output.is_empty() {
                        output = format!("(grep: nije pronađen uzorak '{}')", pattern);
                    }
                }
                "uppercase" | "tr_upper" => {
                    output = current_input.to_uppercase();
                }
                "wc" => {
                    let lines = current_input.lines().count();
                    let words = current_input.split_whitespace().count();
                    output = format!("Linija: {}, Reči: {}", lines, words);
                }
                _ => {
                    output = format!("Okidanje komande '{}' uspešno obrađeno.", stage.command);
                }
            }

            // Rukovanje preusmeravanjem (Redirections)
            if let Some(ref redir) = stage.redirection {
                match redir {
                    RedirectionType::Write(file) => {
                        self.logs.push(format!("💾 PREUSMERENJE (>): Zapisano u fajl '{}'", file));
                        output = format!("Output upisan u '{}'", file);
                    }
                    RedirectionType::Append(file) => {
                        self.logs.push(format!("➕ PREUSMERENJE (>>): Dodato na fajl '{}'", file));
                        output = format!("Output dodat u '{}'", file);
                    }
                    RedirectionType::Read(file) => {
                        self.logs.push(format!("📖 PREUSMERENJE (<): Pročitano iz fajla '{}'", file));
                        output = format!("Pročitan sadržaj fajla '{}'", file);
                    }
                }
            }

            current_input = output;
        }

        self.last_pipeline_output = current_input.clone();
        current_input
    }

    pub fn execute_script(&mut self, script_index: usize) -> Vec<String> {
        let mut results = Vec::new();
        if let Some(script) = self.scripts.get(script_index).cloned() {
            self.logs.push(format!("📜 POKRETANJE SKRIPTE: {}", script.name));
            for line in script.lines() {
                let res = self.run_pipeline(&line);
                results.push(format!("$ {} => {}", line, res));
            }
        } else {
            results.push("Skripta nije pronađena.".into());
        }
        results
    }

    pub fn seed_demo_scripts(&mut self) {
        let mut demo = ShellScript::new(
            "sys_health_check.qsh",
            r#"
# QuantumOS System Health Automation
echo Provera sistema pokrenuta od strane $USER
echo Mod sistema: $SYS_MODE
echo Provera logova | grep OK > /var/log/health.log
echo Obrada završena sa statusom $STATUS
"#,
        );
        demo.is_running = false;
        self.scripts.push(demo);
    }
}