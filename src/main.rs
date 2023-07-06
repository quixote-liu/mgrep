use std::{env};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("the arguments error, please input query and file path.")
    }

    for a in &args {
        println!("the arg = {:?}", a);
    }
}
