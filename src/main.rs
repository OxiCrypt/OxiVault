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
    output: Option<String>,
}
fn main() -> ExitCode {
    println!("Welcome to OxiVault, the blazing-fast file encryptor!");
    let args = Oxivault::parse();
    let inpath = args.file;
    let outpath_raw = args.output;
    let inpath = if let Ok(p) = full(&inpath) {
        PathBuf::from(p.as_ref())
    } else {
        eprintln!("Failure: Failed to expand environment variables.");
        return ExitCode::FAILURE;
    };
    if !inpath.exists() {
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
    let mut outpath = if let Some(ref o) = outpath_raw {
        if let Ok(p) = full(o) {
            PathBuf::from(p.as_ref())
        } else {
            eprintln!("Failure: Failed to expand environment variables.");
            return ExitCode::FAILURE;
        }
    } else if ecdc == 'e' {
        inpath.with_added_extension("oxv")
    } else if let Some(ext) = inpath.extension()
        && ext == "oxv"
    {
        let mut temp = inpath.clone();
        temp.set_extension("");
        temp
    } else {
        eprintln!("Could not autodetect output path.");
        eprintln!("Try again with an argument for <output>");
        return ExitCode::FAILURE;
    };

    if ecdc == 'e' {
        if let Err(e) = ecdcwrap(&inpath, &mut outpath, true) {
            return e;
        }
        println!("OxiVault encrypted file saved to {}", outpath.display());
    } else if let Err(e) = ecdcwrap(&inpath, &mut outpath, false) {
        return e;
    }

    ExitCode::SUCCESS
}
fn ecdcwrap(pathin: &PathBuf, pathout: &mut PathBuf, encrypt: bool) -> Result<(), ExitCode> {
    checkexists(pathout.as_path())?;
    let Ok(mut infile) = File::open(pathin) else {
        eprintln!("Error opening input file");
        return Err(ExitCode::FAILURE);
    };
    let Ok(mut outfile) = File::create(pathout) else {
        eprintln!("Error creating output file");
        return Err(ExitCode::FAILURE);
    };
    if encrypt {
        if encrypt::encrypt_file(&mut infile, &mut outfile).is_err() {
            eprintln!("Error during Encryption. Exiting program.");
        }
    } else if encrypt::decrypt_file(&mut infile, &mut outfile).is_err() {
        eprintln!("Error during Decryption. Exiting program.");
        return Err(ExitCode::FAILURE);
    }
    Ok(())
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
