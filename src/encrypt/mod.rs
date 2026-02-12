use std::{
    fs::File,
    io::{Error, Write},
};
mod passwd;
use argon2::password_hash::rand_core::RngCore;
use chacha20poly1305::{
    XChaCha20Poly1305, XNonce,
    aead::{Aead, AeadCore, KeyInit, OsRng, Payload},
};
use passwd::getkey;

const MAGIC_BYTES: [u8; 8] = *b"oxivault";
const VERSION: [u8; 3] = [0, 0, 1];
pub fn encrypt_file(plaintext: &[u8], file: &mut File) -> Result<(), Error> {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    let key = getkey(&salt);
    let cipher = match XChaCha20Poly1305::new_from_slice(key.as_slice()) {
        Ok(c) => c,
        Err(_) => {
            drop(key);
            panic!("Creating Cipher Failed.");
        }
    };
    let nonce = XChaCha20Poly1305::generate_nonce(OsRng);
    let mut aad = Vec::new();
    aad.extend_from_slice(&MAGIC_BYTES);
    aad.extend_from_slice(&VERSION);
    aad.extend_from_slice(nonce.as_slice());
    aad.extend_from_slice(&salt);
    let ciphertext = match cipher.encrypt(
        &nonce,
        Payload {
            msg: plaintext,
            aad: &aad[..],
        },
    ) {
        Ok(c) => c,
        Err(_) => return Err(Error::other("Error in encryption")),
    };
    drop(cipher);
    drop(key);
    file.set_len(0)?;
    file.write_all(&MAGIC_BYTES)?;
    file.write_all(&VERSION)?;
    file.write_all(nonce.as_slice())?;
    file.write_all(&salt)?;
    file.write_all(&ciphertext)?;
    Ok(())
}
pub fn decrypt_file(ciphertext: &[u8]) -> Result<Vec<u8>, Error> {
    if !ciphertext.starts_with(&MAGIC_BYTES) {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Error: Magic Bytes do not match. Are you sure this is a Oxivault file?",
        ));
    }
    let version = &ciphertext[8..11];
    let nonce: &XNonce = XNonce::from_slice(&ciphertext[11..35]);
    let salt = &ciphertext[35..51];
    let key = getkey(salt);
    let cipher = match XChaCha20Poly1305::new_from_slice(key.as_slice()) {
        Ok(c) => c,
        Err(_) => {
            drop(key);
            panic!("Creating Cipher Failed.");
        }
    };
    let mut aad = Vec::new();
    aad.extend_from_slice(&MAGIC_BYTES);
    aad.extend_from_slice(nonce.as_slice());
    aad.extend_from_slice(salt);
    let plaintext = match cipher.decrypt(
        nonce,
        Payload {
            msg: &ciphertext[51..],
            aad: &aad[..],
        },
    ) {
        Ok(c) => c,
        Err(_) => return Err(Error::other("Error in decryption")),
    };
    drop(cipher);
    drop(key);
    Ok(plaintext)
}
