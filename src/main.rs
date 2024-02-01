use std::{process, env};
use minigrep::{Config, run};

#[allow(dead_code)]
fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {err}");
        process::exit(1);
    });

    let result = run(config);
    if let Err(e) = result {
        println!("Application error occured {}", e);
        process::exit(1);
    } else {
        println!("The file contents: \n {}", result.unwrap());
    }

}

