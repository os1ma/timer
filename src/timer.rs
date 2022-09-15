use std::{
    io::{self, Write},
    process::Command,
};

pub fn timer(options: Vec<String>) {
    if options.len() == 0 {
        return;
    }

    let command = &options[0];
    let args = options[1..options.len()].to_vec();

    let output = Command::new(command).args(args).output();
    match output {
        Ok(o) => {
            io::stdout().write(&o.stdout).expect("Error write stdout");
        }
        Err(e) => println!("{}", e),
    }
}
