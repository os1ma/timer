mod timer;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
        println!("No command specified");
        return;
    }

    let command = &args[1];
    let options: Vec<String> = args[2..args.len()].to_vec();

    println!("command = {}", command);
    println!("options = {:?}", options);

    match command.as_str() {
        "timer" => timer::timer(),
        _ => println!("Unsupported command '{}'", command),
    }
}
