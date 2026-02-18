use std::{fs::File, io, io::Write};
mod passwd;
use argon2::{Params, password_hash::rand_core::RngCore};
use chacha20poly1305::{
    self, XChaCha20Poly1305, XNonce,
    aead::{Aead, AeadCore, KeyInit, OsRng, Payload},
};
use crypto_common::InvalidLength;
use passwd::getkey;
#[derive(Debug)]
pub enum Error {
    Kdf(argon2::Error),
    Io(io::Error),
    Enc(String),
}
impl From<InvalidLength> for Error {
    fn from(_e: InvalidLength) -> Self {
        Self::Enc("Invalid Length".to_string())
    }
}
impl From<argon2::Error> for Error {
    fn from(e: argon2::Error) -> Self {
        Self::Kdf(e)
    }
}
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
fn default_params() -> Result<Params, argon2::Error> {
    Params::new(
        64 * 1024, // 64MiB memory cost
        4,         // 4 iterations
        4,         // 4 threads
        Some(32),  // 32-byte output
    )
}
const MAGIC_BYTES: [u8; 8] = *b"oxivault";
pub fn encrypt_file(plaintext: &[u8], file: &mut File) -> Result<(), Error> {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    let key = getkey(&salt, default_params()?)?;
    let cipher = XChaCha20Poly1305::new_from_slice(key.as_slice())?;
    let nonce = XChaCha20Poly1305::generate_nonce(OsRng);
    let mut aad = Vec::new();
    aad.extend_from_slice(&MAGIC_BYTES);
    aad.extend_from_slice(nonce.as_slice());
    aad.extend_from_slice(&salt);
    let Ok(ciphertext) = cipher.encrypt(
        &nonce,
        Payload {
            msg: plaintext,
            aad: &aad[..],
        },
    ) else {
        return Err(Error::Enc("Error in encryption".to_string()));
    };
    drop(cipher);
    drop(key);
    file.set_len(0)?;
    file.write_all(&MAGIC_BYTES)?;
    file.write_all(nonce.as_slice())?;
    file.write_all(&salt)?;
    file.write_all(&ciphertext)?;
    Ok(())
}
pub fn decrypt_file(ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
    if !ciphertext.starts_with(&MAGIC_BYTES) {
        return Err(Error::Enc("Magic Bytes do not match".to_string()));
    }
    #[allow(clippy::no_effect_underscore_binding)]
    let nonce: &XNonce = XNonce::from_slice(&ciphertext[8..32]);
    let salt = &ciphertext[32..48];
    let key = getkey(salt, default_params()?)?;
    let cipher = XChaCha20Poly1305::new_from_slice(key.as_slice())?;
    let mut aad = Vec::new();
    aad.extend_from_slice(&MAGIC_BYTES);
    aad.extend_from_slice(nonce.as_slice());
    aad.extend_from_slice(salt);
    let Ok(plaintext) = cipher.decrypt(
        nonce,
        Payload {
            msg: &ciphertext[48..],
            aad: &aad[..],
        },
    ) else {
        return Err(Error::Enc("Error in decryption".to_string()));
    };
    drop(cipher);
    drop(key);
    Ok(plaintext)
}
