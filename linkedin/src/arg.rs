use std::env;
use std::env::args;
use std::prelude::*;
use std::process::exit;

pub fn get_args() -> String {
    for (index, item) in env::args().enumerate() {
        println!("ARGUMENTS: {} => {}", index, item);
    }

    if env::args().len() <= 1 {
        println!(" provide file name");
        exit(1);
    }

    let arg2 = env::args().nth(0).unwrap();
    println!("Arg2 = {}", arg2);
    let arg2 = env::args().nth(1).unwrap();
    arg2
}
