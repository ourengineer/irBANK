/// irBANK - Intellectual Property Bank
/// Security & Cryptography Module
/// Author: Algie Bookshelves
/// Purpose: Provides encryption, hashing, and security utilities

use sha2::{Sha256, Digest};
use argon2::{Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use argon2::password_hash::{SaltString, ParsingError};
use rand::Rng;
use hex::encode as hex_encode;
use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, KeyInit, Payload}};
use std::error::Error;

/// Secure password hashing using Argon2
pub fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    let salt = SaltString::generate(rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

/// Verify password against stored hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, Box<dyn Error>> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Generate SHA-256 hash for data integrity verification
pub fn generate_sha256_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex_encode(hasher.finalize())
}

/// Verify SHA-256 hash
pub fn verify_sha256_hash(data: &str, hash: &str) -> bool {
    generate_sha256_hash(data) == hash
}

/// Generate random cryptographic nonce for encryption
pub fn generate_nonce() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..12).map(|_| rng.gen::<u8>()).collect()
}

/// Generate random session token
pub fn generate_session_token(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..36);
            if idx < 10 {
                (b'0' + idx as u8) as char
            } else {
                (b'a' + (idx - 10) as u8) as char
            }
        })
        .collect()
}

/// Encrypt sensitive data using AES-256-GCM
pub fn encrypt_aes256_gcm(plaintext: &str, key: &[u8; 32]) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from(*key));
    let nonce = generate_nonce();
    let nonce_arr: [u8; 12] = nonce.as_slice().try_into()?;
    
    let ciphertext = cipher
        .encrypt(Nonce::from(nonce_arr), Payload::from(plaintext.as_bytes()))
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    
    let mut result = nonce.to_vec();
    result.extend(ciphertext);
    Ok(result)
}

/// Decrypt data encrypted with AES-256-GCM
pub fn decrypt_aes256_gcm(ciphertext: &[u8], key: &[u8; 32]) -> Result<String, Box<dyn Error>> {
    if ciphertext.len() < 12 {
        return Err("Ciphertext too short".into());
    }
    
    let (nonce_bytes, encrypted) = ciphertext.split_at(12);
    let nonce_arr: [u8; 12] = nonce_bytes.try_into()?;
    
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from(*key));
    let plaintext = cipher
        .decrypt(Nonce::from(nonce_arr), encrypted)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    
    Ok(String::from_utf8(plaintext)?)
}

/// Hash SSN for secure storage
pub fn hash_ssn(ssn: &str) -> String {
    generate_sha256_hash(&format!("ssn_{}_{}", ssn, "irbank_salt_2026"))
}

/// Generate device fingerprint from user agent and IP
pub fn generate_device_fingerprint(user_agent: &str, ip_address: &str) -> String {
    generate_sha256_hash(&format!("{}_{}", user_agent, ip_address))
}

/// Generate transaction verification hash
pub fn generate_transaction_hash(
    transaction_id: &str,
    amount: &str,
    account_id: &str,
    timestamp: &str,
) -> String {
    let data = format!("{}|{}|{}|{}", transaction_id, amount, account_id, timestamp);
    generate_sha256_hash(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "TestPassword123!@#";
        let hash = hash_password(password).unwrap();
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("WrongPassword", &hash).unwrap());
    }

    #[test]
    fn test_sha256_hashing() {
        let data = "test_data";
        let hash = generate_sha256_hash(data);
        assert!(verify_sha256_hash(data, &hash));
        assert!(!verify_sha256_hash("different_data", &hash));
    }

    #[test]
    fn test_session_token_generation() {
        let token = generate_session_token(64);
        assert_eq!(token.len(), 64);
    }
}
