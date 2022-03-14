

fn simple(){
    let x;
    {
        let y = String::from("SSSS=454");
        x = &y;
        println!("x = {}", x);
    }    
}


fn best<'a>(p1: &'a str, p2:&'a str) -> &'a str {
    if p1.len() > p2.len() {
        p1
    } else {
        p2
    }
}

pub fn lifetime() {
    simple();
    
    let x;
    let p1 = String::from("RP1");
    let p2 = String::from("UN95");
    
    x = best(&p1,&p2);
    
    println!("best is {}", x);
}