mod timer;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("No command specified");
        process::exit(1);
    }

    let command = &args[1];
    let options: Vec<String> = args[2..args.len()].to_vec();

    match command.as_str() {
        "timer" => timer::timer(options),
        _ => {
            println!("Unsupported command '{}'", command);
            process::exit(1);
        }
    }
}
