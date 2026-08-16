#[derive(Debug, Clone)]
pub struct QModHeader {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub exported_symbols: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct QModPackage {
    pub header: QModHeader,
    pub binary_payload: Vec<u8>,
}

impl QModPackage {
    pub fn new(name: &str, version: &str, author: &str, payload: Vec<u8>) -> Self {
        Self {
            header: QModHeader {
                name: name.to_string(),
                version: version.to_string(),
                author: author.to_string(),
                description: format!("QuantumOS Dinamički Modul: {}", name),
                dependencies: vec!["core_kernel".into()],
                exported_symbols: vec![format!("{}_entry", name), format!("{}_cleanup", name)],
            },
            binary_payload: payload,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        // Magic header: QMOD
        data.extend_from_slice(b"QMOD");
        data.extend_from_slice(self.header.name.as_bytes());
        data.push(b':');
        data.extend_from_slice(self.header.version.as_bytes());
        data.push(b'\n');
        data.extend_from_slice(&self.binary_payload);
        data
    }
}