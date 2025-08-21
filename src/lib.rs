use std::error::Error;
use std::fs;
use std::env;

pub struct Command {
    pub order: String,
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Command {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Command, &'static str> {
        args.next();

        let first_arg = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an order"),
        };

        if first_arg == "--help" {
            println!(
                "Usage: <program> <order> <query> <file_path>\n\
                 Orders:\n\
                 - search: search for lines containing the query\n\
                 - countline: count lines containing the query\n\
                 Options:\n\
                 - IGNORE_CASE=1 (set as env var for case-insensitive search)"
            );
            std::process::exit(0);
        }

        let order = first_arg;

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Command{
            order,
            query,
            file_path,
            ignore_case
        })
    }
}

pub fn run(command: Command) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(command.file_path)?;

    if command.ignore_case{
        let results = search_case_insensitive(&command.query, &contents);
        for line in results {
            println!("{line}");
        }
    } else {
        if &command.order == "search" {
            let results = search(&command.query, &contents);
            for line in results {
                println!("{line}");
            }
        } else if &command.order == "countline" {
            let count = count_lines(&command.query, &contents);
            println!("Matched lines: {}", count)
        } else {
            println!("Invalid order!")
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

    #[test]
    fn to_count_lines() {
        let query = "rust";
        let contents = "\
Rust:
rust is safe, fast, productive.
Pick three from rust.
Trust me and rust.";
        assert_eq!(3, count_lines(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}   // A function that works the same as search(), but insensitive to case.

pub fn count_lines(query: &str, contents: &str) -> i32 {
    let num = contents
        .lines()
        .filter(|line| line.contains(query))
        .count() as i32;

    match num {
        0 => println!("There are no lines that contains {}", &query),
        1 => println!("There is only {num} lines that contains {}", &query),
        n if n > 1 => println!("There are {num} lines that contains {}", &query),
        _ => eprintln!("Something went wrong!!!"),
    }
    
    num

}   // Function to count the number of the line that the keywords exist.