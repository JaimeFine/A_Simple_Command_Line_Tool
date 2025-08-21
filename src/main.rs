use std::env;
use std::process;
use minigrep::Command;

fn main() {
    let config = Command::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application err: {e}");
        process::exit(1);
    }
}
