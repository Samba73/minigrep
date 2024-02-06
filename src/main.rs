use std::{process, env};
use minigrep::{Config, run};

#[allow(dead_code)]
fn main() {


    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {err}");
        process::exit(1);
    });


    let result = run(config);
    if let Err(e) = result {
        eprintln!("Application error occured {}", e);
        process::exit(1);
    } else {
        for item in result.unwrap() {
            println!("{item}");
        }
    }

}

