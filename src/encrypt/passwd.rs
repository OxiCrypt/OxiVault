use argon2::Argon2;
use rpassword::prompt_password;
use zeroize::Zeroize;
pub struct Key(pub [u8; 32]);
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
    let mut outkey = Key([0u8; 32]);
    match Argon2::default().hash_password_into(&passbytes, salt, &mut outkey.0) {
        Ok(_) => (),
        Err(_) => {
            passbytes.zeroize();
            panic!("Error in KDF")
        }
    };
    passbytes.zeroize();
    outkey
}
