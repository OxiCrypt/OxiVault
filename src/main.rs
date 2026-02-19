#![allow(dead_code)]
#![warn(clippy::pedantic)]
mod encrypt;
use clap::Parser;
use shellexpand::full;
use std::{fs::File, io, path::PathBuf, process::ExitCode, str::FromStr};
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
    let mut errcount = 0;
    let infile = if let Ok(p) = full(&infile) {
        PathBuf::from_str(p.as_ref())
    } else {
        eprintln!("Failure: Failed to expand environment variables.");
        return ExitCode::FAILURE;
    }
    .unwrap();
    if !infile.exists() {
        eprintln!("Error: File does not exist!");
        return ExitCode::FAILURE;
    }
    println!("Would you like to encrypt or decrypt today?");
    let stdin = io::stdin();
    let ecdc = loop {
        let mut ecdc = String::new();
        let Ok(_) = stdin.read_line(&mut ecdc) else {
            if errcount >= 3 {
                eprintln!("Stdin borked. Try stty sane.");
                return ExitCode::FAILURE;
            }
            eprintln!("Error gathering input, please try again.");
            eprintln!("If the issue persists, this program will exit.");
            errcount += 1;
            continue;
        };

        ecdc = ecdc.to_lowercase();
        if ecdc.starts_with('e') || ecdc.starts_with('d') {
            break ecdc.chars().next();
        }
        println!("Please enter a valid mode.");
    }.expect("This error is impossible. If you look at the code, you can see it only breaks whena a first char exists.");
    errcount = 0;
    if ecdc == 'e' {
        let outfile = infile.with_added_extension(".oxv");
        if outfile.exists() {
            println!(
                "{} already exists. Would you like to overwrite and continue?(y/n)",
                outfile.display()
            );
            let mut choice: String = String::new();
            loop {
                let _ = if stdin.read_line(&mut choice).is_ok() {
                    break;
                } else {
                    if errcount >= 3 {
                        eprintln!("Stdin borked. Try stty sane.");
                        return ExitCode::FAILURE;
                    }
                    eprintln!("Error gathering input. Try again.");
                    eprintln!("If issue persists, this program will exit.");
                    continue;
                };
            }
            choice = choice.to_lowercase();
            if !choice.starts_with('y') {
                println!("Exiting on your choice.");
                return ExitCode::SUCCESS;
            }
        }
        errcount = 0;
        {
            let mut outfile = loop {
                if let Ok(file) = File::create(&outfile) {
                    break file;
                }
                if errcount >= 3 {
                    eprintln!("Error creating file.");
                    return ExitCode::FAILURE;
                }
                errcount += 1;
                eprintln!("Error creating output file. Retrying({errcount}/3)...");
            };
            let mut infile = loop {
                if let Ok(file) = File::open(&infile) {
                    break file;
                }
                if errcount >= 3 {
                    eprintln!("Error reading file.");
                    return ExitCode::FAILURE;
                }
                errcount += 1;
                eprintln!("Error reading input file. Retrying({errcount}/3)...");
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
