use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let mut cmd = String::new();

    io::stdin()
        .read_line(&mut cmd)
        .expect("Couldnt read command");

    match parse_command(
        &cmd.trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    ) {
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
                                println!("{}", secret);
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

fn parse_command(args: &[String]) -> Result<Command, String> {
    let cmd = args.get(0);
    match cmd {
        None => Err("Provide at least one argument".into()),
        Some(cmd) => {
            if cmd.eq_ignore_ascii_case("list") {
                Ok(Command::List)
            } else if cmd.eq_ignore_ascii_case("add") {
                if args.len() < 3 {
                    return Err("add requires <name> <secret>".into());
                }
                let name = args[1].to_string();
                let secret = args[2].to_string();
                Ok(Command::Add { name, secret })
            } else if cmd.eq_ignore_ascii_case("generate") {
                if args.len() < 2 {
                    return Err("generate requires <name>".into());
                }
                let name = args[1].to_string();
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
