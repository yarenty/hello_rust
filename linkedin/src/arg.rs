use std::env;
use std::env::args;
use std::prelude::*;
use std::process::exit;


pub fn get_args() -> String {
    if env::args().len() <= 2 {
        println!(" provide file name");
        exit(1);
    }
    
    for (index,item) in env::args().enumerate(){
        println!("{} => {}",index, item);
    }
    
    
    let arg2 = env::args().nth(2).unwrap();
    println!("Arg2 = {}", arg2);
    let arg2 = env::args().nth(1).unwrap();
    arg2
}