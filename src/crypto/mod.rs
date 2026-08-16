pub mod asymmetric;
pub mod hashing;
pub mod symmetric;
pub mod tls;
pub mod zk_fhe;

use asymmetric::{KeyPair, generate_rsa_keypair};
use tls::TlsSession;

pub struct QuantumCryptoEngine {
    pub rsa_keys: KeyPair,
    pub tls_session: TlsSession,
    pub last_hash: String,
    pub last_encrypted_bytes: Vec<u8>,
    pub last_decrypted_text: String,
}

impl QuantumCryptoEngine {
    pub fn new() -> Self {
        Self {
            rsa_keys: generate_rsa_keypair(),
            tls_session: TlsSession::new(),
            last_hash: String::new(),
            last_encrypted_bytes: Vec::new(),
            last_decrypted_text: String::new(),
        }
    }

    pub fn hash_text(&mut self, text: &str) -> String {
        self.last_hash = hashing::quantum_digest(text.as_bytes());
        self.last_hash.clone()
    }

    pub fn encrypt_text(&mut self, text: &str, key: &str) {
        self.last_encrypted_bytes = symmetric::encrypt_aes_like(text, key);
    }

    pub fn decrypt_bytes(&mut self, bytes: &[u8], key: &str) {
        match symmetric::decrypt_aes_like(bytes, key) {
            Ok(txt) => self.last_decrypted_text = txt,
            Err(err) => self.last_decrypted_text = err,
        }
    }
}