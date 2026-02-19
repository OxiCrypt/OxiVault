#![allow(dead_code)]
#![warn(clippy::pedantic)]
mod encrypt;
use clap::Parser;
use shellexpand::full;
use std::{
    fs::File,
    io::{self, stdin},
    path::{Path, PathBuf},
    process::ExitCode,
};
// TODO: Implement actual handling
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Oxivault {
    file: String,
}
fn main() -> ExitCode {
    println!("Welcome to OxiVault, the blazing-fast file encryptor!");
    let args = Oxivault::parse();
    let infile = args.file;
    let infile = if let Ok(p) = full(&infile) {
        PathBuf::from(p.as_ref())
    } else {
        eprintln!("Failure: Failed to expand environment variables.");
        return ExitCode::FAILURE;
    };
    if !infile.exists() {
        eprintln!("Error: File does not exist!");
        return ExitCode::FAILURE;
    }
    println!("Would you like to encrypt or decrypt today?");
    let stdin = io::stdin();
    let ecdc = loop {
        let mut ecdc = String::new();
        if stdin.read_line(&mut ecdc).is_err() {
            eprintln!("Stdin borked. Try stty sane.");
            return ExitCode::FAILURE;
        }

        ecdc = ecdc.to_lowercase();
        if ecdc.starts_with('e') || ecdc.starts_with('d') {
            break ecdc.chars().next();
        }
        println!("Please enter a valid mode.");
    }.unwrap()
    /*
     * This error is impossible.
    If you look at the code,
    you can see it only breaks whena a first char exists.
    Hence, unwrap
    */;
    if ecdc == 'e' {
        let outfile = infile.with_added_extension(".oxv");
        if let Err(e) = checkexists(outfile.as_path()) {
            return e;
        }
        {
            let Ok(mut outfile) = File::create(&outfile) else {
                eprintln!("Error creating file.");
                return ExitCode::FAILURE;
            };
            let Ok(mut infile) = File::open(&infile) else {
                eprintln!("Error reading file.");
                return ExitCode::FAILURE;
            };
            if encrypt::encrypt_file(&mut infile, &mut outfile).is_err() {
                eprintln!("Error during Encryption. Exiting program.");
                return ExitCode::FAILURE;
            }
        }
        println!("OxiVault encrypted file saved to {}", outfile.display());
    } else {
        todo!("Decryption Process")
    }
    ExitCode::SUCCESS
}
fn checkexists(file: &Path) -> Result<(), ExitCode> {
    if file.exists() {
        println!(
            "{} already exists. Would you like to overwrite and continue?(y/n)",
            file.display()
        );
        let mut choice: String = String::new();
        if stdin().read_line(&mut choice).is_err() {
            eprintln!("Stdin borked. Try stty sane.");
            return Err(ExitCode::FAILURE);
        }
        choice = choice.to_lowercase();
        if !choice.starts_with('y') {
            println!("Exiting on your choice.");
            return Err(ExitCode::SUCCESS);
        }
    }
    Ok(())
}
