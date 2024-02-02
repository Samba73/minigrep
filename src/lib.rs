use std::{fs, error::{Error}};

// struct used in main.rs that have config fields grouped 
// has method implemented that build the config from the command line argument
#[derive(Debug)]
pub struct Config<'a> {
    query_string: &'a str,
    file_path: &'a str,
}

impl Config <'_>{
    pub fn build(args: &[String]) -> Result<Config, &str>{

        if args.len() < 3 {
            return Err("Insufficient arguments provided")
        }

        let query_string = &args[1];
        let file_path    = &args[2];

        Ok(Config { query_string, file_path})   

    }
}

// public function called in main.rs to print the content of file
pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>>{

    // let mut result = Vec::new();

    let file_contents = fs::read_to_string(config.file_path)?;

    let result = search(&config.query_string, &file_contents);

    Ok(result)
}

pub fn search(query_string: &str, contents: &str) -> Vec<String> {

    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query_string) {
            result.push(line.trim().to_string());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query_string = "duct";
        let contents = "\
    Rust: 
    Safe, Fast, and Productive.
    Pick three.";

        assert_eq!(vec!["Safe, Fast, and Productive."], search(query_string, contents));
    }
}
