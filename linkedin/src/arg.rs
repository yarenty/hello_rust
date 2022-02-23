use std::env;
use std::env::args;

pub fn get_args() -> String {
    for (index,item) in env::args().enumerate(){
        println!("{} => {}",index, item);
    }
    
    
    let arg2 = env::args().nth(2).unwrap();
    println!("Arg2 = {}", arg2);
    let arg2 = env::args().nth(1).unwrap();
    arg2
}