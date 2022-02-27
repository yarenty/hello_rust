mod arrs;
mod str;
mod arg;
mod files;
mod structures;
mod mulitthreading;

pub fn min_max() {
    arrs::min_max();
}

pub fn str() {
    str::string_manipulations();
}

pub fn str_trim(){
    let s = String::from("Ala ma kota");
    let ts = str::trim_spaces(&s);
    println!("{} => trimmed => {}", s, ts);

    let s = String::from("    Ala ma kota   ");
    let ts = str::trim_spaces(&s);
    println!("{} => trimmed => {}", s, ts);
    
    
}


pub fn args(){
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
    let b = files::find(&f,&s);
    println!("{} found in {} :: {}",s,f,b);
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
