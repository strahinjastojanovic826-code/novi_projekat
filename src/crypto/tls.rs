#[derive(Debug, Clone, PartialEq)]
pub enum TlsState {
    Disconnected,
    ClientHelloSent,
    ServerHelloReceived,
    KeyExchangeDone,
    EncryptedSessionActive,
}

pub struct TlsSession {
    pub state: TlsState,
    pub cipher_suite: String,
    pub client_random: String,
    pub server_random: String,
    pub master_secret: String,
    pub handshake_logs: Vec<String>,
}

impl TlsSession {
    pub fn new() -> Self {
        Self {
            state: TlsState::Disconnected,
            cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
            client_random: String::new(),
            server_random: String::new(),
            master_secret: String::new(),
            handshake_logs: Vec::new(),
        }
    }

    pub fn execute_handshake(&mut self) {
        self.handshake_logs.clear();

        // 1. ClientHello
        self.client_random = super::hashing::quantum_digest(b"client_seed_123");
        self.state = TlsState::ClientHelloSent;
        self.handshake_logs.push(format!("➡️ [ClientHello] Nasumični niz: {}", &self.client_random[..8]));

        // 2. ServerHello
        self.server_random = super::hashing::quantum_digest(b"server_seed_987");
        self.state = TlsState::ServerHelloReceived;
        self.handshake_logs.push(format!("⬅️ [ServerHello] Izabran algoritam: {}", self.cipher_suite));

        // 3. Key Exchange (Diffie-Hellman Simulation)
        let combined_secret = format!("{}{}", self.client_random, self.server_random);
        self.master_secret = super::hashing::quantum_digest(combined_secret.as_bytes());
        self.state = TlsState::KeyExchangeDone;
        self.handshake_logs.push(format!("🔑 [Key Exchange] Generisan Master Secret: {}", &self.master_secret[..12]));

        // 4. Encrypted Session Established
        self.state = TlsState::EncryptedSessionActive;
        self.handshake_logs.push("🔒 [TLS 1.3] Sigurna enkriptovana sesija uspostavljena!".to_string());
    }
}