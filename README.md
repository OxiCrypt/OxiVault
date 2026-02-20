# OxiVault
OxiVault is a *very* fast file encryptor written in Rust.
It was originally supposed to be a password manager, but because I am **lazy** I decided to pivot to a simpler file encryptor 
Features:
* Uses tried and tested primitives like XChaCha20-Poly1305, Argon2id, and more.
* Very fast, thanks to lean ChaCha20-based encryption.
* Secure, thanks to the underlying primitives and libraries.
* Intuitive, thanks to a CLI built with clap and Rust's native stdin/stdout support.
* Safe, thanks to Rust's memory safety guarantees.
Not for production use.
