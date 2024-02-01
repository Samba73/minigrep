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
pub fn run(config: Config) -> Result<String, Box<dyn Error>>{

    let file_contents = fs::read_to_string(config.file_path)?;

    // println!("The file content is {}", file_contents);

    Ok(file_contents)

}