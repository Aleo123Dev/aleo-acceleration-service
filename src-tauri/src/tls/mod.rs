use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::OsRng;
use anyhow::Result;
use p256::elliptic_curve::ScalarPrimitive;
use hkdf::Hkdf;
use lazy_static::lazy_static;
use sha2::Sha256;

pub fn generate_p256_shared_secret(server_public_key: &[u8]) -> Result<Vec<u8>> {
    let client_private_key = GenericArray::from_slice(CLIENT_SECRET.as_slice());
    let secret = p256::SecretKey::from_bytes(client_private_key)?;
    let pubkey = p256::PublicKey::from_sec1_bytes(server_public_key)?;
    let shared = p256::ecdh::diffie_hellman(secret.to_nonzero_scalar(), pubkey.as_affine());
    Ok(shared.raw_secret_bytes().to_vec())
}

pub fn shared_secret_to_symmetric_secret(shares: &[u8]) -> Vec<u8> {
    let hk = Hkdf::<Sha256>::new(None, shares);
    let mut okm = [0u8; 32];
    hk.expand(&[], &mut okm)
        .expect("32 is a valid length for Sha256 to output");
    okm.to_vec()
}

pub fn generate_p256_secret() -> Result<Vec<u8>> {
    let sk = p256::SecretKey::new(ScalarPrimitive::random(&mut OsRng));
    let sk = sk.to_bytes();
    Ok(sk.to_vec())
}

pub fn get_p256_pubkey(sk: &[u8]) -> Vec<u8> {
    let sk_bytes = GenericArray::from_slice(sk);
    let sk = p256::SecretKey::from_bytes(sk_bytes).unwrap();
    let pubkey = sk.public_key();
    pubkey.to_sec1_bytes().to_vec()
}

//TODO: store the encrypted secret to fs
lazy_static! {
    pub static ref CLIENT_SECRET: Vec<u8> = generate_p256_secret().unwrap();
}
