mod arrs;
mod str;
mod arg;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
