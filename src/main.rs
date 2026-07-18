use crate::models::Account;
use crate::totp::generate_totp;
use anyhow::{Context, Error};
use std::env;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

mod models;
mod totp;

fn main() -> anyhow::Result<()> {
    let cmd: Vec<String> = env::args().collect();

    match parse_command(cmd) {
        Ok(cmd) => match cmd {
            Command::Add(account) => {
                if account_exists(&account.service)? {
                    anyhow::bail!("Account {} already exist", &account.service);
                }
                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(get_file_path())
                    .context("Failed to open the db")?;

                let mut csv_writer = csv::WriterBuilder::new()
                    .has_headers(false)
                    .from_writer(&file);
                csv_writer
                    .serialize(account)
                    .context("Couldnt add account to db")?;
                csv_writer.flush()?;
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
                let mut csv_reader = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(&file);
                for result in csv_reader.deserialize() {
                    let account: Account = result.context("Failed to parse from DB")?;
                    if account.service.eq_ignore_ascii_case(&name) {
                        let timesec = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("System time went backwards")
                            .as_secs() as u64;
                        let otp = generate_totp(&account.secret, timesec, Some(account.period))?;
                        println!("{}", otp);
                        return Ok(());
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
                // TODO: if the file is empty then show "No account found"
                let mut csv_reader = csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(&file);
                for result in csv_reader.deserialize() {
                    let account: Account = result.context("Failed to parse from DB")?;
                    let timesec = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("System time went backwards")
                        .as_secs() as u64;
                    let otp = generate_totp(&account.secret, timesec, Some(account.period))?;
                    println!("{} {}", account.service, otp);
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

fn account_exists(name: &str) -> Result<bool, Error> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(get_file_path())
        .context("Failed to open the db")?;

    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(&file);
    for result in csv_reader.deserialize() {
        let account: Account = result.context("Failed to parse from DB")?;
        if account.service.eq_ignore_ascii_case(&name) {
            return Ok(true);
        }
    }
    Ok(false)
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
