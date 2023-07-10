use std::{env, process};
use mgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)
        .expect("Should have been able to read the file");

    if let Err(e) = mgrep::run(config){
        println!("Application error: {e}");
        process::exit(1);
    }
}
