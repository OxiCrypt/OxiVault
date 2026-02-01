use std::{
    fs::File,
    io::{Error, Seek, SeekFrom, Write},
};
mod passwd;
use chacha20poly1305::{
    XChaCha20Poly1305, XNonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use passwd::getkey;
use rand::Rng;

const MAGIC_BYTES: [u8; 8] = [111, 120, 105, 118, 97, 117, 108, 116];
pub fn encrypt_file(plaintext: &mut Vec<u8>, file: &mut File) -> Result<(), Error> {
    let mut rng = rand::rng();
    let salt: [u8; 16] = rng.random();
    let key = getkey(&salt);
    let cipher = match XChaCha20Poly1305::new_from_slice(&key.0) {
        Ok(c) => c,
        Err(_) => {
            drop(key);
            panic!("Creating Cipher Failed.");
        }
    };
    let nonce = XChaCha20Poly1305::generate_nonce(OsRng);
    let ciphertext = match cipher.encrypt(&nonce, &plaintext[..]) {
        Ok(c) => c,
        Err(_) => return Err(Error::new(std::io::ErrorKind::Other, "Error in encryption")),
    };
    drop(cipher);
    drop(key);
    file.seek(SeekFrom::Start(0))?;
    file.write_all(&MAGIC_BYTES)?;
    file.seek(SeekFrom::Current(8))?;
    file.write_all(nonce.as_slice())?;
    file.seek(SeekFrom::Current(24))?;
    file.write_all(&salt)?;
    file.seek(SeekFrom::Current(16))?;
    file.write_all(&ciphertext[..])?;
    Ok(())
}
pub fn decrypt_file(ciphertext: &mut Vec<u8>, file: &mut File) -> Result<Vec<u8>, Error> {
    if !ciphertext.starts_with(&MAGIC_BYTES) {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Error: Magic Bytes do not match. Are you sure this is a Oxivault file?",
        ));
    }
    let nonce: &XNonce = XNonce::from_slice(&ciphertext[8..32]);
    let salt = &ciphertext[32..48];
    let key = getkey(salt);
    let cipher = match XChaCha20Poly1305::new_from_slice(&key.0) {
        Ok(c) => c,
        Err(_) => {
            drop(key);
            panic!("Creating Cipher Failed.");
        }
    };
    let plaintext = match cipher.decrypt(nonce, &ciphertext[48..]) {
        Ok(c) => c,
        Err(_) => return Err(Error::new(std::io::ErrorKind::Other, "Error in decryption")),
    };
    drop(cipher);
    drop(key);
    file.seek(SeekFrom::Start(0))?;
    file.write_all(&ciphertext[..])?;
    file.seek(SeekFrom::End(0))?;
    file.write_all(nonce.as_slice())?;
    Ok(plaintext)
}
