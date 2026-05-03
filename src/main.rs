use base32::{Alphabet, decode};
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::env;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let cmd: Vec<String> = env::args().collect();

    match parse_command(cmd) {
        // TODO
        Ok(cmd) => match cmd {
            Command::Add { name, secret } => {
                let mut file = OpenOptions::new()
                    .append(true)
                    .write(true)
                    .create(true)
                    .open("2fa")
                    .expect("Couldnt handle the file");
                match writeln!(file, "{} {}", name, secret) {
                    Ok(_) => {}
                    Err(_) => {
                        println!("Couldnt write to file")
                    }
                }
            }
            Command::Generate { name } => {
                let file = OpenOptions::new()
                    .read(true)
                    .open("2fa")
                    .expect("File not found");
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    let line = line.expect("Failed to read line");
                    if let Some(first_word) = line.split_whitespace().next() {
                        if first_word.eq_ignore_ascii_case(&name) {
                            if let Some(secret) = line.split_whitespace().nth(1) {
                                let dec = match decode(Alphabet::Rfc4648 { padding: true }, secret)
                                {
                                    Some(v) => v,
                                    None => {
                                        eprintln!("Base 32 decoding failed for {}", name);
                                        continue;
                                    }
                                };
                                let current_time = (std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .expect("Time went backwards")
                                    .as_secs()
                                    as u64)
                                    / 30;

                                let mut hasher: Hmac<Sha1> = Mac::new_from_slice(dec.as_ref())
                                    .expect("HMAC algoritms can take keys of any size");
                                hasher.update(current_time.to_be_bytes().as_ref());
                                let result = hasher.finalize();
                                let hash = result.into_bytes();

                                // Step 1: dynamic offset
                                let offset = (hash[19] & 0x0f) as usize;

                                // Step 2: 4-byte slice
                                let binary = ((hash[offset] as u32 & 0x7f) << 24)
                                    | ((hash[offset + 1] as u32) << 16)
                                    | ((hash[offset + 2] as u32) << 8)
                                    | (hash[offset + 3] as u32);

                                // Step 3: mod to get OTP
                                let otp = binary % 1_000_000;
                                println!("{}", otp);
                                break;
                            }
                        }
                    }
                }
            }
            Command::List => {
                let file = OpenOptions::new()
                    .read(true)
                    .open("2fa")
                    .expect("File not found");
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    let line = line.expect("Failed to read line");
                    if let Some(first_word) = line.split_whitespace().next() {
                        println!("{}", first_word);
                    }
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
                if args.len() < 3 {
                    return Err("add requires <name> <secret>".into());
                }
                let name = args[2].to_string();
                let secret = args[3].to_string();
                Ok(Command::Add { name, secret })
            } else if cmd.eq_ignore_ascii_case("generate") {
                if args.len() < 2 {
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
    Add { name: String, secret: String },
    Generate { name: String },
    List,
}
