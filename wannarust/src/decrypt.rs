use aes::Aes256;
use cbc::Decryptor;
use cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
use rsa::pkcs8::DecodePrivateKey;
use rsa::Oaep;
use rsa::{pkcs1::DecodeRsaPrivateKey, RsaPrivateKey};
use sha2::Sha256;
use std::fs;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

pub fn decrypt_aes_key() -> Result<Vec<u8>, Box<dyn Error>> {
    let private_key_der = fs::read("private_key.der")?;
    let private_key = match RsaPrivateKey::from_pkcs1_der(&private_key_der) {
        Ok(k) => k,
        Err(_) => RsaPrivateKey::from_pkcs8_der(&private_key_der)?,
    };

    let encrypted_aes_key = fs::read("encryption.key")?;

    let padding = Oaep::new::<Sha256>();
    let decrypted_aes_key = private_key.decrypt(padding, &encrypted_aes_key)?;

    Ok(decrypted_aes_key)
}

pub fn decrypt_file(file_path: &PathBuf, aes_key: &[u8]) -> Result<(), Box<dyn Error>> {
    if aes_key.len() != 32 {
        return Err("Key must be exactly 32 bytes for AES-256".into());
    }

    let mut file = File::open(file_path)?;
    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data)?;

    if encrypted_data.len() < 16 {
        return Err("File too short: missing IV or data".into());
    }

    let mut file = File::open(file_path)?;
    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data)?;

    if encrypted_data.len() < 16 {
        return Err("File too short: missing IV or data".into());
    }

    let iv = &encrypted_data[..16];
    let ciphertext = &encrypted_data[16..];

    let decryptor = Decryptor::<Aes256>::new(aes_key.into(), iv.into());

    let mut buf = ciphertext.to_vec();
    let decrypted_data = decryptor
        .decrypt_padded_mut::<Pkcs7>(&mut buf)
        .map_err(|e| format!("Decryption error: {e}"))?;

    let mut out_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    out_file.write_all(decrypted_data)?;

    println!("✅ File decrypted: {}", file_path.display());

    Ok(())
}
