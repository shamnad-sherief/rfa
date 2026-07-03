use std::env;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use crate::totp::generate_totp;

mod totp;

fn main() {
    let cmd: Vec<String> = env::args().collect();

    match parse_command(cmd) {
        Ok(cmd) => match cmd {
            Command::Add(account) => {
                if account_exists(&account.name) {
                    eprintln!("Account already exists");
                    return;
                }
                let file = OpenOptions::new()
                    .append(true)
                    .write(true)
                    .create(true)
                    .open(get_file_path());
                if let Ok(mut file) = file {
                    if let Ok(_) = writeln!(file, "{} {}", account.name, account.secret) {
                        println!("Added account successfully");
                    } else {
                        eprintln!("Couldnt add account");
                    }
                } else {
                    eprintln!("Couldnt open file");
                }
            }
            Command::Generate { name } => {
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
                            let mut parts = line.split_whitespace();
                            if let Some(first_word) = parts.next() {
                                if first_word.eq_ignore_ascii_case(&name) {
                                    if let Some(secret) = parts.next() {
                                        let timesec = std::time::SystemTime::now()
                                            .duration_since(std::time::UNIX_EPOCH)
                                            .expect("System time went backwards")
                                            .as_secs()
                                            as u64;
                                        let otp = generate_totp(secret, timesec);
                                        match otp {
                                            Ok(otp) => println!("{}", otp),
                                            Err(_) => println!("Failed to generate TOTP"),
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => eprintln!("Couldnt open file"),
                }
            }
            Command::List => {
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
                                println!("{}", first_word);
                            }
                        }
                    }
                    Err(_) => eprintln!("Couldnt open file"),
                }
            }
        },
        Err(_) => println!("Unknown command"),
    }
}

fn parse_command(args: Vec<String>) -> Result<Command, String> {
    let cmd = args.get(1);
    match cmd {
        None => Err("Provide at least one argument".into()),
        Some(cmd) => {
            if cmd.eq_ignore_ascii_case("list") {
                Ok(Command::List)
            } else if cmd.eq_ignore_ascii_case("add") {
                if args.len() < 4 {
                    return Err("add requires <name> <secret>".into());
                }
                let name = args[2].to_string();
                let secret = args[3].to_string();
                Ok(Command::Add(Account { name, secret }))
            } else if cmd.eq_ignore_ascii_case("generate") {
                if args.len() < 3 {
                    return Err("generate requires <name>".into());
                }
                let name = args[2].to_string();
                Ok(Command::Generate { name })
            } else {
                Err("Unknown command".into())
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
    let home = env::home_dir();
    if let Some(source) = home {
        let home = Path::new(&source);
        home.join(".rfa")
    } else {
        Path::new(".rfa").to_path_buf()
    }
}

struct Account {
    name: String,
    secret: String,
}

enum Command {
    Add(Account),
    Generate { name: String },
    List,
}
