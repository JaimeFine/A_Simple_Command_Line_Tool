use std::env;
use std::process;
use minigrep::Command;

fn main() {
    let args: Vec<String> = env::args().collect();  // Reading the arguments.

    let config = Command::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    }); // Building the configuration arguments.

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application err: {e}");
        process::exit(1);
    }
}
