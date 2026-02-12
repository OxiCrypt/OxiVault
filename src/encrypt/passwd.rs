use argon2::Argon2;
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
pub fn getkey(salt: &[u8]) -> Key {
    let mut pass = match prompt_password("Enter your password.") {
        Ok(n) => n,
        Err(_) => {
            panic!("Error prompting password. Please try again.")
        }
    };
    let mut passbytes = pass.as_bytes().to_owned();
    pass.zeroize();
    let mut outkey = Key::from_slice(&[0u8; 32]);
    match Argon2::default().hash_password_into(&passbytes, salt, outkey.as_mut_slice()) {
        Ok(_) => (),
        Err(_) => {
            passbytes.zeroize();
            panic!("Error in KDF")
        }
    };
    passbytes.zeroize();
    outkey
}
