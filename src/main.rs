use std::{env, process};
use std::{
    io::{self, Write},
    process::Command,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("No command specified");
        process::exit(1);
    }

    let command = &args[1];
    let args = args[2..args.len()].to_vec();

    let output = Command::new(command).args(args).output();
    match output {
        Ok(o) => {
            io::stdout().write(&o.stdout).expect("Error write stdout");
        }
        Err(e) => eprintln!("Command execution failed. {}", e),
    }
}
