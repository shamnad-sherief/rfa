use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();

    let cmd : Result<Command, String> = parse_command(&args);

    match cmd {
        // TODO
        Ok(_) => {}
        Err(_) => {}
    }
}

fn parse_command(args : &[String]) -> Result<Command, String> {
    let cmd = args.get(1);
    match cmd {
        None => {
            Err("Provide at least one argument".into())
        }
        Some(cmd) => {
            if cmd.eq_ignore_ascii_case("list"){
                Ok(Command::List)
            } else if  cmd.eq_ignore_ascii_case("add") {
                if args.len() < 4{
                    return Err("add requires <name> <secret>".into());
                }
                let name = args[2].to_string();
                let secret = args[3].to_string();
                Ok(Command::Add{name, secret})
            } else if  cmd.eq_ignore_ascii_case("generate") {
                if args.len() < 3{
                    return Err("generate requires <name>".into());
                }
                let name = args[2].to_string();
                Ok(Command::Generate{name})
            }
            else {
                Err("Unknown command".into())
            }
        }
    }
}

struct Account {
    name : String,
    secret : String
}

enum Command{
    Add{ name : String , secret : String},
    Generate {name: String},
    List
}