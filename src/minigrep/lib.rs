use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool
}

// From what I understand, specifying `pub` is not necessary here, because `Config` is already public.
impl Config {
    fn build(args: &[String]) -> Result<Config, 'static str> {
        if args.len() != 3 {
            return Err("Config needs exactly 2 arguments (query, file_path)!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE")
            // `is_ok()` checks if the Result is Ok and returns a bool.
            .is_ok();
        
        Ok(Config {
            query,
            file_path, 
            case_sensitive
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    }

    for (line in results) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<'a, str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<'a, str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    result
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

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}