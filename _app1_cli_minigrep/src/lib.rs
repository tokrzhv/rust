use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;//.expect("Something went wrong reading the file");
    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result{
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // if args.len() <3 { return Err("not enough arguments") } // let query = args[1].clone(); // let filename = args[2].clone();

        args.next(); //args[0] -> drop in nothing _ just skip

        let query = match args.next() {  //query is now taking an ownership of the string
            Some(arg) => arg,
            None => return Err("Didn't get a query"),
        };
        let filename = match args.next() {
           Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // is_err for return bool

        Ok( Config { query, filename, case_sensitive} )
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
    //using loops or iterators is about the same in terms of speed
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query){
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "safe, fast, productive.";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.";
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
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}