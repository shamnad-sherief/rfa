use crate::models::Account;
use crate::totp::generate_totp;
use anyhow::{Context, Error, Ok};
use clap::{Parser, Subcommand};
use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

mod models;
mod totp;

#[derive(Parser)]
#[command(
    name = "rfa",
    version,
    about = "A fast, terminal-based TOTP authenticator"
)]
struct Cli {
    /// totp generate (eg: 'rfa github')
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Command>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let command = match (cli.name, cli.command) {
        (Some(name), None) => Some(Command::Generate { name }),
        (_, cmd) => cmd,
    };

    match command {
        Some(Command::Add {
            name,
            secret,
            account_id,
            digits,
        }) => {
            if account_exists(&name)? {
                anyhow::bail!("Account {} already exist", &name);
            }
            let account = Account::new(name, secret);
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .read(true)
                .open(get_file_path())
                .context("Failed to open the db")?;

            // check the cursor is on new line
            if file.metadata()?.len() > 0 {
                file.seek(SeekFrom::End(-1))?;

                let mut last_byte = [0; 1];
                file.read_exact(&mut last_byte)?;
                if last_byte[0] != b'\n' {
                    file.write_all(b"\n")?;
                }
            }

            let mut csv_writer = csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(&file);
            csv_writer
                .serialize(account)
                .context("Couldnt add account to db")?;
            csv_writer.flush()?;
            print!("Added account successfully");
            Ok(())
        }
        Some(Command::Generate { name }) => {
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
                    print!("{}", otp);
                    return Ok(());
                }
            }

            print!("Account not found");
            Ok(())
        }
        Some(Command::List) | None => {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(get_file_path())
                .context("Couldnt open db")?;
            let mut csv_reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(&file);
            let mut iter = csv_reader.deserialize().peekable();
            if iter.peek().is_none() {
                print!(
                    "Your vault is empty! Get started by adding your first account: `rfa add <name> <secret>`"
                );
                return Ok(());
            }
            while let Some(result) = iter.next() {
                let account: Account = result.context("Failed to parse from DB")?;
                let timesec = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("System time went backwards")
                    .as_secs() as u64;
                let otp = generate_totp(&account.secret, timesec, Some(account.period))?;
                if iter.peek().is_some() {
                    println!("{} {}", account.service, otp);
                } else {
                    print!("{} {}", account.service, otp);
                }
            }

            Ok(())
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

#[derive(Subcommand)]
enum Command {
    /// Add a new 2FA account
    Add {
        /// Name of the service (e.g. github)
        name: String,
        /// Base32 secret key
        secret: String,
        /// Optional account identifier
        #[arg(short, long)]
        account_id: Option<String>,
        /// Number of digits (default: 6)
        #[arg(short, long, default_value_t = 6)]
        digits: u8,
    },

    /// Generate TOTP code for an account
    Generate { name: String },

    /// List all saved accounts and codes
    List,
}
