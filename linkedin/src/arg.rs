use std::env;
use std::env::args;

pub fn get_args(){
    for (index,item) in env::args().enumerate(){
        println!("{} => {}",index, item);
    }
}