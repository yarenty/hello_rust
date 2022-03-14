use std::fs;
use std::io;


pub fn check_files(){
    let result = fs::read_to_string("data/unknown.txt");
    
    let content = match result {
        Ok(s) => s,
        Err(e) =>  match e.kind() {
            io::ErrorKind::NotFound => String::from("File not exist"),
            io::ErrorKind::PermissionDenied => String::from("No permissions"),
            _ => panic!("Error type: {:?}", e) 
        }
    };
    println!("Content: {}", content);

}
