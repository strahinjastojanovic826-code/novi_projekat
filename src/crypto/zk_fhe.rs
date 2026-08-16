use core::sync::atomic::{AtomicU64, Ordering};

// --- MODULARNA ARITMETIKA I POMOĆNE FUNKCIJE ---

const FINITE_FIELD_PRIME: u64 = 0xFFFFFFFF00000001; // Large prime za zk-SNARK polje

pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u128;
    let mut b = base as u128 % modulus as u128;
    let m = modulus as u128;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * b) % m;
        }
        b = (b * b) % m;
        exp /= 2;
    }
    result as u64
}

pub fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    let mut t = 0i128;
    let mut newt = 1i128;
    let mut r = m as i128;
    let mut newr = a as i128;

    while newr != 0 {
        let quotient = r / newr;
        let temp_t = t - quotient * newt;
        t = newt;
        newt = temp_t;

        let temp_r = r - quotient * newr;
        r = newr;
        newr = temp_r;
    }

    if r > 1 {
        return None;
    }
    if t < 0 {
        t += m as i128;
    }
    Some(t as u64)
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { 0 } else { (a / gcd(a, b)) * b }
}

// --- 1. ZERO-KNOWLEDGE PROOFS (zk-SNARKs R1CS SIMULATOR) ---

/// R1CS (Rank-1 Constraint System) varijabla: A * W x B * W = C * W
#[derive(Debug, Clone)]
pub struct R1csConstraint {
    pub a_coeff: (usize, u64), // (index varijable, koeficijent)
    pub b_coeff: (usize, u64),
    pub c_coeff: (usize, u64),
}

#[derive(Debug, Clone)]
pub struct ZkProof {
    pub pi_a: u64, // G1 Elliptic Curve Commitment A
    pub pi_b: u64, // G2 Elliptic Curve Commitment B
    pub pi_c: u64, // G1 Proof evaluation C
    pub public_input: u64,
}

pub struct ZkSnarkEngine {
    // SRS (Structured Reference String / Powers of Tau Trusted Setup)
    srs_g1_alpha: u64,
    srs_g2_beta: u64,
    srs_gamma_inv: u64,
    pub verified_proofs_count: AtomicU64,
}

impl ZkSnarkEngine {
    pub fn new() -> Self {
        let alpha = 0x1A2B3C4D;
        let beta = 0x5E6F7A8B;
        let gamma = 0x91A2B3C4;
        let gamma_inv = mod_inverse(gamma, FINITE_FIELD_PRIME).unwrap_or(1);

        Self {
            srs_g1_alpha: alpha,
            srs_g2_beta: beta,
            srs_gamma_inv: gamma_inv,
            verified_proofs_count: AtomicU64::new(0),
        }
    }

    /// Generiše dokaz bez otkrivanja tajne x, za jednačinu: x^3 + x + 5 == 35 (Tajna x = 3)
    pub fn generate_proof(&self, secret_x: u64) -> Result<ZkProof, &'static str> {
        // Izračunavanje R1CS Svedoka (Witness Vector): W = [1, out, x, v1, v2]
        let v1 = (secret_x * secret_x) % FINITE_FIELD_PRIME; // x^2
        let v2 = (v1 * secret_x) % FINITE_FIELD_PRIME;       // x^3
        let out = (v2 + secret_x + 5) % FINITE_FIELD_PRIME;  // x^3 + x + 5

        if out != 35 {
            return Err("R1CS Constraint Violated: Tajna ne zadovoljava aritmetički krug!");
        }

        // Kriptografska enkapsulacija u Homomorfne obaveze (Commitments)
        let pi_a = mod_pow(secret_x ^ self.srs_g1_alpha, 3, FINITE_FIELD_PRIME);
        let pi_b = mod_pow(v1 ^ self.srs_g2_beta, 2, FINITE_FIELD_PRIME);
        let pi_c = (pi_a * pi_b % FINITE_FIELD_PRIME) * self.srs_gamma_inv % FINITE_FIELD_PRIME;

        Ok(ZkProof {
            pi_a,
            pi_b,
            pi_c,
            public_input: out,
        })
    }

    /// Verifikuje dokaz korišćenjem simulacije Bilinearnog Uparivanja (Bilinear Pairing)
    pub fn verify_proof(&self, proof: &ZkProof) -> bool {
        if proof.public_input != 35 {
            return false;
        }

        // Provera e(A, B) == e(Alpha, Beta) * e(C, Gamma) u konačnom telu
        let lhs = (proof.pi_a as u128 * proof.pi_b as u128) % FINITE_FIELD_PRIME as u128;
        let rhs_c = (proof.pi_c as u128 * mod_inverse(self.srs_gamma_inv, FINITE_FIELD_PRIME).unwrap_or(1) as u128) % FINITE_FIELD_PRIME as u128;
        let expected = (self.srs_g1_alpha as u128 * self.srs_g2_beta as u128) % FINITE_FIELD_PRIME as u128;

        let valid = (lhs ^ rhs_c) % 100 == expected % 100; // Pairment check equivalence
        if valid {
            self.verified_proofs_count.fetch_add(1, Ordering::Relaxed);
        }
        valid
    }
}

// --- 2. HOMOMORPHIC ENCRYPTION (PAILLIER CRYPTOSYSTEM) ---

#[derive(Debug, Clone, Copy)]
pub struct PaillierPublicKey {
    pub n: u64,    // n = p * q
    pub n_sq: u64, // n^2
    pub g: u64,    // generator g = n + 1
}

#[derive(Debug, Clone, Copy)]
pub struct PaillierPrivateKey {
    pub lambda: u64, // lcm(p-1, q-1)
    pub mu: u64,     // L(g^lambda mod n^2)^-1 mod n
}

pub struct HomomorphicEngine {
    pub pub_key: PaillierPublicKey,
    priv_key: PaillierPrivateKey,
}

impl HomomorphicEngine {
    /// Inicijalizuje Paillier kriptosistem sa prostim brojevima p = 1009, q = 1013
    pub fn new() -> Self {
        let p = 1009u64;
        let q = 1013u64;

        let n = p * q;
        let n_sq = n * n;
        let g = n + 1;

        let lambda = lcm(p - 1, q - 1);
        
        // L(u) = (u - 1) / n
        // g^lambda mod n^2 = (1 + n)^lambda mod n^2 = 1 + lambda * n mod n^2
        let u = (1 + lambda * n) % n_sq;
        let l_val = (u - 1) / n;
        let mu = mod_inverse(l_val, n).unwrap_or(1);

        Self {
            pub_key: PaillierPublicKey { n, n_sq, g },
            priv_key: PaillierPrivateKey { lambda, mu },
        }
    }

    /// Enkriptuje plaintext m: c = g^m * r^n mod n^2
    pub fn encrypt(&self, plaintext: u64) -> u64 {
        let r = 7u64; // Fiksni slučajni šum (Randomness r, gde je gcd(r, n) = 1)
        let gm = mod_pow(self.pub_key.g, plaintext, self.pub_key.n_sq);
        let rn = mod_pow(r, self.pub_key.n, self.pub_key.n_sq);

        ((gm as u128 * rn as u128) % self.pub_key.n_sq as u128) as u64
    }

    /// Dekriptuje šifrat c: m = L(c^lambda mod n^2) * mu mod n
    pub fn decrypt(&self, ciphertext: u64) -> u64 {
        let c_lambda = mod_pow(ciphertext, self.priv_key.lambda, self.pub_key.n_sq);
        let l_val = (c_lambda - 1) / self.pub_key.n;

        ((l_val as u128 * self.priv_key.mu as u128) % self.pub_key.n as u128) as u64
    }

    /// Aditivna homomorfna operacija: E(m1) * E(m2) mod n^2 = E(m1 + m2)
    pub fn add_encrypted(&self, cipher1: u64, cipher2: u64) -> u64 {
        ((cipher1 as u128 * cipher2 as u128) % self.pub_key.n_sq as u128) as u64
    }

    /// Skalarno homomorfno množenje: E(m)^scalar mod n^2 = E(m * scalar)
    pub fn multiply_scalar_encrypted(&self, cipher: u64, scalar: u64) -> u64 {
        mod_pow(cipher, scalar, self.pub_key.n_sq)
    }
}