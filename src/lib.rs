use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Config<'a> {
    query: &'a str,
    filename: &'a str,
    case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let mut case_sensitive = match env::var("CASE_SENSITIVE") {
            Ok(val) => {
                if val.parse::<i32>().unwrap() == 0 {
                    false
                } else {
                    true
                }
            }
            // Default
            Err(_) => true,
        };

        if args.len() < 3 {
            Err("Not enough arguments")
        } else {
            if args.len() > 3 && args[1] == "--case-insensitive" {
                case_sensitive = false;
            }
            Ok(Config {
                query: &args[args.len() - 2],
                filename: &args[args.len() - 1],
                case_sensitive,
            })
        }
    }

    pub fn run(&self) -> Result<(), Box<Error>> {
        let mut file = File::open(self.filename)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        search(self.query, &content, self.case_sensitive)
            .iter()
            .for_each(|&line| println!("{}", line));

        Ok(())
    }
}

fn search<'a>(query: &str, content: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    content
        .lines()
        .filter(|&line| {
            if case_sensitive {
                line.contains(query)
            } else {
                line.to_lowercase().contains(&query.to_lowercase())
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Duct";
        let content = "Rust:\nsafe, fast, productive.\nPick three.\nPick three.\nDuct tape.";
        assert_eq!(vec!["Duct tape."], search(query, content, true));
    }

    #[test]
    fn case_insensitive() {
        let query = "DuCt";
        let content = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content, false)
        );
    }
}
