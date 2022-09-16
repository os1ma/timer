use std::process::Stdio;
use std::{env, process};

use tokio::process::Command;
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, LinesCodec};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("No command specified");
        process::exit(1);
    }

    let command = &args[1];
    let args = args[2..args.len()].to_vec();

    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run command");

    let stdout = child.stdout.take().unwrap();

    let mut reader = FramedRead::new(stdout, LinesCodec::new());

    while let Some(line) = reader.next().await {
        println!("{}", line.expect("Error decoding"));
    }
}
