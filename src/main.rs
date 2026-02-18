#![allow(dead_code)]
#![warn(clippy::pedantic)]
mod encrypt;
use clap::Parser;
use shellexpand::full;
use std::{io, path::PathBuf, process::ExitCode, str::FromStr};
// TODO: Implement actual handling
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Oxivault {
    file: String,
}
fn main() -> ExitCode {
    println!("Welcome to OxiVault, the blazing-fast file encryptor!");
    let args = Oxivault::parse();
    let vaultfile = args.file;
    let mut errcount = 0;
    let vaultfile = &if let Ok(p) = full(&vaultfile) {
        PathBuf::from_str(p.as_ref())
    } else {
        eprintln!("Failure: Failed to expand environment variables.");
        return ExitCode::FAILURE;
    }
    .unwrap();
    if !vaultfile.exists() {
        eprintln!("Error: File does not exist!");
        return ExitCode::FAILURE;
    }
    println!("Would you like to encrypt or decrypt today?");
    let stdin = io::stdin();
    let ecdc: u8 = loop {
        let mut ecdc = String::new();
        let Ok(_) = stdin.read_line(&mut ecdc) else {
            if errcount >= 3 {
                panic!("Stdin borked");
            } else {
                eprintln!("Error gathering input, please try again.");
                eprintln!(
                    "If the issue persists, this program will exit.\nTry stty sane from there."
                );
                errcount += 1;
                continue;
            }
        };

        ecdc = ecdc.to_lowercase();
        if ecdc.starts_with('e') {
            break 1;
        } else if ecdc.starts_with('d') {
            break 0;
        }
        println!("Please enter a valid mode.");
    };
    ExitCode::SUCCESS
}
