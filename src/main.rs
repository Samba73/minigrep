use std::{env, fs};
fn main() {

    let args: Vec<String> = env::args().collect();

    // for arg in args{
    //     println!("{arg}");
    // }

    // let query_string = &args[1];
    // let file_path    = &args[2];

    let config = parse_config(&args);

    // println!("The search string is {} in the file path {}", query_string, file_path);

    let file_contents = fs::read_to_string(config.file_path).expect("Unable to read the file and contents");

    println!("The file content is {}", file_contents);

}

struct Config<'a> {
    query_string: &'a str,
    file_path: &'a str,
}

fn parse_config(args: &[String]) -> Config {

    let query_string = &args[1];
    let file_path    = &args[2];

    Config { query_string, file_path}
}
