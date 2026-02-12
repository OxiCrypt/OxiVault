use argon2::Algorithm;
use argon2::Argon2;
use argon2::Params;
use argon2::Version;
use rpassword::prompt_password;
use zeroize::Zeroize;

pub struct Key([u8; 32]);
impl Key {
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.0
    }
    pub fn from_slice(slice: &[u8; 32]) -> Self {
        Key(*slice)
    }
}
impl Zeroize for Key {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}
impl Drop for Key {
    fn drop(&mut self) {
        self.zeroize();
    }
}
pub fn getkey(salt: &[u8], params: Params) -> Key {
    let Ok(mut pass) = prompt_password("Enter your password.") else {
        panic!("Error prompting password. Please try again.")
    };
    derivekey(salt, params, &mut pass)
}
fn derivekey(salt: &[u8], params: Params, pass: &mut String) -> Key {
    let mut passbytes = (*pass).as_bytes().to_owned();
    (*pass).zeroize();
    let mut outkey = Key::from_slice(&[0u8; 32]);
    let hasher = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let Ok(()) = hasher.hash_password_into(&passbytes, salt, outkey.as_mut_slice()) else {
        passbytes.zeroize();
        panic!("Error in KDF")
    };
    passbytes.zeroize();
    outkey
}
