use crate::models::Database;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{self, Argon2};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use rand_core::RngCore;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::path::Path;
use base64::{Engine as _, engine::general_purpose};

#[derive(Serialize, Deserialize)]
struct EncryptedData {
    nonce: String,
    salt: String,
    data: String,
}

fn derive_key_with_salt(passkey: &str, salt_str: &str) -> Result<[u8; 32], String> {
    let salt = SaltString::from_b64(salt_str)
        .map_err(|e| format!("Error parsing salt: {}", e))?;
    
    let argon2 = Argon2::default();
    
    let mut key = [0u8; 32];
    
    argon2.hash_password_into(
        passkey.as_bytes(),
        salt.as_str().as_bytes(),
        &mut key
    ).map_err(|e| format!("Error deriving key: {}", e))?;
    
    Ok(key)
}

pub fn encrypt_and_save_database(database: &Database, filepath: &Path, passkey: &str) -> Result<(), String> {
    let json = serde_json::to_string(database)
        .map_err(|e| format!("Error serializing database: {}", e))?;
    
    let salt = SaltString::generate(&mut OsRng);
    let salt_string = salt.as_str();
    
    let key = derive_key_with_salt(passkey, salt_string)?;
    
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| format!("Error creating cipher: {}", e))?;
    
    let nonce = generate_nonce();
    let nonce_ref = Nonce::from_slice(&nonce);
    
    let ciphertext = cipher.encrypt(nonce_ref, json.as_bytes())
        .map_err(|e| format!("Error encrypting data: {}", e))?;
    
    let nonce_b64 = general_purpose::STANDARD.encode(nonce);
    let data_b64 = general_purpose::STANDARD.encode(ciphertext);
    
    let encrypted_data = EncryptedData {
        nonce: nonce_b64,
        salt: salt_string.to_string(),
        data: data_b64,
    };
    
    let encrypted_json = serde_json::to_string(&encrypted_data)
        .map_err(|e| format!("Error serializing encrypted data: {}", e))?;
    
    fs::write(filepath, encrypted_json)
        .map_err(|e| format!("Error writing to file: {}", e))?;
    
    Ok(())
}

pub fn load_and_decrypt_database(filepath: &Path, passkey: &str) -> Result<Database, String> {
    let file_content = fs::read_to_string(filepath)
        .map_err(|e| format!("Error reading file: {}", e))?;
    
    let encrypted_data: EncryptedData = serde_json::from_str(&file_content)
        .map_err(|e| format!("Error parsing file content: {}", e))?;
    
    let nonce_bytes = general_purpose::STANDARD.decode(&encrypted_data.nonce)
        .map_err(|e| format!("Error decoding nonce: {}", e))?;
    let ciphertext = general_purpose::STANDARD.decode(&encrypted_data.data)
        .map_err(|e| format!("Error decoding data: {}", e))?;
    
    let key = derive_key_with_salt(passkey, &encrypted_data.salt)?;
    
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| format!("Error creating cipher: {}", e))?;
    
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
        .map_err(|_| "Invalid passkey or corrupted database file".to_string())?;
    
    let database: Database = serde_json::from_slice(&plaintext)
        .map_err(|e| format!("Error parsing database: {}", e))?;
    
    Ok(database)
}

fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    nonce
}