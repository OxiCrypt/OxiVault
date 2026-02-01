mod encrypt;
use clap::{arg, command, value_parser};
use encrypt::encrypt_file;
use std::{path::PathBuf, process::ExitCode};
fn main() -> ExitCode {
    println!("Welcome to OxiVault, the blazing-fast password manager!");
    let matches = command!()
        .arg(
            arg!([file] "OxiVault file to operate on")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();
    let Some(vaultfile) = matches.get_one::<PathBuf>("file") else {
        return ExitCode::FAILURE;
    };
    if !vaultfile.exists() {
        return ExitCode::FAILURE;
    }
    println!("Opening vault {}", vaultfile.display());
    println!("Sorry, OxiVault is unfinished");
    return ExitCode::SUCCESS;
}
