#![allow(dead_code)]
mod encrypt;
use clap::Parser;
use shellexpand::full;
use std::{path::PathBuf, process::ExitCode, str::FromStr};
// TODO: Implement actual handling
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Oxivault {
    file: String,
}
fn main() -> ExitCode {
    println!("Welcome to OxiVault, the blazing-fast password manager!");
    let args = Oxivault::parse();
    let vaultfile = args.file;
    let vaultfile = &match full(&vaultfile) {
        Ok(p) => PathBuf::from_str(p.as_ref()),
        Err(_) => {
            eprintln!("Failure: Failed to expand environment variables.");
            return ExitCode::FAILURE;
        }
    }
    .unwrap();
    if !vaultfile.exists() {
        eprintln!("Error: Vault does not exist!");
        return ExitCode::FAILURE;
    }
    println!("Opening vault {}", vaultfile.display());
    println!("Sorry, OxiVault is unfinished");
    ExitCode::SUCCESS
}
