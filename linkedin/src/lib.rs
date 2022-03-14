mod arg;
mod arrs;
mod boxing;
mod files;
mod generics;
mod mulitthreading;
mod str;
mod structures;
mod traits;
mod lifetimes;
mod enums;
mod errors;
mod collections;

pub fn min_max() {
    arrs::min_max();
}

pub fn str() {
    str::string_manipulations();
}

pub fn str_trim() {
    let s = String::from("Ala ma kota");
    let ts = str::trim_spaces(&s);
    println!("{} => trimmed => {}", s, ts);

    let s = String::from("    Ala ma kota   ");
    let ts = str::trim_spaces(&s);
    println!("{} => trimmed => {}", s, ts);
}

pub fn args() {
    arg::get_args();
}

pub fn read_file() {
    files::read();
}

pub fn write_file() {
    let f = arg::get_args();
    let s = String::from("Testing...\nJaro\nTom\nCat");
    files::write(&f, &s);
}

pub fn finder() {
    let f = arg::get_args();
    let s = String::from("Jaro");
    let b = files::find(&f, &s);
    println!("{} found in {} :: {}", s, f, b);
}

pub fn structs() {
    structures::vehicles();
}

pub fn threads() {
    mulitthreading::test_threads();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn maxes() {
    println!("Testing generics: get max");
    let out = generics::get_max(1, 2);
    println!("Generic max(1,2) = {}", out);
}

pub fn boxing() {
    boxing::boxing();
}

pub fn describe() {
    traits::describe();
}

pub fn lifetime() {
    lifetimes::lifetime();
}

pub fn test_enums(){
    enums::testme();
}

pub fn test_errors() {
    errors::check_files();
}

pub fn test_collections() {
    collections::checkme();
}