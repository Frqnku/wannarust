use rsa::rand_core::{OsRng, RngCore};
use rsa::{RsaPrivateKey, RsaPublicKey};

pub fn generate_keys() -> Result<(RsaPrivateKey, RsaPublicKey, Vec<u8>), Box<dyn std::error::Error>>
{
    let mut rng = OsRng;

    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);

    let mut aes_key = [0u8; 32];
    rng.fill_bytes(&mut aes_key);

    Ok((private_key, public_key, aes_key.to_vec()))
}
