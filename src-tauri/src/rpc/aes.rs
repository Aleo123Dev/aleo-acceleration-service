use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, Nonce},
    Aes256Gcm, KeyInit,
};
use anyhow::Result;

pub fn aes_decode(key: &[u8], nonce: &[u8], cipher_text: &[u8]) -> Result<Vec<u8>> {
    let key = GenericArray::from_slice(key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce);
    let plaintext = cipher
        .decrypt(nonce, cipher_text)
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(plaintext)
}
