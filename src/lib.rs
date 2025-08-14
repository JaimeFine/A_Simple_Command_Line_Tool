use std::error::Error;
use std::fs;
use std::env;

pub struct Command {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}   // This is for the command.

impl Command {
    pub fn build(args: &[String]) -> Result<Command, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Command{query, file_path, ignore_case})
    }
}   // A method for the Config struct.

pub fn run(command: Command) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(command.file_path)?;

    let results = if command.ignore_case{
        search_case_insensitive(&command.query, &contents)
    } else {
        search(&command.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}   // Function to run the commands.

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
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}   // A function for the search of the result of the command

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
    let mut num = 0;

    for line in contents.lines() {
        if line.contains(&query) {
            num += 1;
        }
    }
    if num == 1 {
        println!("There is only {num} lineshat contains {}", &query);
    } else if num == 0 {
        println!("There are no lines that contains {}", &query);
    } else if num > 1 {
        println!("There are {num} lines that contains {}", &query);
    } else {
        eprintln!("Something went wrong!!!");
    }
    
    num
}   // Function to count the number of the line that the keywords exist.