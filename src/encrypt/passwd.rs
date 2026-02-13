use argon2::Algorithm;
use argon2::Argon2;
use argon2::Params;
use argon2::Version;
use rpassword::prompt_password;
use std::cmp::Eq;
use std::cmp::PartialEq;
use zeroize::Zeroize;
#[derive(Debug)]
pub struct Key([u8; 32]);
impl Eq for Key {}
impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
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
#[cfg(test)]
mod tests {
    use super::*;
    use argon2::Params;
    // Yes the parameters are comically weak but its just for tests
    #[test]
    fn salt_works() {
        let par = Params::new(1024, 3, 1, Some(32)).expect("Uh guys");
        assert_ne!(
            derivekey(&[0u8; 16], par.clone(), &mut "aaaa".to_string()),
            derivekey(&[1u8; 16], par, &mut "aaaa".to_string())
        );
    }
    #[test]
    fn param_works() {
        let par1 = Params::new(1024, 3, 1, Some(32)).expect("Uh guys");
        let par2 = Params::new(1024, 3, 2, Some(32)).expect("Uh guys");
        let par3 = Params::new(1024, 2, 1, Some(32)).expect("Uh guys");
        let par4 = Params::new(1023, 3, 1, Some(32)).expect("Uh guys");
        assert_ne!(
            derivekey(&[0u8; 16], par1.clone(), &mut "aaah".to_string()),
            derivekey(&[0u8; 16], par2, &mut "aaah".to_string())
        );
        assert_ne!(
            derivekey(&[0u8; 16], par1.clone(), &mut "aaah".to_string()),
            derivekey(&[0u8; 16], par3, &mut "aaah".to_string())
        );
        assert_ne!(
            derivekey(&[0u8; 16], par1, &mut "aaah".to_string()),
            derivekey(&[0u8; 16], par4, &mut "aaah".to_string())
        );
    }
    #[test]
    fn pass_works() {
        let par = Params::new(1024, 3, 1, Some(32)).expect("Uh guys");
        assert_ne!(
            derivekey(&[0u8; 16], par.clone(), &mut "aaaa".to_string()),
            derivekey(&[0u8; 16], par, &mut "aaah".to_string())
        );
    }
}
