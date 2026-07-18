use anyhow::Context;
use std::env;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use crate::models::Account;
use crate::totp::generate_totp;

mod models;
mod totp;

fn main() -> anyhow::Result<()> {
    let cmd: Vec<String> = env::args().collect();

    match parse_command(cmd) {
        Ok(cmd) => match cmd {
            Command::Add(account) => {
                if account_exists(&account.service) {
                    anyhow::bail!("Account {} already exist", &account.service);
                }
                let mut file = OpenOptions::new()
                    .append(true)
                    .write(true)
                    .create(true)
                    .open(get_file_path())
                    .context("Failed to open the db")?;

                writeln!(file, "{} {}", account.service, account.secret)
                    .context("Couldnt add account to db")?;
                println!("Added account successfully");
                Ok(())
            }
            Command::Generate { name } => {
                let file = OpenOptions::new()
                    .read(true)
                    .create(true)
                    .write(true)
                    .open(get_file_path())
                    .context("Failed to open the db")?;

                let reader = BufReader::new(file);
                for line in reader.lines() {
                    let line = match line {
                        Ok(l) => l,
                        Err(_) => {
                            eprintln!("Failed to read line");
                            continue;
                        }
                    };
                    let mut parts = line.split_whitespace();
                    if let Some(first_word) = parts.next() {
                        if first_word.eq_ignore_ascii_case(&name) {
                            if let Some(secret) = parts.next() {
                                let timesec = std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .expect("System time went backwards")
                                    .as_secs() as u64;
                                let otp = generate_totp(secret, timesec);
                                match otp {
                                    Ok(otp) => println!("{}", otp),
                                    Err(_) => println!("Failed to generate TOTP"),
                                }
                                return Ok(());
                            }
                        }
                    }
                }
                println!("Account not found");
                Ok(())
            }
            Command::List => {
                let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(get_file_path())
                    .context("Couldnt open db")?;

                let reader = BufReader::new(file);
                // TODO: if the file is empty then show "No account found"
                for line in reader.lines() {
                    let line = match line {
                        Ok(l) => l,
                        Err(_) => {
                            eprintln!("Failed to read line");
                            continue;
                        }
                    };
                    if let Some(first_word) = line.split_whitespace().next() {
                        println!("{}", first_word);
                    }
                }

                Ok(())
            }
        },
        Err(e) => anyhow::bail!(e),
    }
}

fn parse_command(args: Vec<String>) -> anyhow::Result<Command> {
    let cmd = args.get(1);
    match cmd {
        None => Ok(Command::List),
        Some(cmd) => {
            if cmd.eq_ignore_ascii_case("list") {
                Ok(Command::List)
            } else if cmd.eq_ignore_ascii_case("add") {
                if args.len() < 4 {
                    anyhow::bail!("add requires <name> <secret>")
                }
                let name = args[2].to_string();
                let secret = args[3].to_string();
                let account = Account::new(name, secret);
                Ok(Command::Add(account))
            } else if cmd.eq_ignore_ascii_case("generate") {
                if args.len() < 3 {
                    anyhow::bail!("generate requires <name>")
                }
                let name = args[2].to_string();
                Ok(Command::Generate { name })
            } else if args.len() == 2 {
                let name = args[1].to_string();
                Ok(Command::Generate { name })
            } else {
                anyhow::bail!("Invalid command")
            }
        }
    }
}

fn account_exists(name: &str) -> bool {
    let file = OpenOptions::new().read(true).open(get_file_path());
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => {
                        eprintln!("Failed to read line");
                        continue;
                    }
                };
                if let Some(first_word) = line.split_whitespace().next() {
                    if first_word.eq_ignore_ascii_case(name) {
                        return true;
                    }
                }
            }
            return false;
        }
        Err(_) => return false,
    }
}

fn get_file_path() -> PathBuf {
    if cfg!(debug_assertions) {
        return Path::new(".rfa-dev").to_path_buf();
    }
    let home = env::home_dir();
    if let Some(source) = home {
        let home = Path::new(&source);
        home.join(".rfa")
    } else {
        Path::new(".rfa").to_path_buf()
    }
}
enum Command {
    Add(Account),
    Generate { name: String },
    List,
}
