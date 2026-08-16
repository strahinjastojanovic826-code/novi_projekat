pub fn quantum_digest(data: &[u8]) -> String {
    let mut h1: u32 = 0x6A09E667;
    let mut h2: u32 = 0xBB67AE85;

    for (i, &byte) in data.iter().enumerate() {
        h1 = h1.wrapping_add((byte as u32).wrapping_mul(31)).rotate_left(5);
        h2 = h2.wrapping_pow((byte as u32).wrapping_add(i as u32)).rotate_right(3);
    }

    format!("{:08x}{:08x}", h1, h2)
}

pub fn hmac(key: &str, message: &str) -> String {
    let combined = format!("{}:{}", key, message);
    quantum_digest(combined.as_bytes())
}