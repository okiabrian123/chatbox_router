use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use aes::Aes256;
use aes::cipher::{
    BlockEncrypt, BlockDecrypt,
    KeyInit,
    generic_array::GenericArray,
};
use rand::Rng;
use std::sync::Arc;
use base64;
use ring::digest::{Context, SHA256};
use hex;

pub struct PasswordManager {
    file_path: PathBuf,
    key: String,
}

impl PasswordManager {
    pub fn new(service_name: &str, username: &str, key: &str) -> Result<Self> {
        let mut hasher = Context::new(&SHA256);
        hasher.update(key.as_bytes());
        let key = hex::encode(hasher.finish());
        
        // Buat nama file dari service dan username
        let mut hasher = Context::new(&SHA256);
        hasher.update(format!("{}:{}", service_name, username).as_bytes());
        let hash = hasher.finish();
        let filename = hex::encode(&hash.as_ref()[..16]);
        let file_path = PathBuf::from("/tmp").join(filename);
        
        Ok(PasswordManager { file_path, key })
    }

    pub fn set_password(&self, password: &Vec<u8>) -> Result<()> {
        let key_bytes = hex::decode(&self.key)
            .map_err(|e| anyhow!("Gagal decode key: {}", e))?;
        let encrypted = encrypt_data(password, &key_bytes)
            .map_err(|e| anyhow!("Gagal mengenkripsi password: {}", e))?;
        
        // Tulis ke file
        fs::write(&self.file_path, base64::encode(encrypted))
            .map_err(|e| anyhow!("Gagal menyimpan password: {}", e))
    }

    pub fn get_password(&self) -> Result<Vec<u8>> {
        // Baca dari file
        let encrypted = fs::read_to_string(&self.file_path)
            .map_err(|e| anyhow!("Gagal membaca password: {}", e))?;
        
        let decoded = base64::decode(encrypted)
            .map_err(|e| anyhow!("Gagal mendecode password: {}", e))?;
        let key_bytes = hex::decode(&self.key)
            .map_err(|e| anyhow!("Gagal decode key: {}", e))?;
        decrypt_data(&decoded, &key_bytes)
            .map_err(|e| anyhow!("Gagal mendekripsi password: {}", e))
    }

    pub fn delete_credential(&self) -> Result<()> {
        fs::remove_file(&self.file_path)
            .map_err(|e| anyhow!("Gagal menghapus kredensial: {}", e))
    }

    pub fn generate_random_key(&self, length: usize) -> Result<Vec<u8>> {
        let mut rng = rand::thread_rng();
        let key: Vec<u8> = (0..length).map(|_| rng.gen()).collect();
        Ok(key)
    }
}

pub fn encrypt_data(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256::new_from_slice(key)
        .map_err(|e| anyhow!("Gagal membuat cipher: {}", e))?;
    let mut encrypted_data = Vec::new();
    
    for chunk in data.chunks(16) {
        let mut block = GenericArray::from([0u8; 16]);
        block[..chunk.len()].copy_from_slice(chunk);
        cipher.encrypt_block(&mut block);
        encrypted_data.extend_from_slice(&block);
    }
    Ok(encrypted_data)
}

pub fn decrypt_data(encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256::new_from_slice(key)
        .map_err(|e| anyhow!("Gagal membuat cipher: {}", e))?;
    let mut decrypted_data = Vec::new();
    
    for chunk in encrypted_data.chunks(16) {
        let mut block = GenericArray::from([0u8; 16]);
        block.copy_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        decrypted_data.extend_from_slice(&block);
    }
    Ok(decrypted_data)
}
