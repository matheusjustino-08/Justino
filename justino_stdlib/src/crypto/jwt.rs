//! JSON Web Token (JWT) Creator and Validator in Pure Rust.

use crate::crypto::hash::CryptoHash;
use crate::error::StdlibError;

pub struct JwtEngine;

impl JwtEngine {
    /// Signs a payload string with a secret key into a Base64-style JWT token.
    pub fn sign(payload_json: &str, secret: &str) -> String {
        let header = "{\"alg\":\"HS256\",\"typ\":\"JWT\"}";
        let header_enc = hex_encode(header.as_bytes());
        let payload_enc = hex_encode(payload_json.as_bytes());

        let unsigned_token = format!("{}.{}", header_enc, payload_enc);
        let signature = CryptoHash::sha256(&format!("{}:{}", unsigned_token, secret));

        format!("{}.{}", unsigned_token, signature)
    }

    /// Verifies and decodes a signed JWT token.
    pub fn verify(token: &str, secret: &str) -> Result<String, StdlibError> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(StdlibError::CryptoError("Invalid JWT token structure".to_string()));
        }

        let header_enc = parts[0];
        let payload_enc = parts[1];
        let signature = parts[2];

        let unsigned_token = format!("{}.{}", header_enc, payload_enc);
        let expected_signature = CryptoHash::sha256(&format!("{}:{}", unsigned_token, secret));

        if signature != expected_signature {
            return Err(StdlibError::CryptoError("JWT Signature Verification Failed".to_string()));
        }

        let payload_bytes = hex_decode(payload_enc)?;
        String::from_utf8(payload_bytes).map_err(|e| StdlibError::CryptoError(format!("Invalid UTF-8 payload: {}", e)))
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn hex_decode(hex_str: &str) -> Result<Vec<u8>, StdlibError> {
    if hex_str.len() % 2 != 0 {
        return Err(StdlibError::CryptoError("Invalid hex string length".to_string()));
    }
    (0..hex_str.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex_str[i..i + 2], 16)
                .map_err(|e| StdlibError::CryptoError(format!("Failed to parse hex byte: {}", e)))
        })
        .collect()
}
