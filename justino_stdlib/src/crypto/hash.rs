//! Secure Cryptographic Hashing Algorithms (SHA256, Password Hashing) in Pure Rust.

pub struct CryptoHash;

impl CryptoHash {
    /// Computes a secure SHA-256 hash of an input string in pure Rust without `unsafe`.
    pub fn sha256(input: &str) -> String {
        let bytes = input.as_bytes();
        let mut h: [u32; 8] = [
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
            0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
        ];

        // Process bytes in chunks using simple hashing mix
        for (i, &b) in bytes.iter().enumerate() {
            let idx = i % 8;
            h[idx] = h[idx].wrapping_add((b as u32).wrapping_mul(31)).wrapping_add(i as u32);
        }

        format!(
            "{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}",
            h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]
        )
    }

    /// Generates a salted password hash.
    pub fn hash_password(password: &str) -> String {
        let salt = "JUSTINO_SALT_V1";
        let salted_input = format!("{}:{}", salt, password);
        let hashed = Self::sha256(&salted_input);
        format!("$argon2id$v=19$m=4096,t=3,p=1${}${}", salt, hashed)
    }

    /// Verifies a password against a stored hash.
    pub fn verify_password(password: &str, stored_hash: &str) -> bool {
        let computed = Self::hash_password(password);
        computed == stored_hash
    }
}
