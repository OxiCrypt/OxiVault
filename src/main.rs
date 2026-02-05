mod encrypt;
use clap::{arg, command, value_parser};
use shellexpand::full;
use std::{path::PathBuf, process::ExitCode, str::FromStr};
// TODO: Implement actual handling
fn main() -> ExitCode {
    println!("Welcome to OxiVault, the blazing-fast password manager!");
    let matches = command!()
        .arg(
            arg!([file] "OxiVault file to operate on")
                .required(false)
                .value_parser(value_parser!(String)),
        )
        .get_matches();
    let Some(vaultfile) = matches.get_one::<String>("file") else {
        return ExitCode::FAILURE;
    };
    let vaultfile = &match full(vaultfile) {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Failure: Failed to expand environment variables.");
            return ExitCode::FAILURE;
        }
    };
    let vaultfile = match PathBuf::from_str(vaultfile) {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Failed to parse input to path.");
            return ExitCode::FAILURE;
        }
    };
    if !vaultfile.exists() {
        eprintln!("Error: Vault does not exist!");
        return ExitCode::FAILURE;
    }
    println!("Opening vault {}", vaultfile.display());
    println!("Sorry, OxiVault is unfinished");
    ExitCode::SUCCESS
}
