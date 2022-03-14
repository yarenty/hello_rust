use std::collections::HashMap;
use std::ptr::hash;

fn vecs() {
    let mut astro : Vec<String>  = Vec::new();
    astro.push(String::from("John"));
    astro.push(String::from("Armstrong"));
    astro.push(String::from("Nqil"));
    
    println!("astronauts: {:?}", astro);
    
    let sec = &astro[1];
    println!("Second is {}", sec);
    
    
    let countdown = vec![5,4,3,2,1]; // check std::vec::Vec
}



fn hashes() {
    let mut astro : HashMap<String, u8>  = HashMap::new();
    astro.insert(String::from("John"),1);
    astro.insert(String::from("Armstrong"),2);
    astro.insert(String::from("Nqil"),3);
    astro.insert(String::from("Nqil"),4); //overwrite
    astro.entry(String::from("Nqil")).or_insert(6); //insert if not exist
    astro.entry(String::from("Aon")).or_insert(6); //insert if not exist
    
    println!("astronauts: {:?}", astro);
    
    let aon = astro.entry(String::from("Aon")).or_insert(4); //update
    *aon += 2;
    println!("astronauts: {:?}", astro);

    
    
    let sec = &astro.get("John");
    println!("Second is {:?}", sec);
    
} 

pub fn checkme(){
    vecs();
    
    hashes();

}