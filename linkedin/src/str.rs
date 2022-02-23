pub fn string_manipulations() -> bool {
    let mut message = String::from("Earth");
    println!("message is {}", message);
    message.push_str(" is flat!");
    println!("message  is now: {}", message);
    message.insert(5,'=');
    println!("and: {}",message);
    message.contains("flat")
}


pub fn trim_spaces(s: &str) -> &str {
    let b = s.as_bytes();
    let mut start = 0;
    for( index, item ) in s.chars().enumerate() {
        if item != ' '
        {
            start = index;
            break;
        }
    }
    let mut end = 0;
    for( index, item ) in s.chars().rev().enumerate() {
        if item != ' '
        {
            end = s.len() - index;
            break;
        }
    }
    
    &s[start..end]
}