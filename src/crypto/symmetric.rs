pub fn encrypt_aes_like(plaintext: &str, key: &str) -> Vec<u8> {
    let key_bytes = key.as_bytes();
    if key_bytes.is_empty() {
        return plaintext.as_bytes().to_vec();
    }

    plaintext
        .bytes()
        .enumerate()
        .map(|(i, b)| {
            let k = key_bytes[i % key_bytes.len()];
            b.wrapping_add(k).rotate_left(3) ^ k
        })
        .collect()
}

pub fn decrypt_aes_like(ciphertext: &[u8], key: &str) -> Result<String, String> {
    let key_bytes = key.as_bytes();
    if key_bytes.is_empty() {
        return String::from_utf8(ciphertext.to_vec()).map_err(|_| "Nevažeći UTF-8".into());
    }

    let bytes: Vec<u8> = ciphertext
        .iter()
        .enumerate()
        .map(|(i, &b)| {
            let k = key_bytes[i % key_bytes.len()];
            (b ^ k).rotate_right(3).wrapping_sub(k)
        })
        .collect();

    String::from_utf8(bytes).map_err(|_| "Greška pri dešifrovanju: Pogrešan ključ!".into())
}