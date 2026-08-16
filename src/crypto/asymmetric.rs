#[derive(Debug, Clone)]
pub struct KeyPair {
    pub public_key: (u64, u64),  // (e, n)
    pub private_key: (u64, u64), // (d, n)
}

pub fn generate_rsa_keypair() -> KeyPair {
    // Odabrani jednostavni prosti brojevi za demonstraciju
    let p: u64 = 61;
    let q: u64 = 53;
    let n = p * q; // 3233
    let e: u64 = 17;
    let d: u64 = 2753; // Modularni inverz od e mod phi(n)

    KeyPair {
        public_key: (e, n),
        private_key: (d, n),
    }
}

pub fn sign_message(message: &str, private_key: &(u64, u64)) -> String {
    let digest = super::hashing::quantum_digest(message.as_bytes());
    // Uzimamo prvih 4 bajta digest-a za potpis
    let num = u32::from_str_radix(&digest[..4], 16).unwrap_or(123) as u64;

    let (d, n) = private_key;
    let mut sig = 1u64;
    for _ in 0..*d {
        sig = (sig * (num % n)) % n;
    }
    format!("{:04x}", sig)
}

pub fn verify_signature(message: &str, signature: &str, public_key: &(u64, u64)) -> bool {
    let expected_digest = super::hashing::quantum_digest(message.as_bytes());
    let num = u32::from_str_radix(&expected_digest[..4], 16).unwrap_or(123) as u64;

    let sig_num = u64::from_str_radix(signature, 16).unwrap_or(0);
    let (e, n) = public_key;

    let mut decrypted = 1u64;
    for _ in 0..*e {
        decrypted = (decrypted * (sig_num % n)) % n;
    }

    decrypted == (num % n)
}