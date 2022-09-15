mod timer;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let options: Vec<String> = args[2..args.len()].to_vec();

    println!("command = {}", command);
    println!("options = {:?}", options);

    timer::timer();
}
