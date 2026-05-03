use base32::{Alphabet, decode};
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::env;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let cmd: Vec<String> = env::args().collect();

    match parse_command(cmd) {
        Ok(cmd) => match cmd {
            Command::Add(account) => {
                let file = OpenOptions::new()
                    .append(true)
                    .write(true)
                    .create(true)
                    .open("2fa");
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
                let file = OpenOptions::new().read(true).open("2fa");
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
                                        let dec = match decode(
                                            Alphabet::Rfc4648 { padding: true },
                                            secret,
                                        ) {
                                            Some(v) => v,
                                            None => {
                                                eprintln!("Base 32 decoding failed for {}", name);
                                                continue;
                                            }
                                        };
                                        let current_time = match std::time::SystemTime::now()
                                            .duration_since(std::time::UNIX_EPOCH)
                                        {
                                            Ok(d) => d,
                                            Err(_) => {
                                                eprintln!("Time went backwards");
                                                continue;
                                            }
                                        }
                                        .as_secs()
                                            as u64
                                            / 30;

                                        let mut hasher: Hmac<Sha1> =
                                            match Mac::new_from_slice(dec.as_ref()) {
                                                Ok(h) => h,
                                                Err(_) => {
                                                    eprintln!("HMAC creation failed for {}", name);
                                                    continue;
                                                }
                                            };
                                        hasher.update(current_time.to_be_bytes().as_ref());
                                        let result = hasher.finalize();
                                        let hash = result.into_bytes();

                                        // Step 1: dynamic offset
                                        let offset = (hash[hash.len() - 1] & 0x0f) as usize;

                                        // Step 2: 4-byte slice
                                        let binary = ((hash[offset] as u32 & 0x7f) << 24)
                                            | ((hash[offset + 1] as u32) << 16)
                                            | ((hash[offset + 2] as u32) << 8)
                                            | (hash[offset + 3] as u32);

                                        // Step 3: mod to get OTP
                                        let otp = binary % 1_000_000;
                                        println!("{:06}", otp);
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
                let file = OpenOptions::new().read(true).open("2fa");
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

struct Account {
    name: String,
    secret: String,
}

enum Command {
    Add(Account),
    Generate { name: String },
    List,
}
