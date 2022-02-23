use std::fs;
use std::io::prelude::*;


pub fn read() {
    
    let contents = fs::read_to_string("README.md").unwrap();
    
    println!("content:: {}",contents);
}


pub fn write(filename: &str, text: &str) {
    fs::write(filename,text);
}


pub fn find(filename: &str, text: &str) -> bool {
    let contents = fs::read_to_string(filename).unwrap();

    contents.contains(text)
}
