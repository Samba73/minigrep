use std::{env, fs, process};
fn main() {

    let args: Vec<String> = env::args().collect();

    // for arg in args{
    //     println!("{arg}");
    // }

    // let query_string = &args[1];
    // let file_path    = &args[2];

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {err}");
        process::exit(1);
    });

    // println!("The search string is {} in the file path {}", query_string, file_path);

    let file_contents = fs::read_to_string(config.file_path).expect("Unable to read the file and contents");

    println!("The file content is {}", file_contents);

}

struct Config<'a> {
    query_string: &'a str,
    file_path: &'a str,
}

impl Config <'_>{
    // fn new(args: &[String]) -> Config {

    //     if args.len() < 3 {
    //         return Err("Insufficient arguments provided")
    //     }

    //     let query_string = &args[1];
    //     let file_path    = &args[2];

    //     Config { query_string, file_path}

    // }
    fn build(args: &[String]) -> Result<Config, &str>{

        if args.len() < 3 {
            return Err("Insufficient arguments provided")
        }

        let query_string = &args[1];
        let file_path    = &args[2];

        Ok(Config { query_string, file_path})   

    }
}

// fn parse_config(args: &[String]) -> Config {

//     let query_string = &args[1];
//     let file_path    = &args[2];

//     Config { query_string, file_path}
// }
